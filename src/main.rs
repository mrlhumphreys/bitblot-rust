extern crate rand;

use rand::Rng;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::Range;

#[derive(Clone)]
#[derive(Eq)]
struct Square {
    x: i16,
    y: i16
}

impl PartialEq for Square {
  fn eq(&self, other: &Square) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl Hash for Square {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        let denominator = 2*(self.x + self.y)*(self.x + self.y + 1);
        if denominator == 0 {
            (0).hash(state)
        } else {
            (1/denominator + self.y).hash(state)
        }
    }
}

struct Group {
    squares: HashSet<Square>
}

impl Group {
    fn random_surrounding_square(&self) -> Square {
        let arr = self.surrounding_squares().iter().cloned().collect::<Vec<Square>>();
        let max = arr.len() - 1;
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0, max);
        arr[random_number].clone()
    }

    fn surrounding_squares(&self) -> HashSet<Square> {
        let all_squares = self.squares.iter().flat_map( |s| {
            return vec![
                Square { x: s.x+1, y: s.y },
                Square { x: s.x, y: s.y+1 },
                Square { x: s.x-1, y: s.y },
                Square { x: s.x, y: s.y-1 }
            ].into_iter();
        }).collect::<HashSet<Square>>();

        all_squares.difference(&self.squares).cloned().collect::<HashSet<Square>>()
    }

    fn print_grid(&self) -> () {
        for y in self.range_y() {
            let row_one = self.range_x().map( |x|
                if self.squares.iter().any(|s| s.x == x && s.y == y) {
                    return "    ";
                } else {
                    return "▐▀▀▌";
                }
            ).collect::<Vec<&str>>().join("");

            let row_two = self.range_x().map( |x|
                if self.squares.iter().any(|s| s.x == x && s.y == y) {
                    return "    ";
                } else {
                    return "▐▄▄▌";
                }
            ).collect::<Vec<&str>>().join("");

            println!("{}", row_one);
            println!("{}", row_two);
        }
    }

    fn range_y(&self) -> Range<i16> {
        Range { start: self.min_y(), end: self.max_y() + 1 }
    }

    fn range_x(&self) -> Range<i16> {
        Range { start: self.min_x(), end: self.max_x() + 1 }
    }

    fn min_y(&self) -> i16 {
        self.squares.iter().min_by_key( |s| s.y ).unwrap().y
    }

    fn max_y(&self) -> i16 {
        self.squares.iter().max_by_key( |s| s.y ).unwrap().y
    }

    fn min_x(&self) -> i16 {
        self.squares.iter().min_by_key( |s| s.x ).unwrap().x
    }

    fn max_x(&self) -> i16 {
        self.squares.iter().max_by_key( |s| s.x ).unwrap().x
    }
}

fn add_random(group: &mut Group) {
    let random = group.random_surrounding_square();
    group.squares.insert(random);
}

fn main() {
    let squares: HashSet<Square> = vec![Square { x: 0, y: 0 }].into_iter().collect();
    let mut group = Group { squares: squares };

    for _ in 0..128 {
        add_random(&mut group);
    }

    group.print_grid();
}
