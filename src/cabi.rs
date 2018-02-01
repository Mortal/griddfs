use bridge::*;
use err::Result;
use types::{CMatrix, CRect, Mat, Rectangle};
use std::ffi::CString;
use griddfs::*;
use std::os::raw::{c_char, c_uchar};

#[no_mangle]
pub unsafe extern "C" fn griddfs_init() {
    set_panic_hook();
}

#[no_mangle]
pub unsafe extern "C" fn griddfs_free(buf: *mut c_char) {
    CString::from_raw(buf);
}

export!(griddfs_mark_downstream(dirs: *mut CMatrix, rect: *const CRect, marks: *mut CMatrix, mark: c_uchar) -> Result<()> {
    let dirs = Mat::from_ptr_mut(dirs);
    let rect = Rectangle::from_ptr(rect);
    let marks = Mat::from_ptr_mut(marks);
    let mark: u8 = mark;
    try!(mark_downstream(dirs, rect, marks, mark));
    Ok(())
});

export!(griddfs_mark_upstream(dirs: *mut CMatrix, rect: *const CRect, marks: *mut CMatrix, mark: c_uchar) -> Result<()> {
    let dirs = Mat::from_ptr_mut(dirs);
    let rect = Rectangle::from_ptr(rect);
    let marks = Mat::from_ptr_mut(marks);
    let mark: u8 = mark;
    try!(mark_upstream(dirs, rect, marks, mark));
    Ok(())
});

