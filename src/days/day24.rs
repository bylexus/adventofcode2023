use super::Day;
use alex_lib::read_lines;
use regex::Regex;

type Point = (f64, f64, f64);
type Velocivy = (f64, f64, f64);

#[derive(Debug)]
struct Particle {
    position: Point,
    velocity: Velocivy,
}

#[derive(Debug)]
pub struct Day24 {
    input: Vec<String>,
    particles: Vec<Particle>,
}

impl Day24 {
    pub fn new() -> Day24 {
        Day24 {
            input: Vec::new(),
            particles: Vec::new(),
        }
    }

    fn parse_input(&mut self) {
        let matcher = Regex::new(
            r"([0-9-.]+),\s+([0-9-.]+),\s+([0-9-.]+)\s+@\s+([0-9-.]+),\s+([0-9-.]+),\s+([0-9-.]+)",
        )
        .unwrap();

        for line in self.input.iter() {
            if let Some(caps) = matcher.captures(line) {
                let x = caps[1].parse::<f64>().unwrap();
                let y = caps[2].parse::<f64>().unwrap();
                let z = caps[3].parse::<f64>().unwrap();
                let vx = caps[4].parse::<f64>().unwrap();
                let vy = caps[5].parse::<f64>().unwrap();
                let vz = caps[6].parse::<f64>().unwrap();
                self.particles.push(Particle {
                    position: (x, y, z),
                    velocity: (vx, vy, vz),
                });
            }
        }
    }

    fn calc_intersection_2d(&self, p1: &Particle, p2: &Particle) -> Point {
        // Lower grade geometry math: linear equation:
        // find intersection of 2 lines
        // 
        // y = ax + b
        // s = dy / dx (s = slope)
        //
        // --> a = s = dy / dx
        // --> b = y - ax
        // Intersection of 2 lines in 2d:
        // a1*x + b1 = a2*x + b2
        // --> x = (b2 - b1) / (a1 - a2)
        // --> y = a1*x + b1
        let a1 = p1.velocity.1 / p1.velocity.0;
        let a2 = p2.velocity.1 / p2.velocity.0;
        let b1 = p1.position.1 - (a1 * p1.position.0);
        let b2 = p2.position.1 - (a2 * p2.position.0);
        let xi = (b2 - b1) / (a1 - a2);
        let yi = a1 * xi + b1;
        return (xi, yi, 0.0);
    }

    fn point_in_future(&self, p: &Particle, pi: &Point) -> bool {
        if p.velocity.0 < 0.0 && pi.0 > p.position.0 {
            return false;
        }
        if p.velocity.0 > 0.0 && pi.0 < p.position.0 {
            return false;
        }
        if p.velocity.1 < 0.0 && pi.1 > p.position.1 {
            return false;
        }
        if p.velocity.1 > 0.0 && pi.1 < p.position.1 {
            return false;
        }
        return true;
    }
}

impl Day for Day24 {
    fn day_nr(&self) -> String {
        String::from("24")
    }
    fn title(&self) -> String {
        String::from("Never Tell Me The Odds")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day24.txt");
        // let input = read_lines("data/day24-test.txt");
        self.input = input;
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut solution: u64 = 0;
        // println!("Particles: {:?}", self.particles);

        let min_bound: f64 = 200000000000000.0;
        let max_bound: f64 = 400000000000000.0;
        // let min_bound: f64 = 7.0;
        // let max_bound: f64 = 24.0;

        let mut intersects_in_bounds = 0;
        for i1 in 0..self.particles.len() - 1 {
            for i2 in (i1 + 1)..self.particles.len() {
                let p1 = &self.particles[i1];
                let p2 = &self.particles[i2];
                let (xi, yi, zi) = self.calc_intersection_2d(p1, p2);
                if (xi >= min_bound && xi <= max_bound) && (yi >= min_bound && yi <= max_bound) {
                    if self.point_in_future(p1, &(xi, yi, zi))
                        && self.point_in_future(p2, &(xi, yi, zi))
                    {
                        intersects_in_bounds += 1;
                    }
                }
            }
        }

        solution = intersects_in_bounds;

        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let mut solution: u64 = 0;
        String::from(format!("{0}", solution))
    }
}
