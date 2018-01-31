import numpy as np
from . import dfs


dirs = np.array([[  2,   4,   8],
                 [  0,   4,  16],
                 [  0,   0,  16],
                 [  0,  64,  16],
                 [128,  64,  32]], dtype=np.uint8)


marks = dfs(dirs, (0, 0))
assert np.transpose(marks.nonzero()).tolist() == (
    [[0, 0], [1, 1], [2, 1]]), (np.transpose(marks.nonzero()).tolist(), marks)
