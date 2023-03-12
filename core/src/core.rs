use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tetromino {
    /// []
    /// []
    /// []
    /// []
    I,
    /// [][]
    /// [][]
    O,
    /// [][][]
    ///   []
    T,
    ///   []
    ///   []
    /// [][]
    J,
    /// []
    /// []
    /// [][]
    L,
    ///   [][]
    /// [][]
    S,
    /// [][]
    ///   [][]
    Z,
}

impl Distribution<Tetromino> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tetromino {
        match rng.gen_range(0..7) {
            0 => Tetromino::I,
            1 => Tetromino::O,
            2 => Tetromino::T,
            3 => Tetromino::J,
            4 => Tetromino::L,
            5 => Tetromino::S,
            _ => Tetromino::Z,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Empty,
    Block(Tetromino),
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Empty => write!(f, " ."),
            State::Block(_) => write!(f, "[]"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Size {
    pub x: usize,
    pub y: usize,
}

impl Size {
    fn new(x: usize, y: usize) -> Self {
        Size { x, y }
    }

    fn rotate(&self) -> Self {
        Self::new(self.y, self.x)
    }

    fn on_inside(&self, p: &Position) -> bool {
        p.x < self.x && p.y < self.y
    }

    fn len(&self) -> usize {
        self.x * self.y
    }
}

#[cfg(test)]
mod size_tests {
    use super::*;

    #[test]
    fn size_new() {
        let size = Size::new(10, 20);
        assert_eq!(size, Size { x: 10, y: 20 });
    }

    #[test]
    fn size_on_inside() {
        let size = Size::new(3, 2);
        assert_eq!(size.on_inside(&Position::new(2, 1)), true);
        assert_eq!(size.on_inside(&Position::new(3, 1)), false);
        assert_eq!(size.on_inside(&Position::new(2, 2)), false);
    }

    #[test]
    fn size_len() {
        assert_eq!(Size::new(3, 2).len(), 6);
    }
}

pub enum MoveDirection {
    Left,
    Right,
    Top,
    Bottom,
}

pub enum RotateDirection {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn move_x(&self, direction: &MoveDirection) -> usize {
        match direction {
            MoveDirection::Left => {
                if self.x > 0 {
                    self.x - 1
                } else {
                    self.x
                }
            }
            MoveDirection::Right => self.x + 1,
            _ => self.x,
        }
    }

    fn move_y(&self, direction: &MoveDirection) -> usize {
        match direction {
            MoveDirection::Top => {
                if self.y > 0 {
                    self.y - 1
                } else {
                    self.y
                }
            }
            MoveDirection::Bottom => self.y + 1,
            _ => self.y,
        }
    }

    pub fn r#move(&self, direction: &MoveDirection) -> Self {
        Self::new(self.move_x(direction), self.move_y(direction))
    }

    fn rotate(&self, size: &Size, direction: &RotateDirection) -> Position {
        match direction {
            RotateDirection::Left => Position::new(self.y, size.x - self.x - 1),
            RotateDirection::Right => Position::new(size.y - self.y - 1, self.x),
        }
    }

    pub fn index(&self, size: &Size) -> usize {
        size.x * self.y + self.x
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

#[cfg(test)]
mod position_tests {
    use super::*;

    #[test]
    fn position_new() {
        let p = Position::new(2, 3);
        assert_eq!(p, Position { x: 2, y: 3 });
    }

    #[test]
    fn position_move() {
        let p = Position::new(3, 3);
        assert_eq!(p.r#move(&MoveDirection::Left), Position::new(2, 3));
        assert_eq!(p.r#move(&MoveDirection::Right), Position::new(4, 3));
        assert_eq!(p.r#move(&MoveDirection::Bottom), Position::new(3, 4));
    }

    #[test]
    fn position_rotate() {}

    #[test]
    fn position_index() {
        let index = Position::new(2, 3).index(&Size::new(10, 10));
        assert_eq!(index, 32);
    }

    #[test]
    fn position_sub() {
        let p = Position::new(2, 3).sub(&Position::new(1, 2));
        assert_eq!(p, Position::new(1, 1));
    }
}

#[derive(Debug, PartialEq)]
struct Weight {
    size: Size,
    source: Vec<u8>,
}

impl Weight {
    fn new(size: Size, source: Vec<u8>) -> Self {
        Self { size, source }
    }

    fn on_inside(&self, p: &Position, other: &Self) -> bool {
        p.x + other.size.x <= self.size.x && p.y + other.size.y <= self.size.y
    }

    fn weight(&self, p: Position) -> u8 {
        self.source[p.index(&self.size)]
    }

    fn add_weight(&mut self, p: Position, w: u8) {
        self.source[p.index(&self.size)] += w;
    }

    fn overlap(&self, p: &Position, other: Self) -> Result<Self, ()> {
        if !self.on_inside(&p, &other) {
            return Err(());
        }

        let mut weight = Self::new(self.size, self.source.clone());
        for y in 0..other.size.y {
            for x in 0..other.size.x {
                let p = Position::new(p.x + x, p.y + y);
                weight.add_weight(p, other.weight(Position::new(x, y)));
            }
        }
        Ok(weight)
    }

    fn valid(&self) -> bool {
        for w in &self.source {
            if w > &1 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod weight_test {
    use super::*;
    #[test]
    fn weight_new() {
        assert_eq!(
            Weight::new(
                Size::new(2, 3),
                vec![
                    1, 0, //
                    1, 0, //
                    1, 1, //
                ]
            ),
            Weight {
                size: Size::new(2, 3),
                source: vec![
                    1, 0, //
                    1, 0, //
                    1, 1, //
                ],
            },
        );
    }

    #[test]
    fn weight_on_inside() {
        let a = Weight::new(Size::new(10, 20), vec![0; 10 * 20]);
        let b = Weight::new(Size::new(2, 3), vec![0; 10 * 20]);

        assert_eq!(a.on_inside(&Position::new(0, 0), &b), true);
        // . . . . . . . . [][]
        // 0.1.2.3.4.5.6.7.8.9.
        // [][][][][][][][][][]
        assert_eq!(a.on_inside(&Position::new(8, 17), &b), true);
        assert_eq!(a.on_inside(&Position::new(9, 17), &b), false);
        assert_eq!(a.on_inside(&Position::new(8, 18), &b), false);
    }

    #[test]
    fn weight_overlap() {
        let a = Weight::new(
            Size::new(10, 20),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // [][][][][][][][][][]
            ],
        );

        let b = Weight::new(
            Size::new(2, 3),
            vec![
                1, 0, // [].
                1, 1, // [][]
                0, 1, // . []
            ],
        );

        assert_eq!(
            a.overlap(&Position::new(0, 0), b),
            Ok(Weight::new(
                Size::new(10, 20),
                vec![
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, // []. . . . . . . . .
                    1, 1, 0, 0, 0, 0, 0, 0, 0, 0, // [][]. . . . . . . .
                    0, 1, 0, 0, 0, 0, 0, 0, 0, 0, // . []. . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // [][][][][][][][][][]
                ],
            ))
        );

        let b = Weight::new(
            Size::new(2, 3),
            vec![
                1, 0, // [].
                1, 1, // [][]
                0, 1, // . []
            ],
        );

        assert_eq!(
            a.overlap(&Position::new(8, 16), b),
            Ok(Weight::new(
                Size::new(10, 20),
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 1, 0, // . . . . . . . . [].
                    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, // . . . . . . . . [][]
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // . . . . . . . . . []
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // [][][][][][][][][][]
                ],
            ))
        );

        let b = Weight::new(
            Size::new(2, 3),
            vec![
                1, 0, // [].
                1, 1, // [][]
                0, 1, // . []
            ],
        );

        assert_eq!(a.overlap(&Position::new(9, 16), b), Err(()),);

        let b = Weight::new(
            Size::new(2, 3),
            vec![
                1, 0, // [].
                1, 1, // [][]
                0, 1, // . []
            ],
        );

        assert_eq!(
            a.overlap(&Position::new(8, 17), b),
            Ok(Weight::new(
                Size::new(10, 20),
                vec![
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // . . . . . . . . . .
                    0, 0, 0, 0, 0, 0, 0, 0, 1, 0, // . . . . . . . . [].
                    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, // . . . . . . . . [][]
                    1, 1, 1, 1, 1, 1, 1, 1, 1, 2, // [][][][][][][][][][]
                ],
            ))
        );
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub size: Size,
    source: Vec<State>,
}

impl Block {
    pub fn new() -> Self {
        match rand::random::<Tetromino>() {
            Tetromino::I => Block::new_i(),
            Tetromino::O => Block::new_o(),
            Tetromino::T => Block::new_t(),
            Tetromino::J => Block::new_j(),
            Tetromino::L => Block::new_l(),
            Tetromino::S => Block::new_s(),
            Tetromino::Z => Block::new_z(),
        }
    }

    pub fn new_i() -> Self {
        let size = Size::new(1, 4);
        let source = vec![
            State::Block(Tetromino::I),
            State::Block(Tetromino::I),
            State::Block(Tetromino::I),
            State::Block(Tetromino::I),
        ];
        Self { size, source }
    }

    pub fn new_o() -> Self {
        let size = Size::new(2, 2);
        let source = vec![
            State::Block(Tetromino::O),
            State::Block(Tetromino::O),
            State::Block(Tetromino::O),
            State::Block(Tetromino::O),
        ];
        Self { size, source }
    }

    pub fn new_t() -> Self {
        let size = Size::new(3, 2);
        let source = vec![
            State::Block(Tetromino::T),
            State::Block(Tetromino::T),
            State::Block(Tetromino::T),
            State::Empty,
            State::Block(Tetromino::T),
            State::Empty,
        ];
        Self { size, source }
    }

    pub fn new_j() -> Self {
        let size = Size::new(2, 3);
        let source = vec![
            State::Empty,
            State::Block(Tetromino::J),
            State::Empty,
            State::Block(Tetromino::J),
            State::Block(Tetromino::J),
            State::Block(Tetromino::J),
        ];
        Self { size, source }
    }

    pub fn new_l() -> Self {
        let size = Size::new(2, 3);
        let source = vec![
            State::Block(Tetromino::L),
            State::Empty,
            State::Block(Tetromino::L),
            State::Empty,
            State::Block(Tetromino::L),
            State::Block(Tetromino::L),
        ];
        Self { size, source }
    }

    pub fn new_s() -> Self {
        let size = Size::new(3, 2);
        let source = vec![
            State::Empty,
            State::Block(Tetromino::S),
            State::Block(Tetromino::S),
            State::Block(Tetromino::S),
            State::Block(Tetromino::S),
            State::Empty,
        ];
        Self { size, source }
    }

    pub fn new_z() -> Self {
        let size = Size::new(3, 2);
        let source = vec![
            State::Block(Tetromino::Z),
            State::Block(Tetromino::Z),
            State::Empty,
            State::Empty,
            State::Block(Tetromino::Z),
            State::Block(Tetromino::Z),
        ];
        Self { size, source }
    }

    pub fn rotate(self, direction: RotateDirection) -> Self {
        let size = self.size.rotate();
        let mut source = vec![State::Empty; size.len()];
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let p = Position::new(x, y);
                source[p.rotate(&self.size, &direction).index(&size)] = self.state(&p);
            }
        }
        Self { size, source }
    }

    fn weight(&self) -> Weight {
        let mut source = Vec::with_capacity(self.source.len());
        for i in 0..self.source.len() {
            match self.source[i] {
                State::Empty => source.push(0),
                State::Block(_) => source.push(1),
            }
        }

        Weight::new(self.size, source)
    }

    pub fn state(&self, p: &Position) -> State {
        if self.size.on_inside(p) {
            self.source[p.index(&self.size)]
        } else {
            State::Empty
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = String::new();
        for y in 0..self.size.y {
            if y > 0 {
                text += "\n";
            }
            for x in 0..self.size.x {
                text += &format!("{}", self.source[Position::new(x, y).index(&self.size)]);
            }
        }
        write!(f, "{text}")
    }
}

#[cfg(test)]
mod block_tests {
    use super::*;

    #[test]
    fn block_weight() {
        // [] . | 1.0.
        // [] . | 1.0.
        // [][] | 1.1.
        let block = Block::new_l();
        assert_eq!(
            block.weight(),
            Weight::new(
                Size::new(2, 3),
                vec![
                    1, 0, //
                    1, 0, //
                    1, 1, //
                ],
            )
        );
    }

    #[test]
    fn block_rotate() {
        // [] . | 0.1. | 0010
        // [] . | 2.3. | 0111
        // [][] | 4.5. | 0212
        let block = Block::new_l();
        assert_eq!(
            block.rotate(RotateDirection::Left),
            Block {
                size: Size::new(3, 2),
                source: vec![
                    //  . .[] | 1.3.5. | 001020
                    // [][][] | 0.2.4. | 011121
                    State::Empty,
                    State::Empty,
                    State::Block(Tetromino::L),
                    State::Block(Tetromino::L),
                    State::Block(Tetromino::L),
                    State::Block(Tetromino::L),
                ],
            }
        );

        let block = Block::new_l();
        assert_eq!(
            block.rotate(RotateDirection::Right),
            Block {
                size: Size::new(3, 2),
                source: vec![
                    // [][][] | 4.2.0. | 001020
                    // [] . . | 5.3.1. | 011121
                    State::Block(Tetromino::L),
                    State::Block(Tetromino::L),
                    State::Block(Tetromino::L),
                    State::Block(Tetromino::L),
                    State::Empty,
                    State::Empty,
                ],
            }
        );

        // [][] .
        //  .[][]
        let block = Block::new_z();
        assert_eq!(
            block.rotate(RotateDirection::Left),
            Block {
                size: Size::new(2, 3),
                source: vec![
                    //  .[]
                    // [][]
                    // [] .
                    State::Empty,
                    State::Block(Tetromino::Z),
                    State::Block(Tetromino::Z),
                    State::Block(Tetromino::Z),
                    State::Block(Tetromino::Z),
                    State::Empty,
                ],
            }
        );

        let block = Block::new_z();
        assert_eq!(
            block.rotate(RotateDirection::Right),
            Block {
                size: Size::new(2, 3),
                source: vec![
                    //  .[]
                    // [][]
                    // [] .
                    State::Empty,
                    State::Block(Tetromino::Z),
                    State::Block(Tetromino::Z),
                    State::Block(Tetromino::Z),
                    State::Block(Tetromino::Z),
                    State::Empty,
                ],
            }
        );

        // [][]
        // [][]
        let block = Block::new_o();
        assert_eq!(
            block.rotate(RotateDirection::Left),
            Block {
                size: Size::new(2, 2),
                source: vec![
                    // [][]
                    // [][]
                    State::Block(Tetromino::O),
                    State::Block(Tetromino::O),
                    State::Block(Tetromino::O),
                    State::Block(Tetromino::O),
                ],
            }
        );

        let block = Block::new_o();
        assert_eq!(
            block.rotate(RotateDirection::Right),
            Block {
                size: Size::new(2, 2),
                source: vec![
                    // [][]
                    // [][]
                    State::Block(Tetromino::O),
                    State::Block(Tetromino::O),
                    State::Block(Tetromino::O),
                    State::Block(Tetromino::O),
                ],
            }
        );

        // []
        // []
        // []
        // []
        let block = Block::new_i();
        assert_eq!(
            block.rotate(RotateDirection::Left),
            Block {
                size: Size::new(4, 1),
                source: vec![
                    // [][][][]
                    State::Block(Tetromino::I),
                    State::Block(Tetromino::I),
                    State::Block(Tetromino::I),
                    State::Block(Tetromino::I),
                ],
            }
        );

        let block = Block::new_i();
        assert_eq!(
            block.rotate(RotateDirection::Right),
            Block {
                size: Size::new(4, 1),
                source: vec![
                    // [][][][]
                    State::Block(Tetromino::I),
                    State::Block(Tetromino::I),
                    State::Block(Tetromino::I),
                    State::Block(Tetromino::I),
                ],
            }
        );
    }

    #[test]
    fn block_new_i() {
        let block = Block::new_i();
        assert_eq!(
            block,
            Block {
                size: Size::new(1, 4),
                source: vec![
                    State::Block(Tetromino::I),
                    State::Block(Tetromino::I),
                    State::Block(Tetromino::I),
                    State::Block(Tetromino::I)
                ]
            },
        );
        assert_eq!(block.size.x * block.size.y, block.source.len());
        assert_eq!(
            format!("\n{block}\n"),
            "
[]
[]
[]
[]
"
        );
    }

    #[test]
    fn block_new_o() {
        let block = Block::new_o();
        assert_eq!(
            block,
            Block {
                size: Size::new(2, 2),
                source: vec![
                    State::Block(Tetromino::O),
                    State::Block(Tetromino::O),
                    State::Block(Tetromino::O),
                    State::Block(Tetromino::O),
                ]
            },
        );
        assert_eq!(block.size.x * block.size.y, block.source.len());
        assert_eq!(
            format!("\n{block}\n"),
            "
[][]
[][]
"
        );
    }

    #[test]
    fn block_new_t() {
        let block = Block::new_t();
        assert_eq!(
            block,
            Block {
                size: Size::new(3, 2),
                source: vec![
                    State::Block(Tetromino::T),
                    State::Block(Tetromino::T),
                    State::Block(Tetromino::T),
                    State::Empty,
                    State::Block(Tetromino::T),
                    State::Empty,
                ]
            },
        );
        assert_eq!(block.size.x * block.size.y, block.source.len());
        assert_eq!(
            format!("\n{block}\n"),
            "
[][][]
 .[] .
"
        );
    }

    #[test]
    fn block_new_j() {
        let block = Block::new_j();
        assert_eq!(
            block,
            Block {
                size: Size::new(2, 3),
                source: vec![
                    State::Empty,
                    State::Block(Tetromino::J),
                    State::Empty,
                    State::Block(Tetromino::J),
                    State::Block(Tetromino::J),
                    State::Block(Tetromino::J),
                ]
            },
        );
        assert_eq!(block.size.x * block.size.y, block.source.len());
        assert_eq!(
            format!("\n{block}\n"),
            "
 .[]
 .[]
[][]
"
        );
    }

    #[test]
    fn block_new_l() {
        let block = Block::new_l();
        assert_eq!(
            block,
            Block {
                size: Size::new(2, 3),
                source: vec![
                    State::Block(Tetromino::L),
                    State::Empty,
                    State::Block(Tetromino::L),
                    State::Empty,
                    State::Block(Tetromino::L),
                    State::Block(Tetromino::L),
                ]
            },
        );
        assert_eq!(block.size.x * block.size.y, block.source.len());
        assert_eq!(
            format!("\n{block}\n"),
            "
[] .
[] .
[][]
"
        );
    }

    #[test]
    fn block_new_s() {
        let block = Block::new_s();
        assert_eq!(
            block,
            Block {
                size: Size::new(3, 2),
                source: vec![
                    State::Empty,
                    State::Block(Tetromino::S),
                    State::Block(Tetromino::S),
                    State::Block(Tetromino::S),
                    State::Block(Tetromino::S),
                    State::Empty,
                ]
            },
        );
        assert_eq!(block.size.x * block.size.y, block.source.len());
        assert_eq!(
            format!("\n{block}\n"),
            "
 .[][]
[][] .
"
        );
    }

    #[test]
    fn block_new_z() {
        let block = Block::new_z();
        assert_eq!(
            block,
            Block {
                size: Size::new(3, 2),
                source: vec![
                    State::Block(Tetromino::Z),
                    State::Block(Tetromino::Z),
                    State::Empty,
                    State::Empty,
                    State::Block(Tetromino::Z),
                    State::Block(Tetromino::Z),
                ]
            },
        );
        assert_eq!(block.size.x * block.size.y, block.source.len());
        assert_eq!(
            format!("\n{block}\n"),
            "
[][] .
 .[][]
"
        );
    }

    #[test]
    fn block_state() {
        let block = Block::new_z();
        assert_eq!(block.state(&Position::new(2, 0)), State::Empty);
        assert_eq!(
            block.state(&Position::new(2, 1)),
            State::Block(Tetromino::Z)
        );
        assert_eq!(block.state(&Position::new(3, 1)), State::Empty);
        assert_eq!(block.state(&Position::new(2, 2)), State::Empty);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    pub size: Size,
    source: Vec<State>,
}

impl Board {
    pub fn new() -> Self {
        let size = Size::new(10, 20);
        let source = vec![State::Empty; size.len()];
        Self { size, source }
    }

    pub fn state(&self, p: &Position) -> State {
        if self.size.on_inside(p) {
            self.source[p.index(&self.size)]
        } else {
            State::Empty
        }
    }

    fn weight(&self) -> Weight {
        let mut source = Vec::with_capacity(self.source.len());
        for i in 0..self.source.len() {
            source.push(match self.source[i] {
                State::Empty => 0,
                State::Block(_) => 1,
            });
        }
        Weight::new(self.size, source)
    }

    pub fn can_change(&self, p: &Position, block: &Block) -> bool {
        match self.weight().overlap(p, block.weight()) {
            Ok(w) => w.valid(),
            Err(()) => false,
        }
    }

    fn set_state(mut self, p: &Position, state: State) -> Self {
        if self.size.on_inside(p) {
            self.source[p.index(&self.size)] = state;
        }
        self
    }

    pub fn set_block(mut self, p: &Position, block: Block) -> Self {
        for y in 0..block.size.y {
            for x in 0..block.size.x {
                let p = Position::new(p.x + x, p.y + y);
                if let State::Block(t) = block.state(&Position::new(x, y)) {
                    self.source[p.index(&self.size)] = State::Block(t);
                }
            }
        }
        self
    }

    /// return self and valid-row-count
    pub fn remove_valid_rows(mut self) -> (Self, usize) {
        let mut remove_rows = Vec::with_capacity(self.size.y);
        for y in 0..self.size.y {
            let start = y * self.size.x;
            let end = start + self.size.x;
            let row = &self.source[start..end];

            let mut count_block = 0;
            for state in row {
                if let State::Block(..) = state {
                    count_block += 1;
                }
            }
            if count_block == self.size.x {
                remove_rows.push(y);
            }
        }

        for y in &remove_rows {
            for x in 0..self.size.x {
                let max_y = y;
                for y in 0..=*y {
                    let y = max_y - y;
                    match y {
                        0 => {
                            self = self.set_state(&Position::new(x, y), State::Empty);
                        }
                        _ => {
                            let p1 = Position::new(x, y);
                            let p2 = Position::new(x, y - 1);
                            let s1 = self.state(&p1);
                            let s2 = self.state(&p2);
                            self = self.set_state(&p1, s2);
                            self = self.set_state(&p2, s1);
                        }
                    }
                }
            }
        }
        (self, remove_rows.len())
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn board_state() {
        let board = Board::new();
        let board = board.set_state(&Position::new(9, 19), State::Block(Tetromino::Z));
        let board = board.set_state(&Position::new(9, 20), State::Block(Tetromino::Z));
        assert_eq!(board.state(&Position::new(9, 18)), State::Empty);
        assert_eq!(
            board.state(&Position::new(9, 19)),
            State::Block(Tetromino::Z)
        );
        assert_eq!(board.state(&Position::new(9, 20)), State::Empty,);
    }

    #[test]
    fn board_weight() {
        let mut board = Board::new();
        for x in 0..10 {
            board = board.set_state(&Position::new(x, 19), State::Block(Tetromino::O));
        }

        let mut source = Vec::with_capacity(board.source.len());
        for i in 0..board.source.len() {
            source.push(match board.source[i] {
                State::Empty => 0,
                State::Block(_) => 1,
            });
        }

        assert_eq!(board.weight(), Weight::new(board.size, source,));
    }

    #[test]
    fn board_can_change() {
        let mut board = Board::new();
        for x in 0..10 {
            board = board.set_state(&Position::new(x, 19), State::Block(Tetromino::O));
        }
        let board = board;

        // [].
        // [][]
        // . []
        let block = Block::new_s().rotate(RotateDirection::Left);

        assert_eq!(board.can_change(&Position::new(8, 16), &block), true);
        assert_eq!(board.can_change(&Position::new(9, 16), &block), false);
        assert_eq!(board.can_change(&Position::new(8, 17), &block), false);
    }

    #[test]
    fn board_remove_valid_rows() {
        let board = Board::new();
        let board = board.set_block(&Position::new(0, 18), Block::new_o());
        let board = board.set_block(&Position::new(2, 18), Block::new_o());
        let board = board.set_block(&Position::new(4, 18), Block::new_o());
        let board = board.set_block(&Position::new(6, 18), Block::new_o());
        let board = board.set_block(&Position::new(8, 18), Block::new_o());

        assert_eq!(board.remove_valid_rows(), (Board::new(), 2));
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = String::new();
        for y in 0..self.size.y {
            text += "<!";
            for x in 0..self.size.x {
                text += &format!("{}", self.state(&Position::new(x, y)));
            }
            text += "!>\n";
        }
        text += "<!";
        for _ in 0..self.size.x {
            text += "==";
        }
        text += "!>\n";
        text += "  ";
        for _ in 0..self.size.x {
            text += "\\/";
        }
        write!(f, "{text}")
    }
}

#[cfg(test)]
mod board_display_tests {
    use super::*;

    #[test]
    fn display_board() {
        let board = Board::new();
        let board = board.set_block(&Position::new(4, 0), Block::new_s());
        assert_eq!(
            format!("{board}"),
            "
<! . . . . .[][] . . .!>
<! . . . .[][] . . . .!>
<! . . . . . . . . . .!>
<! . . . . . . . . . .!>
<! . . . . . . . . . .!>
<! . . . . . . . . . .!>
<! . . . . . . . . . .!>
<! . . . . . . . . . .!>
<! . . . . . . . . . .!>
<! . . . . . . . . . .!>
<! . . . . . . . . . .!>
<! . . . . . . . . . .!>
<! . . . . . . . . . .!>
<! . . . . . . . . . .!>
<! . . . . . . . . . .!>
<! . . . . . . . . . .!>
<! . . . . . . . . . .!>
<! . . . . . . . . . .!>
<! . . . . . . . . . .!>
<! . . . . . . . . . .!>
<!====================!>
  \\/\\/\\/\\/\\/\\/\\/\\/\\/\\/
"
            .trim(),
        );
    }
}
