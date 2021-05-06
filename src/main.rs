use std::collections::{HashMap,VecDeque};
use waiterstray::state::{BottleState, MarbleState, State, NextStatesIter, Move, reverse};

fn main() {
  let start = State {
    bottles: [BottleState::Middle, BottleState::Middle, BottleState::Middle, BottleState::Middle, BottleState::Middle, BottleState::Middle],
    marbles: [MarbleState::TopLeft, MarbleState::BottomRight, MarbleState::BottomRight, MarbleState::BottomRight, MarbleState::BottomRight, MarbleState::TopLeft],
    tray_offset: 7,
  };
  
  let mut seen = HashMap::new();
  seen.insert(start.clone(), (Move::Invalid, start.clone()));
  
  let mut queue = VecDeque::new();
  queue.push_back(start);

  let mut num_explored = 0;
  while !queue.is_empty() {
    let state = queue.pop_front().unwrap();
    num_explored += 1;
    for (change, next_state) in NextStatesIter::new(&state) {
      if next_state.is_solved() {
        eprintln!("Found solution: {:?}\n", next_state);
        println!("var solution = [");
        // Walk up the path.
        let mut cur = state;
        let mut num_moves = 0;
        loop {
          println!("{},", serde_json::to_string(&cur).unwrap());
          let (mv, prev) = seen.remove(&cur).unwrap();
          if let Move::Invalid = mv {
            break;
          }
          eprintln!("{:?}", reverse(mv));
          cur = prev;
          num_moves += 1;
        }
        println!("];");
        eprintln!("in {} moves, after exploring {} states\n", num_moves, num_explored);
        return;
      }
      // eprintln!("{:?}", next);
      if !seen.contains_key(&next_state) {
        seen.insert(next_state.clone(), (change, state.clone()));
        queue.push_back(next_state);
      }
    }
  }
    
  eprintln!("Could not find a solution!");
}
