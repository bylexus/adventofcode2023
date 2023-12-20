use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    marker::PhantomData,
    rc::Rc,
};

use super::Day;
use alex_lib::read_lines;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PulseLevel {
    LOW,
    HIGH,
}

#[derive(Debug, Clone)]
struct Pulse {
    from: String,
    level: PulseLevel,
    module_name: String,
}

type PulseQueue = Rc<RefCell<VecDeque<Pulse>>>;

#[derive(Debug)]
struct FlipFlopModule {
    name: String,
    pulse_queue: PulseQueue,
    is_on: bool,
    destinations: Vec<String>,
}
impl FlipFlopModule {
    fn send(&mut self, pulse: &Pulse) {
        let mut q = self.pulse_queue.borrow_mut();
        if let PulseLevel::LOW = pulse.level {
            self.is_on = !self.is_on;
            let send_pulse = match self.is_on {
                true => PulseLevel::HIGH,
                false => PulseLevel::LOW,
            };
            for dest in self.destinations.iter() {
                // println!("{} ->{:?}-> {}", self.name, send_pulse, dest);
                q.push_back(Pulse {
                    from: self.name.to_string(),
                    level: send_pulse,
                    module_name: dest.to_string(),
                });
            }
        }
    }
}

#[derive(Debug)]
struct ConjunctionModule {
    name: String,
    pulse_queue: PulseQueue,
    input_states: HashMap<String, PulseLevel>,
    destinations: Vec<String>,
}
impl ConjunctionModule {
    fn send(&mut self, pulse: &Pulse) {
        let mut q = self.pulse_queue.borrow_mut();
        // remember input state of received pulse:
        self.input_states.insert(pulse.from.clone(), pulse.level);

        // check if all inputs are HIGH
        let mut all_high = true;
        for state in self.input_states.values() {
            if let PulseLevel::LOW = state {
                all_high = false;
                break;
            }
        }
        let send_pulse = match all_high {
            true => PulseLevel::LOW,
            false => PulseLevel::HIGH,
        };

        if let PulseLevel::LOW = pulse.level {}
        for dest in self.destinations.iter() {
            // println!("{} ->{:?}-> {}", self.name, send_pulse, dest);
            q.push_back(Pulse {
                from: self.name.to_string(),
                level: send_pulse,
                module_name: dest.to_string(),
            });
        }
    }
}

#[derive(Debug)]
struct BroadcastModule {
    name: String,
    pulse_queue: PulseQueue,
    destinations: Vec<String>,
}
impl BroadcastModule {
    fn send(&mut self, pulse: &Pulse) {
        let mut q = self.pulse_queue.borrow_mut();
        for dest in self.destinations.iter() {
            // println!("{} ->{:?}-> {}", self.name, pulse.level, dest);
            q.push_back(Pulse {
                from: self.name.clone(),
                level: pulse.level.clone(),
                module_name: dest.to_string(),
            });
        }
    }
}

#[derive(Debug)]
struct OutputModule {
    name: String,
    pulse_queue: PulseQueue,
}
impl OutputModule {
    fn send(&mut self, pulse: &Pulse) {
        // println!("{} ->{:?}-> {}", pulse.from, pulse.level, self.name);
    }
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
    Broadcast(BroadcastModule),
    Output(OutputModule),
}

type ModuleMap = HashMap<String, ModuleType>;

#[derive(Debug)]
pub struct Day20 {
    input: Vec<String>,
    module_map: Rc<RefCell<ModuleMap>>,
    pulse_queue: PulseQueue,
}

impl Day20 {
    pub fn new() -> Day20 {
        Day20 {
            input: Vec::new(),
            module_map: Rc::new(RefCell::new(HashMap::new())),
            pulse_queue: Rc::new(RefCell::new(VecDeque::new())),
        }
    }

    fn parse_input(&mut self) {
        let matcher = Regex::new(r"(([%&]?)(\w+)) -> (.*)").unwrap();
        self.module_map = Rc::new(RefCell::new(HashMap::new()));
        self.pulse_queue = Rc::new(RefCell::new(VecDeque::new()));

        for line in self.input.iter() {
            if let Some(caps) = matcher.captures(line) {
                let mtype = caps.get(2).unwrap().as_str();
                let module_name = caps.get(3).unwrap().as_str();
                let outputs = caps
                    .get(4)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<String>>();

                // initialize all dest modules with a simple "output" module
                for mname in outputs.iter() {
                    if !self.module_map.borrow().contains_key(mname) {
                        self.module_map.borrow_mut().insert(
                            mname.to_string(),
                            ModuleType::Output(OutputModule {
                                name: mname.to_string(),
                                pulse_queue: self.pulse_queue.clone(),
                            }),
                        );
                    }
                }
                if module_name == "broadcaster" {
                    self.module_map.borrow_mut().insert(
                        module_name.to_string(),
                        ModuleType::Broadcast(BroadcastModule {
                            name: module_name.to_string(),
                            pulse_queue: self.pulse_queue.clone(),
                            destinations: outputs.to_vec(),
                        }),
                    );
                } else if module_name == "output" {
                    self.module_map.borrow_mut().insert(
                        module_name.to_string(),
                        ModuleType::Output(OutputModule {
                            name: module_name.to_string(),
                            pulse_queue: self.pulse_queue.clone(),
                        }),
                    );
                } else {
                    match mtype {
                        "%" => self.module_map.borrow_mut().insert(
                            module_name.to_string(),
                            ModuleType::FlipFlop(FlipFlopModule {
                                name: module_name.to_string(),
                                pulse_queue: self.pulse_queue.clone(),
                                destinations: outputs.to_vec(),
                                is_on: false,
                            }),
                        ),
                        "&" => self.module_map.borrow_mut().insert(
                            module_name.to_string(),
                            ModuleType::Conjunction(ConjunctionModule {
                                name: module_name.to_string(),
                                pulse_queue: self.pulse_queue.clone(),
                                destinations: outputs.to_vec(),
                                input_states: HashMap::new(),
                            }),
                        ),
                        _ => panic!("Unknown module type: {}", mtype),
                    };
                }
            }
        }

        // configure Conjuncture modules: init all intitial states to LOW for all connected inputs:
        let mut module_dest_map: HashMap<String, Vec<String>> = HashMap::new();
        {
            let mm = self.module_map.borrow();
            for (name, module) in mm.iter() {
                let destinations;
                match module {
                    ModuleType::Conjunction(m) => {
                        destinations = m.destinations.clone();
                    }
                    ModuleType::FlipFlop(m) => {
                        destinations = m.destinations.clone();
                    }
                    ModuleType::Broadcast(m) => {
                        destinations = m.destinations.clone();
                    }
                    ModuleType::Output(_) => {
                        destinations = Vec::<String>::new();
                    }
                }
                module_dest_map.insert(name.to_string(), destinations.to_vec());
            }
        }

        for (name, destinations) in module_dest_map.iter() {
            for dest in destinations.iter() {
                let mut mm = self.module_map.borrow_mut();
                let dst = mm.get_mut(dest);
                if let None = dst {
                    panic!("Unknown destination module: {}", dest);
                }
                let dest_module = dst.unwrap();
                if let ModuleType::Conjunction(con_module) = dest_module {
                    con_module
                        .input_states
                        .insert(name.to_string(), PulseLevel::LOW);
                }
            }
        }
    }
}

impl Day for Day20 {
    fn day_nr(&self) -> String {
        String::from("20")
    }
    fn title(&self) -> String {
        String::from("Pulse Propagation")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day20.txt");
        // let input = read_lines("data/day20-test.txt");
        // let input = read_lines("data/day20-test2.txt");
        self.input = input;
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut solution: u64 = 0;
        // println!("Modules: {:?}", self.module_map);
        let mut p_high_count: u64 = 0;
        let mut p_low_count: u64 = 0;

        for i in 0..1000 {
            {
                let mut pq = self.pulse_queue.borrow_mut();
                pq.push_back(Pulse {
                    from: "button".to_string(),
                    level: PulseLevel::LOW,
                    module_name: "broadcaster".to_string(),
                });
            }
            let mut qcount: u64 = self.pulse_queue.borrow().len() as u64;
            while qcount > 0 {
                let pulse;
                {
                    pulse = self.pulse_queue.borrow_mut().pop_front().unwrap();
                }
                match pulse.level {
                    PulseLevel::HIGH => p_high_count += 1,
                    PulseLevel::LOW => p_low_count += 1,
                };
                let mut mmap = self.module_map.borrow_mut();
                let m = mmap.get_mut(pulse.module_name.as_str()).unwrap();
                match m {
                    ModuleType::FlipFlop(m) => {
                        m.send(&pulse);
                    }
                    ModuleType::Conjunction(m) => {
                        m.send(&pulse);
                    }
                    ModuleType::Broadcast(m) => {
                        m.send(&pulse);
                    }
                    ModuleType::Output(m) => {
                        m.send(&pulse);
                    }
                }
                qcount = self.pulse_queue.borrow().len() as u64;
                // if pcount > 10 {
                //     break;
                // }
            }
            // println!("\n")
        }

        println!("High Pulse Count: {0}", p_high_count);
        println!("Low Pulse Count: {0}", p_low_count);
        solution = p_high_count * p_low_count;

        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        // Reset by re-initializing things:
        self.parse_input();

        let mut solution: u64 = 0;
        // println!("Modules: {:?}", self.module_map);
        let mut button_presses: u64 = 0;

        'outer: loop {
            {
                let mut pq = self.pulse_queue.borrow_mut();
                pq.push_back(Pulse {
                    from: "button".to_string(),
                    level: PulseLevel::LOW,
                    module_name: "broadcaster".to_string(),
                });
                button_presses += 1;
            }
            let mut qcount: u64 = self.pulse_queue.borrow().len() as u64;
            while qcount > 0 {
                let pulse;
                {
                    pulse = self.pulse_queue.borrow_mut().pop_front().unwrap();
                }
                if let PulseLevel::LOW = pulse.level {
                    if pulse.module_name == "rx" {
                        println!("rx received LOW! Button press count: {}", button_presses);
                        break 'outer;
                    }
                }
                let mut mmap = self.module_map.borrow_mut();
                let m = mmap.get_mut(pulse.module_name.as_str()).unwrap();
                match m {
                    ModuleType::FlipFlop(m) => {
                        m.send(&pulse);
                    }
                    ModuleType::Conjunction(m) => {
                        m.send(&pulse);
                    }
                    ModuleType::Broadcast(m) => {
                        m.send(&pulse);
                    }
                    ModuleType::Output(m) => {
                        m.send(&pulse);
                    }
                }
                qcount = self.pulse_queue.borrow().len() as u64;
                // if pcount > 10 {
                //     break;
                // }
            }
            // println!("\n")
        }

        solution = button_presses;

        String::from(format!("{0}", solution))
    }
}
