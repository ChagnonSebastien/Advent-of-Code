use crate::utils::Part;
use regex::Regex;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;
use chrono::Utc;

#[derive(Copy, Clone)]
enum Element {
  Thulium, Plutonium, Strontium, Promethium, Ruthenium
}

impl Element {
  fn from_string(name: &str) -> Element {
    match name {
      "thulium" => Element::Thulium,
      "plutonium" => Element::Plutonium,
      "strontium" => Element::Strontium,
      "promethium" => Element::Promethium,
      "ruthenium" => Element::Ruthenium,
      _ => panic!("Invalid Element Name: {}", name),
    }
  }
}

impl Display for Element {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", match self {
      Element::Thulium => "Thulium",
      Element::Plutonium => "Plutonium",
      Element::Strontium => "Strontium",
      Element::Promethium => "Promethium",
      Element::Ruthenium => "Ruthenium",
    })
  }
}

#[derive(Copy, Clone)]
enum Equipment {
  Microchip(Element), Generator(Element)
}

#[derive(Copy, Clone)]
struct Floor {
  generators: [bool; 5],
  microships: [bool; 5],
}

impl Display for Floor {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let mut floor_representation = String::new();
    for i in 0..5 {
      let potential_microship = format!(" M.{} ", i);
      let potential_generator = format!(" G.{} ", i);
      floor_representation.push_str(match self.microships[i] { true => potential_microship.as_str(), false => "     " });
      floor_representation.push_str(match self.generators[i] { true => potential_generator.as_str(), false => "     " });
    }
    write!(f, "{}", floor_representation)
  }
}

impl Floor {
  fn is_valid(&self) -> bool {
    !self.has_unpluged_microchip() || !self.has_at_least_one_generator()
  }

  fn has_unpluged_microchip(&self) -> bool {
    for i in 0..5 {
      if self.microships[i] && !self.generators[i] {
        return true;
      }
    }
    false
  }

  fn generator_amount(&self) -> usize {
    self.generators.iter().fold(0, |sum, g| sum + match g { true => 1, false => 0 })
  }

  fn microship_amount(&self) -> usize {
    self.microships.iter().fold(0, |sum, g| sum + match g { true => 1, false => 0 })
  }

  fn amount_equipment(&self) -> usize {
    self.generator_amount() + self.microship_amount()
  }

  fn has_at_least_one_generator(&self) -> bool {
    self.generator_amount() > 0
  }

  fn install_equipment(&mut self, equipment: Equipment) {
    match equipment {
      Equipment::Generator(element) => self.generators[Self::element_index(element)] = true,
      Equipment::Microchip(element) => self.microships[Self::element_index(element)] = true,
    }
  }

  fn element_index(element: Element) -> usize {
    match element {
      Element::Thulium => 0,
      Element::Plutonium => 1,
      Element::Strontium => 2,
      Element::Promethium => 3,
      Element::Ruthenium => 4,
    }
  }

  fn empty() -> Self {
    Floor {
      generators: [false; 5],
      microships: [false; 5],
    }
  }
}

impl PartialEq for Floor {
    fn eq(&self, other: &Self) -> bool {
        self.generators == other.generators && self.microships == other.microships
    }
}
impl Eq for Floor {}

#[derive(Copy, Clone)]
struct Building {
  floors: [Floor; 4],
  moves: usize,
  elevator: usize,
}

impl Display for Building {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {

    let mut building_representation = String::from("=======================================================\n");
    for i in 0..4 {
      building_representation.push_str(format!("|{} {} |\n", match 3-i == self.elevator {
        true => "E",
        false => " ",
      }, self.floors[3-i]).as_str());
    }
    building_representation.push_str("=======================================================");
    write!(f, "{}", building_representation)
  }
}

impl Building {
  fn possible_moves(&self) -> Vec<Self> {
    let mut available_equipment: Vec<(bool, usize)> = Vec::new();
    for i in 0..5 {
      if self.floors[self.elevator].generators[i] {
        available_equipment.push((false, i));
      }
      if self.floors[self.elevator].microships[i] {
        available_equipment.push((true, i));
      }
    }

    let mut possibilities: Vec<Self> = Vec::new();
    let prev_floor = self.elevator;
    for next_floor in self.next_floors() {
      for i in 0..available_equipment.len() {
        let mut simple_neighbor = *self;
        simple_neighbor.moves += 1;
        simple_neighbor.elevator = next_floor;

        let simple = available_equipment[i];
        match simple.0 {
          true => {
            simple_neighbor.floors[prev_floor].microships[simple.1] = false;
            simple_neighbor.floors[next_floor].microships[simple.1] = true;
          },
          false => {
            simple_neighbor.floors[prev_floor].generators[simple.1] = false;
            simple_neighbor.floors[next_floor].generators[simple.1] = true;
          },
        }
        if simple_neighbor.is_valid() {
          possibilities.push(simple_neighbor);
        }

        for j in (i + 1)..available_equipment.len() {
          let mut double_neighbor = simple_neighbor;
          let double = available_equipment[j];
          match double.0 {
            true => {
              double_neighbor.floors[prev_floor].microships[double.1] = false;
              double_neighbor.floors[next_floor].microships[double.1] = true;
            },
            false => {
              double_neighbor.floors[prev_floor].generators[double.1] = false;
              double_neighbor.floors[next_floor].generators[double.1] = true;
            },
          }
          if double_neighbor.is_valid() {
            possibilities.push(double_neighbor);
          }
        }
      }
    }
    possibilities
  }

  fn is_valid(&self) -> bool {
    for floor in self.floors.iter() {
      if !floor.is_valid() {
        return false;
      }
    }
    true
  }

  fn install_equipment(&mut self, equipment: Equipment, floor_number: usize) {
    self.floors[floor_number].install_equipment(equipment);
  }

  fn cost(&self) -> usize {
    let mut cost = self.moves * 5;
    for i in 0..4 {
      cost += self.floors[i].amount_equipment() * (3-i);
    }
    cost
  }

  fn next_floors(&self) -> Vec<usize> {
    let mut next = Vec::new();
    if self.elevator < 3 {
      next.push(self.elevator + 1);
    }
    if self.elevator > 0 {
      next.push(self.elevator - 1);
    }
    next
  }

  fn has_same_floors(&self, other: &Self) -> bool {
    self.floors == other.floors && self.elevator == other.elevator
  }

  fn is_solution(&self) -> bool {
    self.floors[3].amount_equipment() == 10
  }

  fn empty() -> Self {
    Building {
      floors: [Floor::empty(), Floor::empty(), Floor::empty(), Floor::empty()],
      moves: 0,
      elevator: 0,
    }
  }
}

impl Ord for Building {
  fn cmp(&self, other: &Self) -> Ordering {
    other.cost().cmp(&self.cost())
  }
}

impl PartialOrd for Building {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Building {
    fn eq(&self, other: &Self) -> bool {
        self.cost() == other.cost()
    }
}
impl Eq for Building {}

fn part_one(input: String) {
  let mut initial_building = Building::empty();
  let floor_contents = input.split("\n").collect::<Vec<&str>>();

  let re_generator = Regex::new(r"[a-z]+ generator").unwrap();
  for i in 0..floor_contents.len() {
    for element_match in re_generator.find_iter(floor_contents[i]) {
      let element_name = element_match.as_str().split(" ").next().unwrap();
      let element = Element::from_string(element_name);
      initial_building.install_equipment(Equipment::Generator(element), i);
    }
  }

  let re_microship = Regex::new(r"[a-z]+-compatible microchip").unwrap();
  for i in 0..floor_contents.len() {
    for element_match in re_microship.find_iter(floor_contents[i]) {
      let element_name = element_match.as_str().split("-").next().unwrap();
      let element = Element::from_string(element_name);
      initial_building.install_equipment(Equipment::Microchip(element), i);
    }
  }

  let mut visited: Vec<Building> = Vec::new();
  let mut solutions: BinaryHeap<Building> = BinaryHeap::new();
  solutions.push(initial_building);

  while {
    let possibility = solutions.pop().expect("No possibilities left");
    match possibility.is_solution() {
      true => {
        println!("Found Solution! Moves: {}", possibility.moves);
        false
      },
      false => {
        visited.push(possibility);
        for next_possibility in possibility.possible_moves() {
          if solutions.iter().find_map(|s| match s.has_same_floors(&next_possibility) {
            true => Some(s),
            false => None
          }).is_none() && visited.iter().find_map(|s| match s.has_same_floors(&next_possibility) {
            true => Some(s),
            false => None
          }).is_none(){
            solutions.push(next_possibility);
          }
        }
        true
      },
    }
  } {}
}

const AMOUNT_MATERIALS: u64 = 7;

#[derive(PartialEq)]
enum Operation { Plus, Minus }

#[derive(Copy, Clone)]
struct BitwiseBuilding {
  state: u64,
  moves: u64,
}

impl BitwiseBuilding {
  fn elevator(&self) -> u64 {
    self.state >> 56
  }

  fn set_elevator(&mut self, f: u64) {
    self.state &= 0x00FFFFFFFFFFFFFF;
    self.state |= f << 56;
  }

  fn floor(&self, f: u64) -> u64 {
    self.state >> (AMOUNT_MATERIALS * 2) * f & 0b1111111_1111111
  }

  fn cost(&self) -> u64 {
    let mut c = self.moves;
    for i in 0..3 {
      let mut f = self.floor(i);
      let mut t = 0;
      while f != 0 {
        t += f & 1;
        f >>= 1;
      }
      c += t * (3-i);
    }
    c
  }

  fn is_valid(&self) -> bool {
    for i in 0..4 {
      let floor = self.floor(i);
      let generators = floor >> AMOUNT_MATERIALS;
      if generators == 0 {
        continue
      }

      let microchips = floor & 0b1111111;
      let unassigned_microships = !generators & microchips;
      if unassigned_microships != 0 {
        return false;
      }
    }
    true
  }

  fn move_item(&mut self, from_floor: u64, to_floor: u64, index: u64) {
    let from_index = 1 << (from_floor * AMOUNT_MATERIALS * 2 + index);
    let to_index = 1 << (to_floor * AMOUNT_MATERIALS * 2 + index);
    self.state &= !from_index;
    self.state |= to_index;
  }

  fn neighbors(&self) -> Vec<Self> {
    let current_floor = self.elevator();
    let floor = self.floor(current_floor);
    let mut available_equipment: Vec<u64> = Vec::new();
    for i in 0..AMOUNT_MATERIALS * 2 {
      if floor >> i & 1 == 1 {
        available_equipment.push(i);
      }
    }

    let mut possibilities: Vec<Self> = Vec::new();
    for offset in &[Operation::Minus, Operation::Plus] {
      if (*offset == Operation::Minus && current_floor == 0) || (*offset == Operation::Plus && current_floor == 3) { continue }
      let next_floor = match offset { Operation::Minus => current_floor - 1, Operation::Plus => current_floor + 1 };

      for i in 0..available_equipment.len() {
        let mut simple_neighbor = *self;
        simple_neighbor.moves += 1;
        simple_neighbor.set_elevator(next_floor);
        simple_neighbor.move_item(current_floor, next_floor, available_equipment[i]);
        if simple_neighbor.is_valid() {
          possibilities.push(simple_neighbor);
        }

        for j in (i + 1)..available_equipment.len() {
          let mut double_neighbor = simple_neighbor;
          double_neighbor.move_item(current_floor, next_floor, available_equipment[j]);
          if double_neighbor.is_valid() {
            possibilities.push(double_neighbor);
          }
        }
      }
    }

    possibilities
  }

  fn is_goal(&self) -> bool {
    // Elevator and all components on third floor
    self.state == 0b00000011_1111111_1111111_0000000_0000000_0000000_0000000_0000000_0000000
  }
}

impl Ord for BitwiseBuilding {
  fn cmp(&self, other: &Self) -> Ordering {
    other.cost().cmp(&self.cost())
  }
}

impl PartialOrd for BitwiseBuilding {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BitwiseBuilding {
    fn eq(&self, other: &Self) -> bool {
        self.cost() == other.cost()
    }
}
impl Eq for BitwiseBuilding {}

fn part_two() {
  let mut building = BitwiseBuilding {
    //       Elevator F4.Gen  F4.Mic  F3.Gen  F3.Mic  F2.Gen  F2.Mic  F1.Gen  F1.Mic
    state: 0b00000000_0000000_0000000_0000011_0000011_0000000_0001100_1111100_1110000,
    moves: 0,
  };

  let mut possibilities: BinaryHeap<BitwiseBuilding> = BinaryHeap::new();
  let mut visited: HashSet<u64> = HashSet::new();
  visited.insert(building.state);

  while !building.is_goal() {
    for possibility in building.neighbors() {
      if visited.insert(possibility.state) {
        possibilities.push(possibility);
      }
    }

    building = possibilities.pop().expect("No more possibilities");
  }

  println!("Found solution in {} moves.", building.moves);
}

pub fn execute(input: String, part: &Part) {
  let start = Utc::now().time();
  match part {
    Part::PartOne => part_one(input),
    Part::PartTwo => part_two(),
  }
  let end = Utc::now().time();
  println!("Took {} seconds.", (end-start).num_seconds());
}
