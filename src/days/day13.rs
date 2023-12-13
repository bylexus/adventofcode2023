use super::Day;
use alex_lib::read_lines;
use itertools::Itertools;

#[derive(Debug)]
pub struct Day13 {
    input: Vec<String>,
    fields: Vec<Vec<Vec<char>>>,
    field_results_part1: Vec<(i64, i64)>,
}

impl Day13 {
    pub fn new() -> Day13 {
        Day13 {
            input: Vec::new(),
            fields: Vec::new(),
            field_results_part1: Vec::new(),
        }
    }

    fn parse_input(&mut self) {
        let mut act_field = Vec::new();
        for line in &self.input {
            if line.trim().is_empty() {
                if act_field.len() > 0 {
                    self.fields.push(act_field);
                }
                act_field = Vec::new();
            } else {
                act_field.push(line.chars().collect_vec());
            }
        }
        if act_field.len() > 0 {
            self.fields.push(act_field);
        }
    }

    fn find_mirror_lines(&self, field: &Vec<Vec<char>>, prev_res: (i64, i64)) -> (i64, i64) {
        (
            self.find_horizontal_mirror_line(field, prev_res.0),
            self.find_vertical_mirror_line(field, prev_res.1),
        )
    }

    fn find_vertical_mirror_line(&self, field: &Vec<Vec<char>>, ignore_col: i64) -> i64 {
        // test all cols if it is a mirror col:
        // c is the actual 'right border' of the mirror col
        for c in 1..field[0].len() {
            if ignore_col > -1 && c as i64 == ignore_col {
                continue;
            }
            if self.test_vertical_symmetry(field, c as i64) {
                return c as i64;
            }
        }

        -1
    }

    fn test_vertical_symmetry(&self, field: &Vec<Vec<char>>, mirror_col: i64) -> bool {
        let mut left_col = mirror_col - 1;
        let mut right_col = mirror_col;
        // test symmetry on all lines
        while left_col >= 0 && right_col < field[0].len() as i64 {
            for line_idx in 0..field.len() {
                if field[line_idx][left_col as usize] != field[line_idx][right_col as usize] {
                    return false;
                }
            }
            left_col -= 1;
            right_col += 1;
        }
        true
    }

    fn find_horizontal_mirror_line(&self, field: &Vec<Vec<char>>, ignore_row: i64) -> i64 {
        // test all rows if it is a mirror row:
        // r is the actual 'bottom border' of the mirror row
        for r in 1..field.len() {
            if ignore_row > -1 && r as i64 == ignore_row {
                continue;
            }
            if self.test_horizontal_symmetry(field, r as i64) {
                return r as i64;
            }
        }

        -1
    }

    fn test_horizontal_symmetry(&self, field: &Vec<Vec<char>>, mirror_row: i64) -> bool {
        let mut up_row = mirror_row - 1;
        let mut down_row = mirror_row;
        // test symmetry on all lines
        while up_row >= 0 && down_row < field.len() as i64 {
            for col_idx in 0..field[0].len() {
                if field[up_row as usize][col_idx] != field[down_row as usize][col_idx] {
                    return false;
                }
            }
            up_row -= 1;
            down_row += 1;
        }
        true
    }
}

impl Day for Day13 {
    fn day_nr(&self) -> String {
        String::from("13")
    }
    fn title(&self) -> String {
        String::from("Point of Incidence")
    }

    fn prepare(&mut self) {
        let input = read_lines("data/day13.txt");
        // let input = read_lines("data/day13-test.txt");
        self.input = input;
        self.parse_input();
    }

    fn solve1(&mut self) -> String {
        let mut solution: i64 = 0;
        for field in self.fields.iter() {
            let (horiz_line, vert_line) = self.find_mirror_lines(field, (-1, -1));
            self.field_results_part1.push((horiz_line, vert_line));
            solution +=
                100 * match horiz_line {
                    -1 => 0,
                    h => h,
                } + match vert_line {
                    -1 => 0,
                    v => v,
                };
        }
        String::from(format!("{0}", solution))
    }

    fn solve2(&mut self) -> String {
        let mut solution: i64 = 0;
        for (field_idx, field) in self.fields.iter().enumerate() {
            // modify loop: modify each cell in the field, and check for the new mirror line:
            // This is a brute-force-approach, which seems to work just fine:
            'field_break: for line_idx in 0..field.len() {
                for col_idx in 0..field[0].len() {
                    // modify a single cell, calc the new solution, check it:
                    let mut new_field = field.clone();
                    let val = new_field[line_idx][col_idx];
                    new_field[line_idx][col_idx] = match val {
                        '.' => '#',
                        '#' => '.',
                        _ => panic!("Unexpected value: {}", val),
                    };

                    let prev_res = self.field_results_part1[field_idx];
                    let (horiz_line, vert_line) = self.find_mirror_lines(&new_field, prev_res);

                    // no solution for this mutation, continue with next mutation
                    if horiz_line == -1 && vert_line == -1 {
                        continue;
                    }

                    // if we have the same result as in part 1, skip: we need a new solution
                    if prev_res == (horiz_line, vert_line) {
                        continue;
                    }

                    // finally, we found a new solution
                    solution +=
                        100 * match horiz_line {
                            -1 => 0,
                            h => h,
                        } + match vert_line {
                            -1 => 0,
                            v => v,
                        };
                    // as soon as we found a solution, we skip the rest of the field
                    break 'field_break;
                }
            }
        }
        String::from(format!("{0}", solution))
    }
}
