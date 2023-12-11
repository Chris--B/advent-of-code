#![allow(unused)]

use image::ImageBuffer;
use smallvec::{smallvec, SmallVec};
use ultraviolet::IVec2;

use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Range;
use std::ops::{Index, IndexMut};

/*
 --> src/framebuffer.rs
  |
  | pub struct Framebuffer<T, const LOCAL_SIZE: usize = 1024> {
  |                                                   ^^^^^^
  |
  = note: see issue #44580 <https://github.com/rust-lang/rust/issues/44580> for more information
  = help: add `#![feature(const_generics_defaults)]` to the crate attributes to enable
*/
const LOCAL_SIZE: usize = 1024;

#[derive(Clone)]
pub struct Framebuffer<T> {
    /// Backing storage of pixel data
    buf: SmallVec<[T; LOCAL_SIZE]>,

    width: isize,
    height: isize,

    offsets: IVec2,

    /// When accessing out of bounds elements, return this
    ///
    /// All OOB accesses panic if this is `None`
    border_color: Option<T>,
}

/// Construction Methods
impl<T> Framebuffer<T>
where
    T: Default + Clone,
{
    /// Construct a framebuffer with specified dimensions. Elements are default constructed.
    pub fn new(width: u32, height: u32) -> Self {
        let w = width as i32;
        let h = height as i32;

        Self::new_with_ranges(0..w, 0..h)
    }

    pub fn new_with_ranges_square(xs: Range<i32>) -> Self {
        let ys = xs.clone();
        Self::new_with_ranges(xs, ys)
    }

    pub fn new_with_ranges(xs: Range<i32>, ys: Range<i32>) -> Self {
        let mut offsets = IVec2::zero();

        let width = (xs.end - xs.start) as isize;
        offsets.x = 0 - xs.start; // We'll add this to all x lookups

        let height = (ys.end - ys.start) as isize;
        offsets.y = 0 - ys.start; // We'll add this to all y lookups

        let buf_len = (width * height) as usize;
        let buf = smallvec![T::default(); buf_len];

        Framebuffer {
            buf,
            width,
            height,
            offsets,
            border_color: None,
        }
    }

    pub fn new_matching_size<U>(other: &Framebuffer<U>) -> Self {
        let width = other.width;
        let height = other.height;
        let offsets = other.offsets;

        let buf_len = (width * height) as usize;
        let buf = smallvec![T::default(); buf_len];

        Self {
            buf,
            width,
            height,
            offsets,
            border_color: None,
        }
    }
}

/// Construction Methods
impl<T> Framebuffer<T> {
    pub fn new_with_ranges_and(
        xs: Range<i32>,
        ys: Range<i32>,
        mut func: impl FnMut(i32, i32) -> T,
    ) -> Self {
        let mut offsets = IVec2::zero();

        let width = (xs.end - xs.start) as isize;
        offsets.x = -xs.start; // We'll add this to all x lookups

        let height = (ys.end - ys.start) as isize;
        offsets.y = -ys.start; // We'll add this to all y lookups

        let buf_len = (width * height) as usize;
        let mut buf = SmallVec::with_capacity(buf_len);

        // Generate elements in row-major order
        // We choose this order so we can .push() each new element. This will change if we modify
        // the backing memory tiling.
        for y in ys {
            for x in xs.clone() {
                buf.push(func(x, y));
            }
        }

        Framebuffer {
            buf,
            width,
            height,
            offsets,
            border_color: None,
        }
    }
}

impl<T> Framebuffer<T> {
    /// Sets a color to be returned when accesses are out of bounds, returning the old color.
    pub fn set_border_color(&mut self, mut border_color: Option<T>) -> Option<T> {
        std::mem::swap(&mut self.border_color, &mut border_color);
        border_color
    }

    pub fn width(&self) -> usize {
        self.width as usize
    }

    pub fn height(&self) -> usize {
        self.height as usize
    }

    /// Call a kernel per pixel
    pub fn kernel_1x1(&mut self, mut kernel: impl FnMut(usize, usize, &mut T)) {
        for x in 0..self.width {
            for y in 0..self.height {
                kernel(x as usize, y as usize, &mut self[(x, y)]);
            }
        }
    }

    pub fn flatten(&self) -> std::slice::Iter<T> {
        self.buf.iter()
    }

    pub fn flatten_mut(&mut self) -> std::slice::IterMut<T> {
        self.buf.iter_mut()
    }

    pub fn range_x(&self) -> Range<i32> {
        let x_start = 0 - self.offsets.x;
        let x_end = (self.width as i32) - self.offsets.x;

        x_start..x_end
    }

    pub fn range_y(&self) -> Range<i32> {
        let y_start = 0 - self.offsets.y;
        let y_end = self.height as i32 - self.offsets.y;

        y_start..y_end
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = (i32, i32)> {
        let xs = self.range_x();
        let ys = self.range_y();

        ys.flat_map(move |y| xs.clone().map(move |x| (x, y)))
    }
}

impl<T: Default + PartialEq> Framebuffer<T> {
    // Return an axis aligned bounding region of all content that's not default constructed
    pub fn content_bounds(&self) -> Option<[IVec2; 2]> {
        let ignore = T::default();

        let mut bounds = None;

        for (x, y) in self.iter_coords() {
            if self[(x, y)] != ignore {
                let xy = IVec2::new(x, y);

                if bounds.is_none() {
                    bounds = Some([xy, xy]);
                }

                if let Some([min, max]) = &mut bounds {
                    *min = min.min_by_component(xy);
                    *max = max.max_by_component(xy);
                }
            }
        }

        bounds
    }
}

impl<T> Framebuffer<T>
where
    T: Clone,
{
    /// Call a kernel per pixel, sampling the neighboring 3x3 pixels
    ///
    /// If border color is unset, this panics
    pub fn kernel_3x3(&mut self, mut kernel: impl FnMut(usize, usize, &[[&T; 3]; 3]) -> T) {
        // Duplicate out buffer to run the kernel "in place"
        // "Taps" will reference back to this snapshot, while we update the main buffer after each
        // kernel call.
        let prev = self.buf.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                #[rustfmt::skip]
                let taps: [[&T;3]; 3] = [
                    [ self.get_buf(&prev, x-1, y-1), self.get_buf(&prev, x, y-1), self.get_buf(&prev, x+1, y-1)],
                    [ self.get_buf(&prev, x-1, y  ), self.get_buf(&prev, x, y  ), self.get_buf(&prev, x+1, y  )],
                    [ self.get_buf(&prev, x-1, y+1), self.get_buf(&prev, x, y+1), self.get_buf(&prev, x+1, y+1)],
                ];
                self[(x, y)] = kernel(x as usize, y as usize, &taps);
            }
        }
    }

    pub fn clear(&mut self, clear_color: T) {
        for t in self.flatten_mut() {
            *t = clear_color.clone();
        }
    }
}

impl<T> Framebuffer<T>
where
    T: Eq + Hash,
{
    /// Count the occurrences of each distinct pixel
    pub fn counts(&self) -> HashMap<&T, usize> {
        let mut counts = HashMap::new();

        for t in self.flatten() {
            *counts.entry(t).or_insert(0) += 1;
        }

        counts
    }
}

/// Ascii Art - assume display is 1 character per
impl<T> Framebuffer<T> {
    pub fn print<U>(&self, func: impl Fn(i32, i32, &T) -> U)
    where
        U: std::fmt::Display,
    {
        self.print_range_with(self.range_x(), self.range_y(), func)
    }

    pub fn print_range_with<U>(
        &self,
        xs: Range<i32>,
        ys: Range<i32>,
        func: impl Fn(i32, i32, &T) -> U,
    ) where
        U: std::fmt::Display,
    {
        for y in ys.rev() {
            for x in xs.clone() {
                print!("{}", func(x, y, &self[(x, y)]));
            }
            println!();
        }
        println!();
    }
}

/// Interop with `image` crate
impl<T> Framebuffer<T> {
    pub fn make_image<P, F>(&self, scale: u32, f: F) -> ImageBuffer<P, Vec<P::Subpixel>>
    where
        P: image::Pixel + 'static,
        [<P as image::Pixel>::Subpixel]: image::EncodableLayout,
        F: Fn(&T) -> P,
    {
        let width = self.width;
        let height = self.height;
        let img = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
            let x = x as i32;
            let y = y as i32;
            f(&self[(x - self.offsets.x, y - self.offsets.y)])
        });

        image::imageops::resize(
            &img,
            width as u32 * scale,
            height as u32 * scale,
            image::imageops::FilterType::Nearest,
        )
    }
}

// ==== Index using anything that can be converted to `usize` ==================
macro_rules! impl_indexing {
    ($($t:ty),* $(,)?) => {
        $(
            impl<T> Index<($t, $t)> for Framebuffer<T> {
                type Output = T;

                fn index(&self, idx: ($t, $t)) -> &Self::Output {
                    &self[(idx.0 as isize, idx.1 as isize)]
                }
            }

            impl<T> IndexMut<($t, $t)> for Framebuffer<T> {
                fn index_mut(&mut self, idx: ($t, $t)) -> &mut Self::Output {
                    &mut self[(idx.0 as isize, idx.1 as isize)]
                }
            }
        )+
    }
}

// Route all integer types to isize
impl_indexing![
    // Unsigned
    u8, u16, u32, u64, usize, // Signed
    i8, i16, i32, i64,
];

impl<T> Index<IVec2> for Framebuffer<T> {
    type Output = T;

    fn index(&self, idx: IVec2) -> &Self::Output {
        &self[(idx.x as isize, idx.y as isize)]
    }
}

impl<T> IndexMut<IVec2> for Framebuffer<T> {
    fn index_mut(&mut self, idx: IVec2) -> &mut Self::Output {
        &mut self[(idx.x as isize, idx.y as isize)]
    }
}

impl<T> Index<&IVec2> for Framebuffer<T> {
    type Output = T;

    fn index(&self, idx: &IVec2) -> &Self::Output {
        &self[(idx.x as isize, idx.y as isize)]
    }
}

impl<T> IndexMut<&IVec2> for Framebuffer<T> {
    fn index_mut(&mut self, idx: &IVec2) -> &mut Self::Output {
        &mut self[(idx.x as isize, idx.y as isize)]
    }
}

// The real index logic, using isize

impl<T> Framebuffer<T> {
    fn idx_from_xy(&self, mut x: isize, mut y: isize) -> Option<usize> {
        // Offset back to unsigned coordinates
        x += self.offsets.x as isize;
        y += self.offsets.y as isize;

        if x < 0 || y < 0 {
            return None;
        }

        if x >= self.width || y >= self.height {
            return None;
        }

        let idx = x + y * self.width;
        Some(idx as usize)
    }

    fn get_buf<'a>(&'a self, buf: &'a [T], x: isize, y: isize) -> &'a T {
        self.idx_from_xy(x, y)
            .map(|idx| &buf[idx])
            .or(self.border_color.as_ref())
            .unwrap_or_else(|| panic!("oob index ({x}, {y}) but no border color set"))
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        let idx = self.idx_from_xy(x, y)?;
        self.buf.get(idx)
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        let idx = self.idx_from_xy(x, y)?;
        self.buf.get_mut(idx)
    }

    pub fn get_v(&self, xy: IVec2) -> Option<&T> {
        let (x, y) = (xy.x as isize, xy.y as isize);
        let idx = self.idx_from_xy(x, y)?;
        self.buf.get(idx)
    }

    pub fn get_mut_v(&mut self, xy: IVec2) -> Option<&mut T> {
        let (x, y) = (xy.x as isize, xy.y as isize);
        let idx = self.idx_from_xy(x, y)?;
        self.buf.get_mut(idx)
    }
}

impl<T> Index<(isize, isize)> for Framebuffer<T> {
    type Output = T;

    #[track_caller]
    fn index(&self, idx: (isize, isize)) -> &Self::Output {
        let w = self.width();
        let h = self.height();
        if let Some(t) = self.get(idx.0, idx.1).or(self.border_color.as_ref()) {
            t
        } else {
            panic!(
                "oob index ({x}, {y}) dims=({w}, {h}), but no border color set",
                x = idx.0,
                y = idx.1
            )
        }
    }
}

impl<T> IndexMut<(isize, isize)> for Framebuffer<T> {
    #[track_caller]
    fn index_mut(&mut self, idx: (isize, isize)) -> &mut Self::Output {
        let w = self.width();
        let h = self.height();
        if let Some(t) = self.get_mut(idx.0, idx.1) {
            t
        } else {
            panic!(
                "oob index ({x}, {y}) dims=({w}, {h}), but no border color set",
                x = idx.0,
                y = idx.1
            )
        }
    }
}
