mod days;
use days::{
    Day, Day00, Day01, Day02, Day03, Day04, Day05, Day06, Day07, Day08, Day09, Day10, Day11,
    Day12, Day13, Day14, Day15, Day16, Day17, Day18, Day19, Day20, Day21, Day22, Day23, Day24,
    Day25, DayTest,
};
use std::env;
use std::time::{Duration, Instant};
use std::vec::Vec;

#[derive(Debug)]
struct Result {
    day: String,
    title: String,
    solution1: String,
    solution2: String,
    runtime1: Duration,
    runtime2: Duration,
    preptime: Duration,
    totaltime: Duration,
}

fn main() {
    let mut probs = Vec::<Box<dyn Day>>::new();
    // Create the problems instances to solve:
    for a in env::args().skip(1) {
        probs.push(match a.as_str() {
            "test" => Box::new(DayTest::new()),
            "0" => Box::new(Day00::new()),
            "1" => Box::new(Day01::new()),
            "2" => Box::new(Day02::new()),
            "3" => Box::new(Day03::new()),
            "4" => Box::new(Day04::new()),
            "5" => Box::new(Day05::new()),
            "6" => Box::new(Day06::new()),
            "7" => Box::new(Day07::new()),
            "8" => Box::new(Day08::new()),
            "9" => Box::new(Day09::new()),
            "10" => Box::new(Day10::new()),
            "11" => Box::new(Day11::new()),
            "12" => Box::new(Day12::new()),
            "13" => Box::new(Day13::new()),
            "14" => Box::new(Day14::new()),
            "15" => Box::new(Day15::new()),
            "16" => Box::new(Day16::new()),
            "17" => Box::new(Day17::new()),
            "18" => Box::new(Day18::new()),
            "19" => Box::new(Day19::new()),
            "20" => Box::new(Day20::new()),
            "21" => Box::new(Day21::new()),
            "22" => Box::new(Day22::new()),
            "23" => Box::new(Day23::new()),
            "24" => Box::new(Day24::new()),
            "25" => Box::new(Day25::new()),
            _ => panic!("Unknown problem"),
        })
    }

    // Run them:
    let mut results: Vec<Result> = Vec::new();
    let start = Instant::now();
    probs.iter_mut().for_each(|p| {
        let title = p.title();
        let day = p.day_nr();
        let now = Instant::now();

        p.prepare();
        let preptime = now.elapsed();

        let now_solution1 = Instant::now();
        let solution1 = p.solve1();
        let runtime1 = now_solution1.elapsed();

        let now_solution2 = Instant::now();
        let solution2 = p.solve2();
        let runtime2 = now_solution2.elapsed();

        let totaltime = now.elapsed();

        let result = Result {
            day,
            title,
            solution1,
            runtime1,
            solution2,
            runtime2,
            preptime,
            totaltime,
        };
        results.push(result);
    });

    // Output results:
    results.iter().for_each(|r| {
        println!(
            "\n\n{day} - {title}: prep time: {p:?}, total time: {t:?}",
            day = r.day,
            title = r.title,
            p = r.preptime,
            t = r.totaltime
        );
        println!(
            "     Solution 1: \x1B[1;97m {s} \x1B[0m\n     runtime: {t:?}\n",
            s = r.solution1,
            t = r.runtime1
        );
        println!(
            "     Solution 2: \x1B[1;97m {s} \x1B[0m\n     runtime: {t:?}\n\n",
            s = r.solution2,
            t = r.runtime2
        );
    });

    let total_duration = start.elapsed();

    println!("\n\nGrand Total runtime: {t:?}\n", t = total_duration);
}
