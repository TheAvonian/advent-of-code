#![allow(dead_code, unused)]

use std::fmt::Display;

///
/// NOT DONE
///

pub fn run_day(input: String) {
    let mut part1 = 0;

    let game = Game::new(input);

    for y in (0..3) {
        for x in (0..11) {
            print!("{}", game.map[x][y]);
        }
        println!();
    }

    println!("Part 1: {}", part1);
    let mut part2 = 0;

    println!("Part 2: {}", part2);
}


struct Game {
    map: [[Space; 3]; 11],
}

impl Game {
    fn new(input: String) -> Self {
        // hard coded for now
        let map = [
            [Space::Open(None), Space::Blocked, Space::Blocked],
            [Space::Open(None), Space::Blocked, Space::Blocked],
            [
                Space::Door,
                Space::Room(Some(AmphipodType::Bronze)),
                Space::Room(Some(AmphipodType::Amber)),
            ],
            [Space::Open(None), Space::Blocked, Space::Blocked],
            [
                Space::Door,
                Space::Room(Some(AmphipodType::Copper)),
                Space::Room(Some(AmphipodType::Desert)),
            ],
            [Space::Open(None), Space::Blocked, Space::Blocked],
            [
                Space::Door,
                Space::Room(Some(AmphipodType::Bronze)),
                Space::Room(Some(AmphipodType::Copper)),
            ],
            [Space::Open(None), Space::Blocked, Space::Blocked],
            [
                Space::Door,
                Space::Room(Some(AmphipodType::Desert)),
                Space::Room(Some(AmphipodType::Amber)),
            ],
            [Space::Open(None), Space::Blocked, Space::Blocked],
            [Space::Open(None), Space::Blocked, Space::Blocked],
        ];
        Self { map }
    }

    fn r#move(&mut self) {

    }
}


enum Space {
    Open(Option<AmphipodType>),
    Room(Option<AmphipodType>),
    Door, // Doors are space right outside of room that are unable to be stayed in
    Blocked,
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Space::Open(v) => match v {
                Some(v) => v.to_string(),
                None => ".",
            },
            Space::Room(v) => match v {
                Some(v) => v.to_string(),
                None => ".",
            },
            Space::Door => ".",
            Space::Blocked => "#",
        })
    }
}

enum AmphipodType {
    Amber = 1,
    Bronze = 10,
    Copper = 100,
    Desert = 1000,
}

impl AmphipodType {
    fn to_string(&self) -> &str {
        match self {
            AmphipodType::Amber => "A",
            AmphipodType::Bronze => "B",
            AmphipodType::Copper => "C",
            AmphipodType::Desert => "D",
        }
    }
}
