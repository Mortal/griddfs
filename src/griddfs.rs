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

pub fn mark_downstream<'a>(dirs: Mat<'a, u8>, sources: Rectangle, mut marks: Mat<'a, u8>, mark: u8) -> Result<()> {
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

pub fn mark_upstream<'a>(dirs: Mat<'a, u8>, destinations: Rectangle, mut marks: Mat<'a, u8>, mark: u8) -> Result<()> {
    for pos in destinations.iter() {
        marks[pos] |= mark;
    }
    for source in dirs.rect() {
        let mut pos_opt = Some(source);
        while let Some(pos) = pos_opt {
            if marks[pos] & mark != 0 {
                break;
            }
            pos_opt = pos.neighbor(dirs[pos]);
        }
        if let Some(dest) = pos_opt {
            let mut pos = source;
            while pos != dest {
                marks[pos] |= mark;
                pos = pos.neighbor(dirs[pos]).unwrap();
            }
        }
    }
    Ok(())
}
