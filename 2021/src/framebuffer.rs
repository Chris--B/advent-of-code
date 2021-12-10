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
        let (width, height) = (width as isize, height as isize);
        let buf = smallvec![T::default(); buf_len];

        Framebuffer {
            buf,
            width,
            height,
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
}

/// Construction Methods
impl<T> Framebuffer<T> {
    pub fn from_func(width: usize, height: usize, func: impl Fn(usize, usize) -> T) -> Self {
        let buf_len = width * height;
        let (width, height) = (width as isize, height as isize);
        let mut buf = SmallVec::with_capacity(buf_len);

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
            border_color: None,
        }
    }

    // type Buf = SmallVec<[T; LOCAL_SIZE]>;

    pub fn into_inner(self) -> SmallVec<[T; LOCAL_SIZE]> {
        self.buf
    }
}

/// Access Methods
impl<T> Framebuffer<T> {
    #[inline(always)]
    pub fn width(&self) -> usize {
        self.width as usize
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.height as usize
    }

    #[inline(always)]
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

    #[inline(always)]
    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        let idx = self.idx_from_xy(x, y)?;
        self.buf.get(idx)
    }

    #[inline(always)]
    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        let idx = self.idx_from_xy(x, y)?;
        self.buf.get_mut(idx)
    }

    /// Call a kernel per pixel, sampling the neighboring 3x3 pixels
    ///
    /// If border color is unset, this panics
    pub fn kernel_3x3(&self, mut kernel: impl FnMut(usize, usize, &[[&T; 3]; 3])) {
        for x in 0..self.width {
            for y in 0..self.height {
                #[rustfmt::skip]
                let taps: [[&T;3]; 3] = [
                    [ &self[(x-1, y-1)], &self[(x, y-1)], &self[(x+1, y-1)]],
                    [ &self[(x-1, y  )], &self[(x, y  )], &self[(x+1, y  )]],
                    [ &self[(x-1, y+1)], &self[(x, y+1)], &self[(x+1, y+1)]],
                ];
                kernel(x as usize, y as usize, &taps);
            }
        }
    }
}

// Getting weird mut errors, this might not be useful. 🤷‍♀️
// impl<T> Framebuffer<T> {
//     /// Do something for each pixel
//     pub fn for_each<'a>(&'a mut self, mut func: impl FnMut(usize, usize, &'a T)) {
//         for x in 0..self.width() {
//             for y in 0..self.height() {
//                 let t: &T = &self[(x, y)];
//                 func(x, y, t);
//             }
//         }
//     }
// }

impl<T> Framebuffer<T>
where
    T: Eq + Hash,
{
    /// Count the occurrences of each distinct pixel
    pub fn counts(&self) -> HashMap<&T, usize> {
        let mut counts = HashMap::new();

        for x in 0..self.width() {
            for y in 0..self.height() {
                let t: &T = &self[(x, y)];
                *counts.entry(t).or_insert(0) += 1;
            }
        }

        counts
    }
}

/// Interop with `image1` crate
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

    pub fn save_to<P, F>(&self, path: &str, scale: u32, f: F) -> Result<(), image::ImageError>
    where
        P: image::Pixel + 'static,
        [<P as image::Pixel>::Subpixel]: image::EncodableLayout,
        F: Fn(&T) -> P,
    {
        let img = self.make_image(scale, f);
        img.save(path)
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

impl<T> Index<(isize, isize)> for Framebuffer<T> {
    type Output = T;

    fn index(&self, idx: (isize, isize)) -> &Self::Output {
        self.get(idx.0, idx.1)
            .or_else(|| self.border_color.as_ref())
            .expect("oob index but no border color set")
    }
}

impl<T> IndexMut<(isize, isize)> for Framebuffer<T> {
    fn index_mut(&mut self, idx: (isize, isize)) -> &mut Self::Output {
        self.get_mut(idx.0, idx.1)
            .expect("oob index but no border color set")
    }
}
