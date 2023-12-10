use std::{
    collections::{hash_map::Iter, HashMap},
    fmt::Display,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Coord2d {
    pub x: i64,
    pub y: i64,
}

const UP_1: Coord2d = Coord2d { x: 0, y: -1 };
const RIGHT_1: Coord2d = Coord2d { x: 1, y: 0 };
const DOWN_1: Coord2d = Coord2d { x: 0, y: 1 };
const LEFT_1: Coord2d = Coord2d { x: -1, y: 0 };

#[allow(dead_code)]
impl Coord2d {
    pub fn add(&self, amount: &Coord2d) -> Coord2d {
        Coord2d {
            x: self.x + amount.x,
            y: self.y + amount.y,
        }
    }
    pub fn up(&self) -> Coord2d {
        self.add(&UP_1)
    }
    pub fn right(&self) -> Coord2d {
        self.add(&RIGHT_1)
    }
    pub fn down(&self) -> Coord2d {
        self.add(&DOWN_1)
    }
    pub fn left(&self) -> Coord2d {
        self.add(&LEFT_1)
    }
    pub fn up_n(&self, amount: i64) -> Coord2d {
        self.add(&Coord2d { x: 0, y: -amount })
    }
    pub fn right_n(&self, amount: i64) -> Coord2d {
        self.add(&Coord2d { x: amount, y: 0 })
    }
    pub fn down_n(&self, amount: i64) -> Coord2d {
        self.add(&Coord2d { x: 0, y: amount })
    }
    pub fn left_n(&self, amount: i64) -> Coord2d {
        self.add(&Coord2d { x: -amount, y: 0 })
    }

    pub fn manhattan_dist(&self, other: &Coord2d) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[allow(dead_code)]
impl Display for Coord2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x:{}, y:{})", self.x, self.y)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Coord2dMap<T> {
    map: HashMap<Coord2d, T>,
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
}

#[allow(dead_code)]
impl<T> Coord2dMap<T> {
    pub fn new() -> Self {
        Coord2dMap {
            map: HashMap::new(),
            min_x: i64::MAX,
            min_y: i64::MAX,
            max_x: i64::MIN,
            max_y: i64::MIN,
        }
    }

    pub fn insert(&mut self, key: Coord2d, value: T) -> Option<T> {
        if key.x > self.max_x {
            self.max_x = key.x
        }
        if key.x < self.min_x {
            self.min_x = key.x
        }
        if key.y > self.max_y {
            self.max_y = key.y
        }
        if key.y < self.min_y {
            self.min_y = key.y
        }
        self.map.insert(key, value)
    }

    pub fn iter(&self) -> Iter<Coord2d, T> {
        self.map.iter()
    }

    pub fn get(&self, key: &Coord2d) -> Option<&T> {
        self.map.get(key)
    }
    pub fn contains_key(&self, key: &Coord2d) -> bool {
        self.map.contains_key(key)
    }

    pub fn remove(&mut self, key: &Coord2d) -> Option<T> {
        self.map.remove(key)
    }

    pub fn width(&self) -> i64 {
        if self.min_x < i64::MAX && self.max_x > i64::MIN {
            self.max_x - self.min_x + 1
        } else {
            0
        }
    }

    pub fn height(&self) -> i64 {
        if self.min_y < i64::MAX && self.max_y > i64::MIN {
            self.max_y - self.min_y + 1
        } else {
            0
        }
    }
}

#[allow(dead_code)]
impl<T: Display> Display for Coord2dMap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dsp = String::new();
        if self.map.len() > 0 {
            for y in self.min_y..=self.max_y {
                for x in self.min_x..=self.max_x {
                    let val = self.map.get(&Coord2d { x, y });
                    if let Some(v) = val {
                        dsp.push_str(v.to_string().as_str());
                    }
                }
                dsp.push('\n');
            }
        }
        write!(f, "{}", dsp)
    }
}
