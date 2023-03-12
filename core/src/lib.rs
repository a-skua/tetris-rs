use std::fmt;

mod core;

pub type State = core::State;
pub type Position = core::Position;

#[derive(Debug, PartialEq)]
pub enum InputMoveDirection {
    Left,
    Right,
    Bottom,
}

#[derive(Debug, PartialEq)]
pub struct InputMove {
    direction: InputMoveDirection,
    amount: usize,
}

impl InputMove {
    pub fn new(direction: InputMoveDirection, amount: usize) -> Self {
        Self { direction, amount }
    }

    fn can_move(&self) -> bool {
        self.amount > 0
    }

    fn r#move(mut self) -> Self {
        self.amount -= 1;
        self
    }
}

#[cfg(test)]
mod input_move_tests {
    use super::*;

    #[test]
    fn input_move_new() {
        assert_eq!(
            InputMove::new(InputMoveDirection::Left, 3),
            InputMove {
                direction: InputMoveDirection::Left,
                amount: 3
            }
        );
    }
}

#[derive(Debug, PartialEq)]
pub enum InputRotateDirection {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub struct InputRotate {
    direction: InputRotateDirection,
    amount: usize,
}

impl InputRotate {
    pub fn new(direction: InputRotateDirection, amount: usize) -> Self {
        Self { direction, amount }
    }

    fn can_rotate(&self) -> bool {
        self.amount > 0
    }

    fn rotate(mut self) -> Self {
        self.amount -= 1;
        self
    }
}

#[cfg(test)]
mod input_rotate_tests {
    use super::*;

    #[test]
    fn input_rotate_new() {
        assert_eq!(
            InputRotate::new(InputRotateDirection::Left, 2),
            InputRotate {
                direction: InputRotateDirection::Left,
                amount: 2
            }
        );
    }
}

pub enum Input {
    Move(InputMove),
    Rotate(InputRotate),
}

#[derive(Debug, PartialEq, Clone)]
struct Block {
    position: core::Position,
    block: core::Block,
}

impl Block {
    fn new(position: core::Position, block: core::Block) -> Self {
        Self { block, position }
    }

    fn on_inside(&self, p: &core::Position) -> bool {
        p.x >= self.position.x
            && p.x < self.block.size.x + self.position.x
            && p.y >= self.position.y
            && p.y < self.block.size.y + self.position.y
    }

    fn state(&self, p: &core::Position) -> core::State {
        if self.on_inside(p) {
            self.block.state(&p.sub(&self.position))
        } else {
            core::State::Empty
        }
    }

    fn can_change(&self, board: &core::Board) -> bool {
        board.can_change(&self.position, &self.block)
    }

    fn reverse_move(mut self, direction: &InputMoveDirection) -> Self {
        let direction = match direction {
            InputMoveDirection::Left => core::MoveDirection::Right,
            InputMoveDirection::Right => core::MoveDirection::Left,
            InputMoveDirection::Bottom => core::MoveDirection::Top,
        };
        self.position = self.position.r#move(&direction);
        self
    }

    fn r#move(mut self, direction: &InputMoveDirection) -> Self {
        let direction = match direction {
            InputMoveDirection::Left => core::MoveDirection::Left,
            InputMoveDirection::Right => core::MoveDirection::Right,
            InputMoveDirection::Bottom => core::MoveDirection::Bottom,
        };
        self.position = self.position.r#move(&direction);
        self
    }

    fn reverse_rotate(mut self, direction: &InputRotateDirection) -> Self {
        let direction = match direction {
            InputRotateDirection::Left => core::RotateDirection::Right,
            InputRotateDirection::Right => core::RotateDirection::Left,
        };
        self.block = self.block.rotate(direction);
        self
    }

    fn rotate(mut self, direction: &InputRotateDirection) -> Self {
        let direction = match direction {
            InputRotateDirection::Left => core::RotateDirection::Left,
            InputRotateDirection::Right => core::RotateDirection::Right,
        };
        self.block = self.block.rotate(direction);
        self
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Game {
    board: core::Board,
    block: Option<Block>,
}

impl Game {
    pub fn new() -> Self {
        let board = core::Board::new();
        let block = None;
        Self { board, block }
    }

    pub fn input(self, input: Input) -> Self {
        match input {
            Input::Move(input) => self.input_move(input),
            Input::Rotate(input) => self.input_rotate(input),
        }
    }

    fn input_move(mut self, input: InputMove) -> Self {
        if let None = self.block {
            return self;
        }

        if !input.can_move() {
            return self;
        }

        let block = self.block.unwrap().r#move(&input.direction);

        if !block.can_change(&self.board) {
            self.block = Some(block.reverse_move(&input.direction));
            return self;
        }

        self.block = Some(block);
        return self.input_move(input.r#move());
    }

    fn input_rotate(mut self, input: InputRotate) -> Self {
        if let None = self.block {
            return self;
        }

        if !input.can_rotate() {
            return self;
        }

        let block = self.block.unwrap().rotate(&input.direction);

        if !block.can_change(&self.board) {
            self.block = Some(block.reverse_rotate(&input.direction));
            return self;
        }

        self.block = Some(block);
        self.input_rotate(input.rotate())
    }

    /// return self and removed row-count
    pub fn deside(mut self) -> (Self, usize) {
        match self.block {
            None => {
                self.block = Some(Block::new(core::Position::new(3, 0), core::Block::new()));
            }
            Some(block) => {
                let direction = InputMoveDirection::Bottom;
                let block = block.r#move(&direction);
                if block.can_change(&self.board) {
                    self.block = Some(block);
                } else {
                    let block = block.reverse_move(&direction);
                    self.board = self.board.set_block(&block.position, block.block);
                    self.block = None;
                }
            }
        }

        let (board, cnt) = self.board.remove_valid_rows();
        self.board = board;
        (self, cnt)
    }

    pub fn size(&self) -> core::Size {
        self.board.size
    }

    pub fn state(&self, p: Position) -> core::State {
        if let Some(block) = &self.block {
            if let core::State::Block(t) = block.state(&p) {
                return core::State::Block(t);
            }
        }
        self.board.state(&p)
    }

    pub fn table(&self) -> Vec<Vec<core::State>> {
        let mut table = Vec::with_capacity(self.board.size.y);
        for y in 0..self.board.size.y {
            let mut row = Vec::with_capacity(self.board.size.x);
            for x in 0..self.board.size.x {
                let p = core::Position::new(x, y);
                if let Some(block) = &self.block {
                    if let core::State::Block(t) = block.state(&p) {
                        row.push(core::State::Block(t));
                        continue;
                    }
                }

                row.push(self.board.state(&p));
            }
            table.push(row);
        }
        table
    }
}

#[cfg(test)]
mod game_tests {
    use super::*;

    #[test]
    fn game_input() {
        let game = Game::new();
        assert_eq!(
            game.input(Input::Move(InputMove::new(InputMoveDirection::Left, 3))),
            Game::new()
        );

        let mut game = Game::new();
        game.block = Some(Block::new(core::Position::new(0, 0), core::Block::new_s()));
        let game = game;
        let mut expect = Game::new();
        expect.block = Some(Block::new(core::Position::new(0, 0), core::Block::new_s()));
        let expect = expect;
        assert_eq!(
            game.input(Input::Move(InputMove::new(InputMoveDirection::Left, 3))),
            expect,
        );

        let mut game = Game::new();
        game.block = Some(Block::new(core::Position::new(0, 0), core::Block::new_s()));
        let game = game;
        let mut expect = Game::new();
        expect.block = Some(Block::new(core::Position::new(7, 0), core::Block::new_s()));
        let expect = expect;
        assert_eq!(
            game.input(Input::Move(InputMove::new(InputMoveDirection::Right, 10))),
            expect,
        );

        let mut game = Game::new();
        game.block = Some(Block::new(core::Position::new(0, 0), core::Block::new_s()));
        let game = game;
        let mut expect = Game::new();
        expect.block = Some(Block::new(core::Position::new(0, 18), core::Block::new_s()));
        let expect = expect;
        assert_eq!(
            game.input(Input::Move(InputMove::new(InputMoveDirection::Bottom, 20))),
            expect,
        );

        let mut game = Game::new();
        game.block = Some(Block::new(core::Position::new(0, 0), core::Block::new_s()));
        let game = game;
        let mut expect = Game::new();
        expect.block = Some(Block::new(
            core::Position::new(0, 0),
            core::Block::new_s().rotate(core::RotateDirection::Left),
        ));
        let expect = expect;
        assert_eq!(
            game.input(Input::Rotate(InputRotate::new(
                InputRotateDirection::Left,
                1
            ))),
            expect,
        );
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = String::new();
        for y in 0..self.board.size.y {
            text += "<!";
            for x in 0..self.board.size.x {
                let block = if let Some(block) = &self.block {
                    block
                } else {
                    let p = core::Position::new(x, y);
                    text += &format!("{}", self.board.state(&p));
                    continue;
                };

                let p = core::Position::new(x, y);
                let block = block.state(&p);
                match block {
                    core::State::Block(_) => text += &format!("{block}"),
                    _ => text += &format!("{}", self.board.state(&p)),
                }
            }
            text += "!>\n";
        }
        text += "<!";
        for _ in 0..self.board.size.x {
            text += "==";
        }
        text += "!>\n";
        text += "  ";
        for _ in 0..self.board.size.x {
            text += "\\/";
        }
        write!(f, "{text}")
    }
}

#[cfg(test)]
mod game_display_tests {
    use super::*;

    #[test]
    fn display_game() {
        let mut game = Game::new();
        game.block = Some(Block {
            block: core::Block::new_s(),
            position: core::Position::new(4, 0),
        });

        game.board = game.board.set_block(
            &Position::new(0, 19),
            core::Block::new_i().rotate(core::RotateDirection::Left),
        );
        game.board = game.board.set_block(
            &Position::new(4, 19),
            core::Block::new_i().rotate(core::RotateDirection::Left),
        );

        assert_eq!(
            format!("{game}"),
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
<![][][][][][][][] . .!>
<!====================!>
  \\/\\/\\/\\/\\/\\/\\/\\/\\/\\/
"
            .trim(),
        );
    }
}
