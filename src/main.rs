mod days;
use days::{Day, Day00, Day01, Day02};
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
            "0" => Box::new(Day00::new()),
            "1" => Box::new(Day01::new()),
            "2" => Box::new(Day02::new()),
            &_ => panic!("Unknown problem"),
        })
    }

    // Run them:
    let mut results: Vec<Result> = Vec::new();
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
}
