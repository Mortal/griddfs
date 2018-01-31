use types::{Mat, Rectangle};
use err::Result;

trait Neighbor: Sized {
    fn neighbor(self, dir: u8) -> Option<Self>;
}

impl Neighbor for (usize, usize) {
    fn neighbor(self, dir: u8) -> Option<Self> {
        let (i, j) = self;
        match dir {
            0 | 255 => return None,
            1 | 2 | 4 | 8 | 16 | 32 | 64 | 128 => (),
            _ => panic!("Invalid direction {}", dir),
        };
        let i = match dir {
            2 | 4 | 8 => i + 1,
            32 | 64 | 128 => i - 1,
            _ => i,
        };
        let j = match dir {
            1 | 2 | 128 => j + 1,
            8 | 16 | 32 => j - 1,
            _ => j,
        };
        Some((i, j))
    }
}

pub fn dfs<'a>(dirs: Mat<'a>, sources: Rectangle, mut marks: Mat<'a>, mark: u8) -> Result<()> {
    for pos in sources {
        // println!("Visit src {:?} {}", pos, dirs[pos]);
        marks[pos] |= mark;
        let mut pos_opt = pos.neighbor(dirs[pos]);
        while let Some(pos) = pos_opt {
            // println!("Visit path {:?} {}", pos, dirs[pos]);
            if marks[pos] & mark != 0 {
                break;
            }
            marks[pos] |= mark;
            pos_opt = pos.neighbor(dirs[pos]);
        }
    }
    Ok(())
}
