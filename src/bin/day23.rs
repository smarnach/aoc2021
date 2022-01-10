#![feature(array_zip, io_read_to_string)]

use anyhow::{Context, Error, Result};
use arrayvec::ArrayVec;
use regex::Regex;
use rustc_hash::FxHashSet as HashSet;
use std::{
    collections::BinaryHeap,
    io::{read_to_string, stdin},
    rc::Rc,
};

fn main() -> Result<()> {
    let input_a = parse_input(&read_to_string(&mut stdin())?)?;
    let input_b = unfold(input_a);
    println!("{}", solve(Burrow::new(input_a))?.cost);
    println!("{}", solve(Burrow::new(input_b))?.cost);
    Ok(())
}

fn unfold(input: [[Amphipod; 2]; 4]) -> [[Amphipod; 4]; 4] {
    use Amphipod::*;
    let insert = [[D, D], [B, C], [A, B], [C, A]];
    input.zip(insert).map(|(r, s)| [r[0], s[0], s[1], r[1]])
}

fn solve<const N: usize>(burrow: Burrow<N>) -> Result<Rc<Node<N>>> {
    let mut queue = BinaryHeap::new();
    queue.push(Node::new(burrow));
    let mut seen = HashSet::default();
    while let Some(node) = queue.pop() {
        if node.burrow.solved() {
            return Ok(node);
        }
        if !seen.insert(node.burrow.clone()) {
            continue;
        }
        for (additional_cost, new_burrow) in node.burrow.moves() {
            queue.push(node.add(additional_cost, new_burrow));
        }
    }
    Err(Error::msg("no solution found"))
}

#[derive(Eq)]
struct Node<const N: usize> {
    cost: i32,
    burrow: Burrow<N>,
    #[allow(dead_code)]
    previous: Option<Rc<Node<N>>>,
}

impl<const N: usize> Node<N> {
    fn new(burrow: Burrow<N>) -> Rc<Self> {
        Rc::new(Self {
            cost: burrow.min_cost(),
            burrow,
            previous: None,
        })
    }

    fn add(self: &Rc<Self>, additional_cost: i32, burrow: Burrow<N>) -> Rc<Self> {
        Rc::new(Self {
            cost: self.cost + additional_cost,
            burrow,
            previous: Some(Rc::clone(self)),
        })
    }
}

impl<const N: usize> PartialEq for Node<N> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<const N: usize> PartialOrd for Node<N> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for Node<N> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Burrow<const N: usize> {
    rooms: [ArrayVec<Amphipod, N>; 4],
    cells: [Option<Amphipod>; 7],
}

impl<const N: usize> Burrow<N> {
    fn new(rooms: [[Amphipod; N]; 4]) -> Self {
        let rooms = rooms.map(|r| r.into_iter().collect::<ArrayVec<_, N>>());
        Self {
            rooms,
            cells: [None; 7],
        }
    }

    fn traversable(&self, x0: i32, x1: i32) -> bool {
        geom::corridor_range(x0, x1).all(|i| self.cells[i].is_none())
    }

    fn dest_y(&self, i: usize) -> usize {
        N - self.rooms[i]
            .iter()
            .take_while(|&&a| a as usize == i)
            .count()
    }

    #[inline]
    fn solved(&self) -> bool {
        (0..4).all(|i| self.dest_y(i) == 0)
    }

    fn min_cost(&self) -> i32 {
        let mut cost = (0..4)
            .map(|i| {
                let y = self.dest_y(i) as i32;
                Amphipod::try_from(i as u8).unwrap().cost(y * (y + 1) / 2)
            })
            .sum::<i32>()
            + self
                .cells
                .iter()
                .enumerate()
                .filter_map(|(i, &a)| a.map(|a| a.cost(geom::dist_corridor_room(i, a as _, 0))))
                .sum::<i32>();
        for (i, r) in self.rooms.iter().enumerate() {
            for (k, &a) in r.iter().enumerate().skip_while(|(_, &a)| a as usize == i) {
                cost += a.cost(geom::dist_room_room(i, N - k, a as _, 0));
            }
        }
        cost
    }

    fn corridor_to_room(&mut self) -> bool {
        let mut moved = false;
        for corr_i in 0..7 {
            if let Some(a) = self.cells[corr_i] {
                let room_i = a as usize;
                let dest_y = self.dest_y(room_i);
                if dest_y + self.rooms[room_i].len() == N
                    && self.traversable(geom::corridor_x(corr_i), geom::room_x(room_i))
                {
                    self.cells[corr_i].take();
                    self.rooms[room_i].push(a);
                    moved = true;
                }
            }
        }
        moved
    }

    fn room_to_room(&mut self) -> bool {
        let mut moved = false;
        for i in 0..4 {
            if let Some(&a) = self.rooms[i].last() {
                let j = a as usize;
                let dest_y = self.dest_y(j);
                if i != j
                    && dest_y + self.rooms[j].len() == N
                    && self.traversable(geom::room_x(i), geom::room_x(j))
                {
                    self.rooms[i].pop();
                    self.rooms[j].push(a);
                    moved = true;
                }
            }
        }
        moved
    }

    fn moves(&self) -> Moves<'_, N> {
        Moves {
            burrow: self,
            room_i: 0,
            corr_i: 0,
        }
    }
}

type Move<const N: usize> = (i32, Burrow<N>);

struct Moves<'a, const N: usize> {
    burrow: &'a Burrow<N>,
    room_i: usize,
    corr_i: usize,
}

impl<'a, const N: usize> Iterator for Moves<'a, N> {
    type Item = Move<N>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.room_i < 4 {
            let room_i = self.room_i;
            if self.burrow.dest_y(room_i) + self.burrow.rooms[room_i].len() != N {
                while self.corr_i < 7 {
                    let corr_i = self.corr_i;
                    self.corr_i += 1;
                    let a = *self.burrow.rooms[room_i].last().unwrap();
                    let room_x = geom::room_x(room_i);
                    let corr_x = geom::corridor_x(corr_i);
                    let dest_x = geom::room_x(a as _);
                    let detour =
                        (corr_x - room_x).abs() + (dest_x - corr_x).abs() - (dest_x - room_x).abs();
                    if self.burrow.cells[corr_i].is_none()
                        && detour > 0
                        && self.burrow.traversable(room_x, corr_x)
                    {
                        let mut new_burrow = self.burrow.clone();
                        new_burrow.rooms[room_i].pop();
                        new_burrow.cells[corr_i] = Some(a);
                        while new_burrow.corridor_to_room() || new_burrow.room_to_room() {}
                        return Some((a.cost(detour), new_burrow));
                    }
                }
            }
            self.corr_i = 0;
            self.room_i += 1;
        }
        None
    }
}

mod geom {
    // x       0 1 2 3 4 5 6 7 8 9 10    y
    //        ########################
    // corr_i #0 1   2   3   4   5 6 #   0
    //        #####  ##  ##  ##  #####   1
    //            #  ##  ##  ##  #       ⋮
    //            #  ##  ##  ##  #       N
    //            ################
    // room_i      0   1   2   3

    use std::{
        cmp::{max, min},
        ops::Range,
    };

    pub fn corridor_x(corr_i: usize) -> i32 {
        match corr_i {
            0 => 0,
            1..=5 => 2 * corr_i as i32 - 1,
            6 => 10,
            _ => panic!("invalid corridor index"),
        }
    }

    pub fn room_x(room_i: usize) -> i32 {
        2 * room_i as i32 + 2
    }

    pub fn corridor_range(x0: i32, x1: i32) -> Range<usize> {
        let (x0, x1) = (min(x0, x1), max(x0, x1));
        ((x0 + 3) / 2) as usize..((x1 + 2) / 2) as usize
    }

    pub fn dist_corridor_room(corr_i: usize, room_i: usize, y: usize) -> i32 {
        (corridor_x(corr_i) - room_x(room_i)).abs() + y as i32
    }

    pub fn dist_room_room(room_i0: usize, y0: usize, room_i1: usize, y1: usize) -> i32 {
        2 * (room_i0 as i32 - room_i1 as i32).abs() + (y0 + y1) as i32
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn cost(&self, dist: i32) -> i32 {
        dist * 10i32.pow(*self as _)
    }
}

impl TryFrom<u8> for Amphipod {
    type Error = Error;

    fn try_from(i: u8) -> Result<Self, Self::Error> {
        match i {
            0..=3 => unsafe { Ok(std::mem::transmute(i)) },
            _ => Err(Error::msg("invalid amphipod type")),
        }
    }
}

impl TryFrom<char> for Amphipod {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A'..='D' => Self::try_from(c as u8 - b'A'),
            _ => Err(Error::msg("invalid amphipod type")),
        }
    }
}

fn parse_input(input: &str) -> Result<[[Amphipod; 2]; 4]> {
    const PATTERN: &str = r"(?m)\A#############
#\.\.\.\.\.\.\.\.\.\.\.#
###(.)#(.)#(.)#(.)###
  #(.)#(.)#(.)#(.)#
  #########\s*\z";
    let regex = Regex::new(PATTERN).unwrap();
    let captures = regex.captures(input).context("invalid input format")?;
    let a: Vec<Amphipod> = captures
        .iter()
        .skip(1)
        .map(|m| m.unwrap().as_str().chars().next().unwrap().try_into())
        .collect::<Result<_>>()?;
    Ok([0, 1, 2, 3].map(|i| [a[4 + i], a[i]]))
}
