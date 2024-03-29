use std::collections::{HashMap};
use crate::array_utils::{product_n, top_n};
use crate::days::day11::Operation::{PLUS, TIMES};
use crate::parser::read_unsigned_int;

const MAX_HAND_SIZE: usize = 24;
const AMOUNT_MONKEYS: usize = 8;
const CHASING_AMOUNT: usize = 2;

enum Term {
    OLD, NUMBER(usize)
}

impl Term {
    fn parse(buffer: &[u8], offset: &mut usize) -> Self {
        match buffer[*offset] as char {
            'o' => {
                *offset += 3;
                Term::OLD
            },
            _ => Term::NUMBER(read_unsigned_int(buffer, offset).unwrap() as usize),
        }
    }

    fn value(&self, old: usize) -> usize {
        match self {
            Term::OLD => old,
            Term::NUMBER(value) => *value,
        }
    }
}

enum Operation {
    PLUS(Term, Term), TIMES(Term, Term)
}

impl Operation {
    fn parse(buffer: &[u8], offset: &mut usize) -> Self {
        let first_term = Term::parse(buffer, offset);
        *offset += 1;
        let operator = buffer[*offset] as char;
        *offset += 2;
        let second_term = Term::parse(buffer, offset);
        match operator {
            '*' => TIMES(first_term, second_term),
            '+' => PLUS(first_term, second_term),
            _ => panic!("Unknown operator: {}", operator),
        }
    }

    fn apply(&self, old: usize) -> usize {
        match self {
            PLUS(a, b) => a.value(old) + b.value(old),
            TIMES(a, b) => a.value(old) * b.value(old),
        }
    }
}

struct Monkey {
    index: usize,
    inventory: [Option<usize>; MAX_HAND_SIZE],
    inventory_size: usize,
    operation: Operation,
    test: usize,
    test_outcome_true: usize,
    test_outcome_false: usize,
}

impl Default for Monkey {
    fn default() -> Self {
        Monkey {
            index: 0,
            inventory: [None; MAX_HAND_SIZE],
            inventory_size: 0,
            operation: PLUS(Term::OLD, Term::OLD),
            test: 0,
            test_outcome_true: 0,
            test_outcome_false: 0,
        }
    }
}

impl Monkey {
    fn parse(&mut self, buffer: &[u8], offset: &mut usize) {
        *offset += 7;
        read_unsigned_int(buffer, offset).expect("Was expecting the monkey index");
        *offset += 18;
        while buffer[*offset] != '\n' as u8 {
            *offset += 2;
            self.inventory[self.inventory_size] = Some(read_unsigned_int(buffer, offset).unwrap() as usize);
            self.inventory_size += 1;
        }
        *offset += 20;
        self.operation = Operation::parse(buffer, offset);
        *offset += 22;
        self.test = read_unsigned_int(buffer, offset).unwrap() as usize;
        *offset += 30;
        self.test_outcome_true = read_unsigned_int(buffer, offset).unwrap();
        *offset += 31;
        self.test_outcome_false = read_unsigned_int(buffer, offset).unwrap();
        *offset += 2;
    }

    fn consider_throw(&self, worry_level: usize) -> usize {
        match worry_level % self.test {
            0 => self.test_outcome_true,
            _ => self.test_outcome_false,
        }
    }
}

fn parse_monkeys(buffer: &[u8]) -> [Monkey; AMOUNT_MONKEYS] {
    let mut offset = 0;
    let mut monkeys: [Monkey; AMOUNT_MONKEYS] = Default::default();
    let mut monkey_index = 0;
    while offset < buffer.len() {
        monkeys[monkey_index].parse(buffer, &mut offset);
        monkeys[monkey_index].index = monkey_index;
        monkey_index += 1;
    }
    return monkeys;
}

fn simulate(monkeys: &mut [Monkey; AMOUNT_MONKEYS], rounds: usize, tensed: bool) -> [usize; AMOUNT_MONKEYS] {

    let mut modulus = 1;
    if tensed {
        for i in 0..monkeys.len() {
            modulus *= monkeys[i].test;
        }
    }

    let mut total_inspections = [0 as usize; AMOUNT_MONKEYS];
    for _ in 0..rounds {
        for monkey_index in 0..AMOUNT_MONKEYS {
            for considering_item in 0..monkeys[monkey_index].inventory_size {
                let mut worry_level = monkeys[monkey_index].inventory[considering_item].unwrap();
                worry_level = monkeys[monkey_index].operation.apply(worry_level);
                total_inspections[monkey_index] += 1;
                if tensed {
                    worry_level %= modulus;
                } else {
                    worry_level /= 3;
                }
                let throwing_to = monkeys[monkey_index].consider_throw(worry_level);
                monkeys[throwing_to].inventory[monkeys[throwing_to].inventory_size] = Some(worry_level);
                monkeys[throwing_to].inventory_size += 1;
            }
            monkeys[monkey_index].inventory_size = 0;
        }
    }

    return total_inspections;
}

fn simulate_alternative(monkeys: &mut [Monkey; AMOUNT_MONKEYS], rounds: usize, tensed: bool) -> [usize; AMOUNT_MONKEYS] {

    let mut modulus = 1;
    if tensed {
        for i in 0..monkeys.len() {
            modulus *= monkeys[i].test;
        }
    }

    let mut total_inspections = [0 as usize; AMOUNT_MONKEYS];
    for m_i in 0..AMOUNT_MONKEYS {
        for item in monkeys[m_i].inventory {
            if item.is_none() {
                break;
            }

            let mut monkey_index = monkeys[m_i].index;
            let mut worry_level = item.unwrap();
            for _ in 0..rounds {
                let mut prev_monkey_index = monkey_index;
                while prev_monkey_index <= monkey_index {
                    worry_level = monkeys[monkey_index].operation.apply(worry_level);
                    total_inspections[monkey_index] += 1;
                    if tensed {
                        worry_level %= modulus;
                    } else {
                        worry_level /= 3;
                    }
                    prev_monkey_index = monkey_index;
                    monkey_index = monkeys[monkey_index].consider_throw(worry_level)
                }
            }
        }
    }

    return total_inspections;
}

fn simulate_alternative_2(monkeys: &mut [Monkey; AMOUNT_MONKEYS], rounds: usize, tensed: bool) -> [usize; AMOUNT_MONKEYS] {

    let mut modulus = 1;
    if tensed {
        for i in 0..monkeys.len() {
            modulus *= monkeys[i].test;
        }
    }

    let mut visited = HashMap::new();
    let mut visits = Vec::new();
    let mut total_inspections = [0 as usize; AMOUNT_MONKEYS];

    for m_i in 0..AMOUNT_MONKEYS {
        'next_item:
        for item in monkeys[m_i].inventory {
            if item.is_none() {
                break;
            }

            visited.clear();
            visits.clear();

            let mut monkey_index = monkeys[m_i].index;
            let mut worry_level = item.unwrap();
            visited.insert((monkey_index, worry_level), 0);
            visits.push((monkey_index, 0));

            let mut round = 0;
            while round < rounds {
                let mut prev_monkey_index = monkey_index;
                let mut first = true;
                while prev_monkey_index <= monkey_index {
                    if first {
                        first = false;
                    } else {
                        visits.push((monkey_index, round));
                    }
                    worry_level = monkeys[monkey_index].operation.apply(worry_level);
                    if tensed {
                        worry_level %= modulus;
                    } else {
                        worry_level /= 3;
                    }
                    prev_monkey_index = monkey_index;
                    monkey_index = monkeys[monkey_index].consider_throw(worry_level);
                }
                round += 1;
                let prev_visit = visited.insert((monkey_index, worry_level), round);
                if prev_visit.is_some() {
                    let prev_visit_index = prev_visit.unwrap();
                    let circle_size = round - prev_visit_index;
                    let circling_rounds = rounds - prev_visit_index;
                    let amount_circles = circling_rounds / circle_size;
                    let remaining_rounds_after_whole_circles = circling_rounds % circle_size;
                    for (monkey_data, round_number) in &visits {
                        if *round_number < prev_visit_index {
                            total_inspections[*monkey_data] += 1;
                        } else {
                            let adjusted_index = (round_number - prev_visit_index) % circle_size;
                            total_inspections[*monkey_data] += amount_circles;
                            if adjusted_index < remaining_rounds_after_whole_circles {
                                total_inspections[*monkey_data] += 1;
                            }
                        }
                    }
                    continue 'next_item;
                } else {
                    visits.push((monkey_index, round));
                }
            }
        }
    }

    return total_inspections;
}

pub(crate) fn part1_old(buffer: &[u8]) -> String {
    let mut monkeys = parse_monkeys(buffer);
    let mut total_inspections = simulate(&mut monkeys, 20, false);
    top_n(&mut total_inspections, CHASING_AMOUNT);
    return product_n(&total_inspections, CHASING_AMOUNT).to_string()
}

pub(crate) fn part1(buffer: &[u8]) -> String {
    let mut monkeys = parse_monkeys(buffer);
    let mut total_inspections = simulate(&mut monkeys, 20, false);
    top_n(&mut total_inspections, CHASING_AMOUNT);
    return product_n(&total_inspections, CHASING_AMOUNT).to_string()
}

pub(crate) fn part2_oldest(buffer: &[u8]) -> String {
    let mut monkeys = parse_monkeys(buffer);
    let mut total_inspections = simulate_alternative(&mut monkeys, 10000, true);
    top_n(&mut total_inspections, CHASING_AMOUNT);
    return product_n(&total_inspections, CHASING_AMOUNT).to_string()
}

pub(crate) fn part2_old(buffer: &[u8]) -> String {
    let mut monkeys = parse_monkeys(buffer);
    let mut total_inspections = simulate_alternative(&mut monkeys, 10000, true);
    top_n(&mut total_inspections, CHASING_AMOUNT);
    return product_n(&total_inspections, CHASING_AMOUNT).to_string()
}

pub(crate) fn part2(buffer: &[u8]) -> String {
    let mut monkeys = parse_monkeys(buffer);
    let mut total_inspections = simulate_alternative_2(&mut monkeys, 10000, true);
    top_n(&mut total_inspections, CHASING_AMOUNT);
    return product_n(&total_inspections, CHASING_AMOUNT).to_string()
}