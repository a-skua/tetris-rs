use std::fmt;
use tetris::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Tetris {
    game: Game,
}

impl fmt::Display for Tetris {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.game.fmt(f)
    }
}

#[wasm_bindgen]
pub enum JsState {
    Empty,
    Block,
}

#[wasm_bindgen]
pub enum JsInput {
    MoveLeft,
    MoveRight,
    MoveTop,
    MoveBottom,
    RotateLeft,
    RotateRight,
}

#[wasm_bindgen]
impl Tetris {
    pub fn new() -> Tetris {
        Self { game: Game::new() }
    }

    pub fn deside(&mut self) -> usize {
        let (game, count) = self.game.clone().deside();
        self.game = game;
        count
    }

    pub fn input(&mut self, input: JsInput) {
        let input = match input {
            JsInput::MoveLeft => Input::Move(InputMove::new(InputMoveDirection::Left, 1)),
            JsInput::MoveRight => Input::Move(InputMove::new(InputMoveDirection::Right, 1)),
            JsInput::MoveBottom => Input::Move(InputMove::new(InputMoveDirection::Bottom, 1)),
            JsInput::MoveTop => Input::Move(InputMove::new(InputMoveDirection::Bottom, 20)),
            JsInput::RotateLeft => Input::Rotate(InputRotate::new(InputRotateDirection::Left, 1)),
            JsInput::RotateRight => Input::Rotate(InputRotate::new(InputRotateDirection::Right, 1)),
        };
        self.game = self.game.clone().input(input);
    }

    pub fn to_string(&self) -> String {
        format!("{self}")
    }

    pub fn size_x(&self) -> usize {
        self.game.size().x
    }

    pub fn size_y(&self) -> usize {
        self.game.size().y
    }

    pub fn state(&self, x: usize, y: usize) -> JsState {
        match self.game.state(Position::new(x, y)) {
            State::Empty => JsState::Empty,
            State::Block(..) => JsState::Block,
        }
    }
}
