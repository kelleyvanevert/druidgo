use druid::{Data, Lens};
use enum_map::{Enum, EnumMap};
use std::ops::Neg;

#[derive(Clone, Debug, Data, Copy, PartialEq, Eq)]
pub struct Pos(pub i32, pub i32);

impl Pos {
    pub fn neighbors(&self, size: usize) -> Vec<Pos> {
        vec![
            Pos(self.0 - 1, self.1),
            Pos(self.0 + 1, self.1),
            Pos(self.0, self.1 - 1),
            Pos(self.0, self.1 + 1),
        ]
        .iter()
        .filter(|&p| p.0 >= 0 && p.0 < (size as i32) && p.1 >= 0 && p.1 < (size as i32))
        .map(|p| *p)
        .collect()
    }

    pub fn valid(&self, size: usize) -> bool {
        self.0 >= 0 && self.0 < (size as i32) && self.1 >= 0 && self.1 < (size as i32)
    }

    pub fn and_valid(&self, size: usize) -> Option<Pos> {
        if self.valid(size) {
            Some(*self)
        } else {
            None
        }
    }

    pub fn index(&self, size: usize) -> Option<usize> {
        if self.valid(size) {
            Some((self.1 as usize) * size + (self.0 as usize))
        } else {
            None
        }
    }
}

impl From<(usize, usize)> for Pos {
    fn from((x, y): (usize, usize)) -> Self {
        Self(x as i32, y as i32)
    }
}

#[derive(Clone, PartialEq, Copy, Enum, Data)]
pub enum Stone {
    White,
    Black,
}

impl Neg for Stone {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

type Board = Vec<Option<Stone>>;

#[derive(Clone, PartialEq, Data, Lens)]
pub struct GameState {
    #[data(eq)]
    pub board: Board,
    #[data(eq)]
    pub captures: EnumMap<Stone, usize>,
}

impl GameState {
    pub fn new(size: usize) -> Self {
        Self {
            board: vec![None; size * size],
            captures: enum_map! {
                Stone::White => 0,
                Stone::Black => 0,
            },
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct Game {
    pub size: usize,
    pub turn: Stone,
    pub state: GameState,
    #[data(eq)]
    history: Vec<GameState>,
}

impl Game {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            turn: Stone::White,
            state: GameState::new(size),
            history: vec![],
        }
    }

    /// Checks whether the structure around position `p` is surrounded, and if so, returns the the whole structure.
    pub fn is_surrounded(&self, p: Pos) -> Option<(Stone, Vec<Pos>)> {
        let mut structure: Vec<Pos> = vec![];

        if let Some(color) = self.stone_at(p) {
            let mut todo: Vec<Pos> = vec![p];

            while let Some(p) = todo.pop() {
                structure.push(p);

                for np in p.neighbors(self.size) {
                    if !self.has_stone_at(np) {
                        return None;
                    } else if let Some(neighbor_color) = self.stone_at(np) {
                        if neighbor_color == color
                            && !structure.contains(&np)
                            && !todo.contains(&np)
                        {
                            todo.push(np);
                        }
                    }
                }
            }

            return Some((color, structure));
        }

        // should not happen
        None
    }

    fn remove_if_surrounded(&mut self, p: Pos) {
        if let Some((color, structure)) = self.is_surrounded(p) {
            let num_captures = structure.len();
            for p in structure {
                if let Some(i) = p.index(self.size) {
                    self.state.board[i] = None;
                }
            }
            self.state.captures[color] += num_captures;
        }
    }

    pub fn try_place_stone(&mut self, p: Pos) {
        if let Some(i) = p.index(self.size) {
            if self.has_stone_at(p) {
                return;
            }

            self.history.push(self.state.clone());

            self.state.board[i] = Some(self.turn);
            for np in p.neighbors(self.size) {
                if self.stone_at(np) == Some(-self.turn) {
                    self.remove_if_surrounded(np);
                }
            }
            self.remove_if_surrounded(p);

            // ko rule
            let len = self.history.len();
            if len >= 2
                && self.history.get(len - 2).map(|s| s.board.clone())
                    == Some(self.state.board.clone())
            {
                self.state = self.history.pop().unwrap();
                return;
            }

            self.turn = -self.turn;
        }
    }

    pub fn stone_at(&self, p: Pos) -> Option<Stone> {
        p.index(self.size).and_then(|i| self.state.board[i])
    }

    pub fn has_stone_at(&self, p: Pos) -> bool {
        None != self.stone_at(p)
    }
}
