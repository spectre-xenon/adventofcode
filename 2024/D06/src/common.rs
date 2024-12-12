#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Dir {
    pub fn next(&self) -> Self {
        match self {
            Dir::UP => Dir::RIGHT,
            Dir::RIGHT => Dir::DOWN,
            Dir::DOWN => Dir::LEFT,
            Dir::LEFT => Dir::UP,
        }
    }
}

pub fn next_block(current_pos: &(usize, usize), direction: &Dir) -> (usize, usize) {
    let (y, x) = *current_pos;
    match direction {
        Dir::UP => (y - 1, x),
        Dir::RIGHT => (y, x + 1),
        Dir::DOWN => (y + 1, x),
        Dir::LEFT => (y, x - 1),
    }
}
