use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::Day;
use alex_lib::read_lines;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Eq)]
struct Wire {
    cmp1: String,
    cmp2: String,
    active: bool,
}

impl PartialEq for Wire {
    fn eq(&self, other: &Self) -> bool {
        self.cmp1 == other.cmp1 && self.cmp2 == other.cmp2
    }
}

#[derive(Debug)]
struct Component {
    name: String,
    visited: bool,
    wires: HashMap<(String, String), Rc<RefCell<Wire>>>,
    graph_nr: u64,
}

#[derive(Debug)]
pub struct Day25 {
    input: Vec<String>,
    wires: HashMap<(String, String), Rc<RefCell<Wire>>>,
    components: HashMap<String, Rc<RefCell<Component>>>,
}

impl Day25 {
    pub fn new() -> Day25 {
        Day25 {
            input: Vec::new(),
            wires: HashMap::new(),
            components: HashMap::new(),
        }
    }

    fn parse_input(&mut self) {
        let matcher = Regex::new(r"(\w+):\s+(.*)").unwrap();
        for line in self.input.iter() {
            if let Some(caps) = matcher.captures(line) {
                let cmp_name = caps[1].to_string();
                let targets: Vec<String> =
                    caps[2].split(' ').map(|s| s.trim().to_string()).collect();
                let cmp = Component {
                    name: cmp_name.clone(),
                    visited: false,
                    wires: HashMap::new(),
                    graph_nr: 0,
                };
                self.components
                    .insert(cmp_name.clone(), Rc::new(RefCell::new(cmp)));
                for target in targets {
                    // The cmps in the wire are ordered by alphabet:
                    // 1st wire is always the one first in the alphabet, this makes
                    // recognizing easier
                    if let None = self.components.get(&target) {
                        self.components.insert(
                            target.to_string(),
                            Rc::new(RefCell::new(Component {
                                name: target.to_string(),
                                visited: false,
                                wires: HashMap::new(),
                                graph_nr: 0,
                            })),
                        );
                    }
                    let wire = Wire {
                        cmp1: match cmp_name < target {
                            true => cmp_name.clone(),
                            false => target.clone(),
                        },
                        cmp2: match cmp_name < target {
                            true => target.clone(),
                            false => cmp_name.clone(),
                        },
                        active: true,
                    };
                    self.wires.insert(
                        (wire.cmp1.clone(), wire.cmp2.clone()),
                        Rc::new(RefCell::new(wire)),
                    );
                }
            }
        }

        // now, add all wires to their corresponding nodes:
        for (key, wire) in self.wires.iter() {
            self.components
                .get(wire.as_ref().borrow().cmp1.as_str())
                .unwrap()
                .as_ref()
                .borrow_mut()
                .wires
                .insert(key.clone(), Rc::clone(wire));
            self.components
                .get(wire.as_ref().borrow().cmp2.as_str())
                .unwrap()
                .as_ref()
                .borrow_mut()
                .wires
                .insert(key.clone(), Rc::clone(wire));
        }
    }

    fn reset(&self) {
        for cmp in self.components.values() {
            cmp.as_ref().borrow_mut().visited = false;
            cmp.as_ref().borrow_mut().graph_nr = 0;
        }
        for wire in self.wires.values() {
            wire.as_ref().borrow_mut().active = true;
        }
    }

    fn find_unvisited_cmp(&self) -> Option<Rc<RefCell<Component>>> {
        for cmp in self.components.values() {
            if !cmp.as_ref().borrow().visited {
                return Some(Rc::clone(cmp));
            }
        }
        None
    }

    // Walks a graph by starting at the given node, visiting all the nodes that are
    // not visited and reachable from this node.
    // Returns the number of visited nodes.
    fn walk_graph(&self, cmp: Rc<RefCell<Component>>) -> u64 {
        let mut count = 1;
        cmp.as_ref().borrow_mut().visited = true;
        for wire in cmp.as_ref().borrow().wires.values() {
            if wire.as_ref().borrow().active {
                let cmp1 = self
                    .components
                    .get(wire.as_ref().borrow().cmp1.as_str())
                    .unwrap();
                let cmp2 = self
                    .components
                    .get(wire.as_ref().borrow().cmp2.as_str())
                    .unwrap();
                if cmp1.as_ref().borrow().visited == false {
                    count += self.walk_graph(Rc::clone(cmp1));
                }
                if cmp2.as_ref().borrow().visited == false {
                    count += self.walk_graph(Rc::clone(cmp2));
                }
            }
        }
        count
    }

    // Returns the number of graphs in the graph,
    // returning a list of the number of nodes in each graph,
    // so the length of the Vec is the number of graphs,
    // while the value at each index is the number of nodes in that graph
    fn count_graphs(&self) -> Vec<u64> {
        let mut graph_counts = Vec::new();
        let mut cmp;

        loop {
            // We take a random unvisited node, and walk the graph from there.
            // Every time we do this, we count another graph.
            // When there are no more unvisited nodes, we are done, and know the number
            // of graphs.
            cmp = self.find_unvisited_cmp();
            if cmp.is_none() {
                break;
            }
            if graph_counts.len() == 2 {
                // we already have the target number of graphs, so this is not it:
                graph_counts.clear();
                break;
            }
            graph_counts.push(self.walk_graph(Rc::clone(&cmp.unwrap())));
            // println!("Graph nr: {}", graph_counts.len());
        }

        graph_counts
    }

    fn print_wires(&self) {
        println!("// Graphviz Graph - Print it with sfdp, undirected large graph:");
        println!("graph G {{");
        for wire in self.wires.values() {
            println!(
                "{0} -- {1};",
                wire.as_ref().borrow().cmp1,
                wire.as_ref().borrow().cmp2
            );
        }
        println!("}}");
    }
}

impl Day for Day25 {
    fn day_nr(&self) -> String {
        String::from("25")
    }
    fn title(&self) -> String {
        String::from("Snowverload")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day25.txt");
        // let input = read_lines("data/day25-test.txt");
        self.input = input;
        self.parse_input();
    }

    /// For part 1, I just drawn the graph, and solved it visually:
    /// I printed the wires with print_wires() as unidirected graphviz graph using sfdp,
    /// then just looked at the image.
    /// 
    /// Calculating take way toooooo long!
    /// Note that my solution only works with my input!
    fn solve1(&mut self) -> String {
        let mut solution: u64 = 0;
        self.print_wires();

        // Visually identified wires from real set:
        self.reset();
        self.wires
            .get(&("jff".to_string(), "zns".to_string()))
            .unwrap()
            .as_ref()
            .borrow_mut()
            .active = false;
        self.wires
            .get(&("fts".to_string(), "nvb".to_string()))
            .unwrap()
            .as_ref()
            .borrow_mut()
            .active = false;
        self.wires
            .get(&("kzx".to_string(), "qmr".to_string()))
            .unwrap()
            .as_ref()
            .borrow_mut()
            .active = false;

        // Demo set:
        // Test: disconnect 3 fixed wires from the example:
        // self.wires
        //     .get(&("hfx".to_string(), "pzl".to_string()))
        //     .unwrap()
        //     .as_ref()
        //     .borrow_mut()
        //     .active = false;
        // self.wires
        //     .get(&("bvb".to_string(), "cmg".to_string()))
        //     .unwrap()
        //     .as_ref()
        //     .borrow_mut()
        //     .active = false;
        // self.wires
        //     .get(&("jqt".to_string(), "nvd".to_string()))
        //     .unwrap()
        //     .as_ref()
        //     .borrow_mut()
        //     .active = false;
        // now, start walking the graphs:
        let graph_counts = self.count_graphs();
        let nr_of_graphs = graph_counts.len();
        solution = graph_counts.iter().product::<u64>();

        println!("Nr of graphs: {0}", nr_of_graphs);
        println!("Graph sizes: {:?}", graph_counts);

        /* 
        // Disconnect 3 wires, reset, walk, and count:
        // If we have only 2 graphs, we found our solution:
        // This solution takes too long!
        let wires = self.wires.values().collect_vec();
        'outer: for i1 in 0..wires.len() - 2 {
            for i2 in i1 + 1..wires.len() - 1 {
                for i3 in i2 + 1..wires.len() {
                    // println!("Testing: {0}, {1}, {2}", i1, i2, i3);
                    self.reset();
                    wires[i1].as_ref().borrow_mut().active = false;
                    wires[i2].as_ref().borrow_mut().active = false;
                    wires[i3].as_ref().borrow_mut().active = false;
                    let graph_counts = self.count_graphs();
                    let nr_of_graphs = graph_counts.len();
                    if nr_of_graphs == 2 {
                        solution = graph_counts.iter().product::<u64>();
                        break 'outer;
                    }
                }
            }
        }
        */


        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let mut solution: u64 = 0;
        String::from(format!("{0}", solution))
    }
}
