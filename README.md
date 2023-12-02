# Advent of Code 2023

Another Year, Another Try! But Remember - Rust or Die! Muahahahaa!

As every year, I try to solve the [Advent of Code](https://adventofcode.com/) challenges.
Last year I tried Rust, gave up after day 12, and switched to GO.

This year, I try harder - I want to learn Rust for real, so let's see if I get it this year.

Take this repo as inspiration, deterrence, ideas, if you like.

Happy coding!

alex

## How to use it

```shell
$> cargo run [problem-nr ...]
```

## How to add a new Problem

### 1. Create a new day file: `src/days/dayxx.rs`

I create a new module for each day, with a type that implements the `Day` trait:

```rs
use super::Day;

pub struct Day03 {
    input: Vec<String>,
}

impl Day03 {
    pub fn new() -> Day03 {
        Day03 { input: Vec::new() }
    }

    fn parse_input(&mut self) {}
}

impl Day for Day03 {
    fn day_nr(&self) -> String {
        String::from("03")
    }
    fn title(&self) -> String {
        String::from("Day 3: xxx")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day03.txt");
        self.input = input;
        self.parse_input();
    }

    fn solve1(&self) -> String {
        let mut sum = 0;
        String::from(format!("{0}", sum))
    }
    fn solve2(&self) -> String {
        let mut sum = 0;
        String::from(format!("{0}", sum))
    }
}
```

### 2. Export the module

In `src/days.rs`, add the new module and re-export it:

```rs
// src/days.rs:
// ...
pub mod day03;
pub use day03::Day03;
// ...
```

### 3. Add it in the main program

In the main function, add a new command line param that instantiates the new problem:

```rs
// main.rs:
use days::{Day03, /* .....*/ };

// .....
fn main() {
    let mut probs = Vec::<Box<dyn Day>>::new();
    for a in env::args().skip(1) {
        probs.push(match a.as_str() {
			// ...
            "3" => Box::new(Day03::new()),
			// ...
        })
    }
	// .....
}
```

### 4. Run it

Run the new problem:

```sh
$ cargo run 3
```
