use std::ops::{Index, IndexMut};
use std::iter::{Iterator, IntoIterator};
use std::os::raw::{c_uchar, c_uint};
use std::slice;

#[repr(C)]
pub struct CMatrix {
    data: *mut c_uchar,
    rows: c_uint,
    cols: c_uint,
}

#[repr(C)]
pub struct CRect {
    top: c_uint,
    left: c_uint,
    width: c_uint,
    height: c_uint,
}

pub struct Mat<'a, T: 'a> {
    data: &'a mut [T],
    rows: usize,
    cols: usize,
}

pub struct Rectangle {
    top: usize,
    left: usize,
    width: usize,
    height: usize,
}

impl<'a, T> Index<(usize, usize)> for Mat<'a, T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &T {
        let (i, j) = index;
        if i >= self.rows || j >= self.cols {
            panic!("{},{} out of bounds {},{}", i, j, self.rows, self.cols);
        }
        let offs = i * self.cols + j;
        &self.data[offs]
    }
}

impl<'a, T> IndexMut<(usize, usize)> for Mat<'a, T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        let (i, j) = index;
        if i >= self.rows || j >= self.cols {
            panic!("{},{} out of bounds {},{}", i, j, self.rows, self.cols);
        }
        let offs = i * self.cols + j;
        &mut self.data[offs]
    }
}

impl<'a> Mat<'a, u8> {
    pub unsafe fn from_ptr_mut(p: *mut CMatrix) -> Self {
        let rows = (*p).rows as usize;
        let cols = (*p).cols as usize;
        Mat {
            data: slice::from_raw_parts_mut((*p).data, rows*cols),
            rows: rows,
            cols: cols,
        }
    }
}

pub struct RectangleIterator {
    i: usize,
    j: usize,
    left: usize,
    right: usize,
    bottom: usize,
}

impl Iterator for RectangleIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        if self.i >= self.bottom {
            return None;
        }
        let r = (self.i, self.j);
        if self.j + 1 < self.right {
            self.j += 1;
        } else {
            self.i += 1;
            self.j = self.left;
        }
        Some(r)
    }
}

impl Rectangle {
    pub unsafe fn from_ptr(p: *const CRect) -> Self {
        Rectangle {
            top: (*p).top as usize,
            left: (*p).left as usize,
            width: (*p).width as usize,
            height: (*p).height as usize,
        }
    }

    pub fn iter(&self) -> RectangleIterator {
        RectangleIterator {
            i: self.top,
            j: self.left,
            left: self.left,
            right: self.left + self.width,
            bottom: self.top + self.height,
        }
    }

    pub fn contains(&self, index: (usize, usize)) -> bool {
        let (i, j) = index;
        self.top <= i && i < self.top + self.height &&
            self.left <= j && j < self.left + self.width
    }
}

impl IntoIterator for Rectangle {
    type Item = (usize, usize);
    type IntoIter = RectangleIterator;
    fn into_iter(self) -> RectangleIterator {
        self.iter()
    }
}

impl<'a, T> Mat<'a, T> {
    pub fn rect(&self) -> Rectangle {
        Rectangle {
            top: 0,
            left: 0,
            width: self.cols,
            height: self.rows,
        }
    }
}
