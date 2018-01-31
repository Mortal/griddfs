use bridge::*;
use err::Result;
use types::{CMatrix, CRect, Mat, Rectangle};
use std::ffi::CString;
use griddfs::dfs;
use std::os::raw::{c_char, c_uchar};

#[no_mangle]
pub unsafe extern "C" fn griddfs_init() {
    set_panic_hook();
}

#[no_mangle]
pub unsafe extern "C" fn griddfs_free(buf: *mut c_char) {
    CString::from_raw(buf);
}

export!(griddfs_dfs(dirs: *mut CMatrix, sources: *const CRect, marks: *mut CMatrix, mark: c_uchar) -> Result<()> {
    let dirs = Mat::from_ptr_mut(dirs);
    let sources = Rectangle::from_ptr(sources);
    let marks = Mat::from_ptr_mut(marks);
    let mark: u8 = mark;
    try!(dfs(dirs, sources, marks, mark));
    Ok(())
});

