# The Waiter's Tray: Solving the puzzle automatically

A couple years ago two friends of my wife gave me a puzzle called the [Waiter's Tray](https://www.logicagiochi.com/fr/the-waiters-tray-le-vin-est-servi) from [Jean Claude Constantin](http://www.constantin-jean-clau.de/) (merci Gaëlle et Aurélie!). This was a fun little puzzle and took me some time to solve.

Now of course it's always fun to code the solution to a puzzle once you've solved it, this one is no exception, so here's some code to solve it and visualize the result.

The solver is written in [rust](https://www.rust-lang.org/), and dumps the solution as a JSON array of itermediate states that can be visualized in a web page in the browser (there is a live version [here](http://legrosbuffle.github.io/waiterstray/waiterstray.html)).


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

## Analyzing the puzzle

If we encode the marble positions as a binary number `G` with the `i`-th bit corresponding to the position of the `i`-th marble (`TopLeft` == 1, `Bottom*` == 0), then we can see that the the interlocking mechanism only allow us to change single bit of the value at a time. This kind of sequence is known as a *Reflected Binary Code* (a.k.a *Gray Code*) [[wikipedia](https://en.wikipedia.org/wiki/Gray_code)].

The freeing condition for the tray is for all the bottles to be in the `Top` state. For this to happen, the only possibility is for all the marbles to be in the `BottomRight` state. This corresponds to `G=0`. When unboxing the puzzle, we start at the other extreme of the sequence: `G=32`. In between, we go through the full 6-bit Reflected Binary Code sequence:

```
32, 33, 35, 34, 38, 39, 37, 36,
44, 45, 47, 46, 42, 43, 41, 40,
56, 57, 59, 58, 62, 63, 61, 60,
52, 53, 55, 54, 50, 51, 49, 48,
16, 17, 19, 18, 22, 23, 21, 20,
28, 29, 31, 30, 26, 27, 25, 24,
 8,  9, 11, 10, 14, 15, 13, 12,
 4,  5,  7,  6,  2,  3,  1,  0
```

So the puzzle is essentially a **6-bit mechanical Reflected Binary Code reverse counter**.

