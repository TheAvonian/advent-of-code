#![allow(dead_code, unused)]

///
/// NOT DONE
/// 

pub fn run_day(input: String) {
    let mut part1 = 0;

    // 0  1  3  4  5  6  7  8  9 10 11
    //      12    14    16    18
    //      13    15    17    19

    let game = Game::new(input);



    println!("Part 1: {}", part1);
    let mut part2 = 0;

    println!("Part 2: {}", part2);
}

struct Game {
    amphipods: Vec<Amphipod>,
    nodes: Vec<Node>
}

impl Game {
    fn new(input: String) -> Self {
        let mut nodes = (0..20).map(|x| Node {
            index: x,
            connections: vec![]
        }).collect::<Vec<Node>>();

        nodes[0].connections = vec![1];
        nodes[1].connections = vec![0, 2];
        nodes[2].connections = vec![1, 3];
        nodes[3].connections = vec![2, 4, 12];
        nodes[4].connections = vec![3, 5];
        nodes[5].connections = vec![4, 6, 14];
        nodes[6].connections = vec![5, 7];
        nodes[7].connections = vec![6, 8, 16];
        nodes[8].connections = vec![7, 9];
        nodes[9].connections = vec![8, 10, 18];
        nodes[10].connections = vec![9, 11];
        nodes[11].connections = vec![10];
        nodes[12].connections = vec![3, 13];
        nodes[13].connections = vec![12];
        nodes[14].connections = vec![7, 18];
        nodes[15].connections = vec![14];
        nodes[16].connections = vec![7, 16];
        nodes[17].connections = vec![16];
        nodes[18].connections = vec![9, 19];
        nodes[19].connections = vec![18];

        let amphipods = input
            .split('\n')
            .collect::<Vec<&str>>()[2..4]
            .iter()
            .map(|c| c.chars().skip(3).step_by(2).take_while(|f| *f != '#').collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut finished_amp = Vec::new();
        let mut i = 12;
        for x in amphipods {
            for y in &x {
                finished_amp.push(match *y {
                    'A' => {
                        Amphipod::Amber { node: i }
                    },
                    'B' => {
                        Amphipod::Bronze { node: i }
                    },
                    'C' => {
                        Amphipod::Copper { node: i }
                    },
                    'D' => {
                        Amphipod::Desert { node: i }
                    },
                    _ => panic!("Ssss"),
                });
                i += 2;
            }
            i = 13;
        }

        Self {
            nodes,
            amphipods: finished_amp
        }
    }
}

enum Amphipod {
    Amber {
        node: i32
    },
    Bronze {
        node: i32
    },
    Copper {
        node: i32
    },
    Desert {
        node: i32
    }
}

struct Node {
    index: i32,
    connections: Vec<i32>
}
