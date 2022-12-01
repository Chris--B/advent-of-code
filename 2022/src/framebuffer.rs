use smallvec::{smallvec, SmallVec};

use image::ImageBuffer;

use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
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
    pub fn with_dims(width: usize, height: usize) -> Self {
        let buf_len = width * height;

        let buf = smallvec![T::default(); buf_len];
        let (width, height) = (width as isize, height as isize);
        let border_color = None;

        Framebuffer {
            buf,
            width,
            height,
            border_color,
        }
    }

    pub fn with_dims_of<U>(other: &Framebuffer<U>) -> Self {
        Self::with_dims(other.width(), other.height())
    }
}

/// Construction Methods
impl<T> Framebuffer<T> {
    pub fn from_func(width: usize, height: usize, func: impl Fn(usize, usize) -> T) -> Self {
        let buf_len = width * height;

        let mut buf = SmallVec::with_capacity(buf_len);
        let (width, height) = (width as isize, height as isize);
        let border_color = None;

        // Generate elements in row-major order
        // We choose this order so we can .push() each new element. This will change if we modify
        // the backing memory tiling.
        for y in 0..(height as usize) {
            for x in 0..(width as usize) {
                buf.push(func(x, y));
            }
        }

        Framebuffer {
            buf,
            width,
            height,
            border_color,
        }
    }

    pub fn into_inner(self) -> SmallVec<[T; LOCAL_SIZE]> {
        self.buf
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

/// Interop with `image` crate
impl<T> Framebuffer<T> {
    pub fn make_image<P, F>(&self, scale: u32, f: F) -> ImageBuffer<P, Vec<P::Subpixel>>
    where
        P: image::Pixel + 'static,
        [<P as image::Pixel>::Subpixel]: image::EncodableLayout,
        F: Fn(&T) -> P,
    {
        let width = self.width as u32;
        let height = self.height as u32;
        let img = ImageBuffer::from_fn(width, height, |x, y| f(&self[(x, y)]));

        image::imageops::resize(
            &img,
            width * scale,
            height * scale,
            image::imageops::FilterType::Nearest,
        )
    }
}

// ==== Index using anything that can be converted to `usize` ==================
impl<T> Index<(u32, u32)> for Framebuffer<T> {
    type Output = T;

    fn index(&self, idx: (u32, u32)) -> &Self::Output {
        &self[(idx.0 as isize, idx.1 as isize)]
    }
}

impl<T> IndexMut<(u32, u32)> for Framebuffer<T> {
    fn index_mut(&mut self, idx: (u32, u32)) -> &mut Self::Output {
        &mut self[(idx.0 as isize, idx.1 as isize)]
    }
}

impl<T> Index<(u64, u64)> for Framebuffer<T> {
    type Output = T;

    fn index(&self, idx: (u64, u64)) -> &Self::Output {
        &self[(idx.0 as isize, idx.1 as isize)]
    }
}

impl<T> IndexMut<(u64, u64)> for Framebuffer<T> {
    fn index_mut(&mut self, idx: (u64, u64)) -> &mut Self::Output {
        &mut self[(idx.0 as isize, idx.1 as isize)]
    }
}

impl<T> Index<(usize, usize)> for Framebuffer<T> {
    type Output = T;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self[(idx.0 as isize, idx.1 as isize)]
    }
}

impl<T> IndexMut<(usize, usize)> for Framebuffer<T> {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self[(idx.0 as isize, idx.1 as isize)]
    }
}

// The real index logic

impl<T> Framebuffer<T> {
    fn idx_from_xy(&self, x: isize, y: isize) -> Option<usize> {
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
            .expect("oob index but no border color set")
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        let idx = self.idx_from_xy(x, y)?;
        self.buf.get(idx)
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        let idx = self.idx_from_xy(x, y)?;
        self.buf.get_mut(idx)
    }
}

impl<T> Index<(isize, isize)> for Framebuffer<T> {
    type Output = T;

    fn index(&self, idx: (isize, isize)) -> &Self::Output {
        self.get(idx.0, idx.1)
            .or(self.border_color.as_ref())
            .expect("oob index but no border color set")
    }
}

impl<T> IndexMut<(isize, isize)> for Framebuffer<T> {
    fn index_mut(&mut self, idx: (isize, isize)) -> &mut Self::Output {
        self.get_mut(idx.0, idx.1)
            .expect("oob index but no border color set")
    }
}
