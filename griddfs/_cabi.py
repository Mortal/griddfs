import numpy as np
import ctypes
from ._native import lib, ffi
from ._bridge import rustcall, special_errors


lib.griddfs_init()


special_errors.update({
})


def to_cmatrix(ndarray):
    assert ndarray.dtype == np.uint8
    assert ndarray.flags.c_contiguous
    res = ffi.new('struct griddfs_matrix *')
    res.data = ffi.cast('unsigned char *', ndarray.ctypes.data)
    res.rows, res.cols = ndarray.shape
    return res


def to_crect(rect):
    res = ffi.new('struct griddfs_rect *')
    if len(rect) == 2:
        res.top, res.left = rect
        res.width = res.height = 1
    else:
        res.top, res.left, res.width, res.height = rect
    return res


def mark_downstream(dirs, rect, marks=None, mark=1):
    dirs = np.asarray(dirs, dtype=np.uint8)
    if marks is None:
        marks = np.zeros_like(dirs)
    assert dirs.shape == marks.shape
    rustcall(lib.griddfs_mark_downstream, to_cmatrix(dirs), to_crect(rect),
             to_cmatrix(marks), mark)
    return marks


def mark_upstream(dirs, rect, marks=None, mark=1):
    dirs = np.asarray(dirs, dtype=np.uint8)
    if marks is None:
        marks = np.zeros_like(dirs)
    assert dirs.shape == marks.shape
    rustcall(lib.griddfs_mark_upstream, to_cmatrix(dirs), to_crect(rect),
             to_cmatrix(marks), mark)
    return marks
