use bridge::*;
use err::Result;
use types::{CMatrix, CRect};
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

export_using_from_c!(griddfs_mark_downstream(dirs: *mut CMatrix, rect: *const CRect, marks: *mut CMatrix, mark: c_uchar) -> Result<()> = mark_downstream);
export_using_from_c!(griddfs_mark_upstream(dirs: *mut CMatrix, rect: *const CRect, marks: *mut CMatrix, mark: c_uchar) -> Result<()> = mark_upstream);
