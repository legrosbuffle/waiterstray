use serde::Serialize;

const NUM_BOTTLES : usize = 6;

// The tray has the following shape:
//   __       _
//     _ _____
//      _
const MAX_TRAY_OFFSET: usize = 17;
const TRAY_SHAPE_SIZE : usize = MAX_TRAY_OFFSET + NUM_BOTTLES;
const TRAY_SHAPE: [usize; TRAY_SHAPE_SIZE] = [0,0,0,0,0,0,0,2,2,1,0,1,1,1,1,1,2,0,0,0,0,0,0];


// Bottles have three positions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum BottleState {
  Top,
  Middle,
  Bottom,
}

// Marbles are in L-shaped grooves. And have 3 positions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
pub enum MarbleState {
  TopLeft,
  BottomLeft,
  BottomRight,
}

// Possible moves are:
#[derive(Clone, Debug)]
pub enum Move {
  // Move the tray one position left.
  TrayLeft,
  // Move the tray one position right.
  TrayRight,
  // Move marble left.
  MarbleLeft(usize),
  // Move marble right
  MarbleRight(usize),
  // Move bottle up.
  BottleUp(usize),
  // Move bottle down.
  BottleDown(usize),
  // Invalid move
  Invalid,
}

// The puzzle state.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct State {
  // There are NUM_BOTTLES bottles.
  pub bottles: [BottleState; NUM_BOTTLES],
  // Each bottle has a groove that overlaps with the next one.
  // The rightmost groove sticks out of the rightmost bottle.
  pub marbles: [MarbleState; NUM_BOTTLES],
  pub tray_offset: usize,
}

impl State {
  // The puzze is solved when the tray can be extracted at either end.
  pub fn is_solved(self: &Self) -> bool {
    self.tray_offset == 0 || self.tray_offset == MAX_TRAY_OFFSET
  }
}

// Moves bottle up.
pub fn bottle_up(s: BottleState) -> BottleState {
  match s {
    BottleState::Top => { panic!("invalid move"); },
    BottleState::Middle => BottleState::Top,
    BottleState::Bottom => BottleState::Middle,
  }
}

// Moves bottle down.
pub fn bottle_down(s: BottleState) -> BottleState {
  match s {
    BottleState::Top => BottleState::Middle,
    BottleState::Middle => BottleState::Bottom,
    BottleState::Bottom => { panic!("invalid move"); },
  }
}

// Returns the inverse move.
pub fn reverse(mv: Move) -> Move {
  match mv {
    Move::TrayLeft => Move::TrayRight,
    Move::TrayRight => Move::TrayLeft,
    Move::MarbleLeft(i) => Move::MarbleRight(i),
    Move::MarbleRight(i) => Move::MarbleLeft(i),
    Move::BottleUp(i) => Move::BottleDown(i),
    Move::BottleDown(i) => Move::BottleUp(i),
    Move::Invalid => Move::Invalid,
  }
}

// An iterator that iterates all next possible states given by a valid 
// move from a given state.
pub struct NextStatesIter<'a> {
  state: &'a State,
  next_move: Move, 
}

impl<'a> NextStatesIter<'a> {
  pub fn new(state: &'a State) -> Self {
    Self {
      state, 
      next_move: Move::TrayLeft,
    }
  }
}

fn is_valid_tray_bottle_position(tray_offset: usize, i: usize, b: BottleState) -> bool {
      let tray_height = TRAY_SHAPE[MAX_TRAY_OFFSET + i - tray_offset];
      match b {
        BottleState::Top => true,
        BottleState::Middle => tray_height <= 1,
        // A bottle can only be in the bottom state if on the notch,
        // fully before, or fully past the tray.
        BottleState::Bottom => tray_height <= 0,
      }
}

fn is_valid_tray_position(tray_offset: usize, bottles: &[BottleState]) -> bool {
    for i in 0..NUM_BOTTLES {
      if !is_valid_tray_bottle_position(tray_offset, i, bottles[i]) {
        return false;
      }
    }
    true
}

impl<'a> std::iter::Iterator for NextStatesIter<'a> {
  type Item = (Move, State);
  
  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let cur_move = self.next_move.clone();
      match cur_move {
        Move::TrayLeft => {
          self.next_move = Move::TrayRight;
          if is_valid_tray_position(self.state.tray_offset - 1, &self.state.bottles) {
            let mut next_state = self.state.clone();
            next_state.tray_offset -= 1;
            return Some((cur_move,next_state));
          }
        },
        Move::TrayRight => {
          self.next_move = Move::MarbleLeft(0);
          if is_valid_tray_position(self.state.tray_offset + 1, &self.state.bottles) {
            let mut next_state = self.state.clone();
            next_state.tray_offset += 1;
            return Some((cur_move,next_state));
          }
        },
        Move::MarbleLeft(i) => {
          self.next_move = if i + 1 == NUM_BOTTLES { Move::MarbleRight(0) } else { Move::MarbleLeft(i+1) };
          if self.state.marbles[i] == MarbleState::BottomRight && self.state.bottles[i] == BottleState::Bottom {
            let mut next_state = self.state.clone();
            next_state.marbles[i] = MarbleState::BottomLeft;
            return Some((cur_move,next_state));
          }
        },
        Move::MarbleRight(i) => {
          self.next_move = if i + 1 == NUM_BOTTLES { Move::BottleUp(0) } else { Move::MarbleRight(i+1) };
          if self.state.marbles[i] == MarbleState::BottomLeft && (i + 1 == NUM_BOTTLES || self.state.bottles[i+1] != BottleState::Bottom) {
            let mut next_state = self.state.clone();
            next_state.marbles[i] = MarbleState::BottomRight;
            return Some((cur_move,next_state));
          }
        },
        Move::BottleUp(i) => {
          self.next_move = if i + 1 == NUM_BOTTLES { Move::BottleDown(0) } else { Move::BottleUp(i+1) };
          if i > 0 && self.state.marbles[i-1] == MarbleState::BottomRight {
            // The bottle is stuck because of left marble.
            continue;
          }
            let right_marble = self.state.marbles[i];
            match (self.state.bottles[i], right_marble) {
              (BottleState::Top, _) => {},  // Already at the top.
              (BottleState::Middle, MarbleState::TopLeft) => {},  // Marble prevents moving up.
              (BottleState::Bottom, MarbleState::BottomLeft) => {
                // Move both bottle and right marble one step up.
                let mut next_state = self.state.clone();
                next_state.bottles[i] = BottleState::Middle;
                next_state.marbles[i] = MarbleState::TopLeft;
                return Some((cur_move,next_state));
              },
              (bottle, MarbleState::BottomRight) => {
                // Move bottle one step up.
                let mut next_state = self.state.clone();
                next_state.bottles[i] = bottle_up(bottle);
                return Some((cur_move,next_state));
              },
              _ => {
                eprintln!("{:?} {:?}", self.state.bottles[i], right_marble);
                panic!("invalid input state");
              },
            }
        }
        Move::BottleDown(i) => {
          self.next_move = if i + 1 == NUM_BOTTLES { Move::Invalid } else { Move::BottleDown(i+1) };
          if i > 0 && self.state.marbles[i-1] == MarbleState::BottomRight {
            // The bottle is stuck because of left marble.
            continue;
          }
          let bottle = self.state.bottles[i];
          if bottle != BottleState::Bottom && is_valid_tray_bottle_position(self.state.tray_offset, i, bottle_down(bottle)) {
            // Move bottle one step down.
            let mut next_state = self.state.clone();
            next_state.bottles[i] = bottle_down(bottle);
            if next_state.marbles[i] == MarbleState::TopLeft {
              next_state.marbles[i] = MarbleState::BottomLeft;
            }
            return Some((cur_move,next_state));
          }
        }
        Move::Invalid => { return None; }
      }
    }
  }
}
