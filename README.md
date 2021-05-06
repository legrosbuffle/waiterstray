# The Waiter's Tray: Solving the puzzle automatically

A couple years ago two friends of my wife gave me a puzzle called the [Waiter's Tray](https://www.logicagiochi.com/fr/the-waiters-tray-le-vin-est-servi) from [Jean Claude Constantin](http://www.constantin-jean-clau.de/) (merci Gaëlle et Aurélie!). This was a fun little puzzle and took me some time to solve.

Now of course it's always fun to code the solution to a puzzle once you've solved it, this one is no exception, so here's some code to solve it and visualize the result.

The solver is written in [rust](https://www.rust-lang.org/), and dumps the solution as a JSON array of itermediate states that can be visualized in a web page in the browser (there is a live version [here](http://legrosbuffle.github.io/waiterstray/index.html)).


## Running

Build and run the solver, writing the solution to `solution.js`:

```
cargo run --release > solution.js
```

Point your browser to `file:///path/to/git/repo/waiterstray.html`.


## The Solver

The representation of the puzzle state is quite simple: Every bottle can be in one of three positions (top, middle, bottom), each marble can be in one of three positions (top left, bottom left, bottom right), and the tray translates from left to right (in my model, the tray is given discrete positions as it's all that matters any way).

```

pub enum BottleState {
  Top,
  Middle,
  Bottom,
}

pub enum MarbleState {
  TopLeft,
  BottomLeft,
  BottomRight,
}

pub struct State {
  pub bottles: [BottleState; NUM_BOTTLES],
  pub marbles: [MarbleState; NUM_BOTTLES],
  pub tray_offset: usize,
}
```

When moving from one state to another, one can move the tray one step left or right, move any bottle up or down, or move any marble along its path. So the set of possible moves can be reprensented by the enum:

```
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
```

The tray migh block the bottles, the bottles might block the tray, and the marbles might lock the bottles in place; which gives us a set of constraints for valid moves given the puzzle state. The solver simply does a BFS on the graph of `{V=states, E=valid moves}`, building the graph lazily starting from the original state.

The solution (364 moves) is found after exploring 3232 states (takes 34 milliseconds on my machine).

