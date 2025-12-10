///
/// This is a non AoC puzzle that is actually
/// a final project for a college course that
/// I decided to implement in Rust.
///
/// This will not work without the proper
/// inputs, of which are too big to be given.
///
/// Purpose: Take in a u32 representation of
/// an ip or an ip itself and determine the
/// nearest location in the world via the
/// input data.
///
/// The actual puzzle part of this isn't so
/// much finding but rather just implementing
/// a radix trie to handle IP's.
///
/// Made a fully generic radix trie though
/// so that's pretty cool.
///
use std::{
    fmt::Display,
    io,
    ops::{BitAnd, Shr},
};

pub struct RadixTrie<K, V> {
    root: Option<Box<Node<K, V>>>,
}

#[derive(Clone)]
pub enum Node<K, V> {
    Edge {
        left: Option<Box<Node<K, V>>>,
        right: Option<Box<Node<K, V>>>,
    },
    Leaf {
        key: K,
        value: V,
    },
}

impl<K, V> RadixTrie<K, V>
where
    K: std::ops::Shr<u32>,
    <K as Shr<u32>>::Output: BitAnd<u32>,
    <<K as Shr<u32>>::Output as BitAnd<u32>>::Output: PartialEq<u32>,
    K: Copy,
    V: Clone,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, key: K, value: V) {
        const MAX_RADIX: u32 = 32;

        let mut current_node = match &mut self.root {
            Some(val) => match **val {
                Node::Edge { left: _, right: _ } => &mut self.root,
                Node::Leaf { key: _, value: _ } => {
                    panic!("BAD")
                }
            },
            None => {
                self.root = Some(Box::new(Node::Edge {
                    left: None,
                    right: None,
                }));
                &mut self.root
            }
        };

        for depth in 1..=MAX_RADIX {
            let cur = MAX_RADIX - depth;
            if (key >> cur) & 1 == 1 {
                if let Some(root) = &mut current_node {
                    match **root {
                        Node::Edge {
                            left: _,
                            ref mut right,
                        } => {
                            match depth {
                                MAX_RADIX => {
                                    *right = Some(Box::new(Node::Leaf { key, value }));
                                    return;
                                }
                                _ => {
                                    if right.is_none() {
                                        *right = Some(Box::new(Node::Edge {
                                            left: None,
                                            right: None,
                                        }));
                                    }
                                }
                            };
                        }
                        Node::Leaf { key: _, value: _ } => panic!("NO"),
                    }
                }

                current_node = match current_node.as_deref_mut().unwrap() {
                    Node::Edge { left: _, right } => right,
                    Node::Leaf { key: _, value: _ } => panic!("NO"),
                }
            } else {
                if let Some(root) = &mut current_node {
                    match **root {
                        Node::Edge {
                            ref mut left,
                            right: _,
                        } => {
                            match depth {
                                MAX_RADIX => {
                                    *left = Some(Box::new(Node::Leaf { key, value }));
                                    return;
                                }
                                _ => {
                                    if left.is_none() {
                                        *left = Some(Box::new(Node::Edge {
                                            left: None,
                                            right: None,
                                        }));
                                    }
                                }
                            };
                        }
                        Node::Leaf { key: _, value: _ } => panic!("NO"),
                    }
                }

                current_node = match current_node.as_deref_mut().unwrap() {
                    Node::Edge { left, right: _ } => left,
                    Node::Leaf { key: _, value: _ } => panic!("NO"),
                };
            }
        }
    }

    fn find_closest_inner(se: &Node<K, V>, find_key: K, depth: u32) -> Option<Node<K, V>> {
        match se {
            Node::Leaf { key: _, value: _ } => Some(se.clone()),
            Node::Edge { left, right } => {
                if (find_key >> (31 - depth)) & 1u32 == 0 {
                    if let Some(inn) = left {
                        Self::find_closest_inner(&inn, find_key, depth + 1)
                    } else if let Some(ott) = right {
                        Self::find_closest_inner(&ott, find_key, depth + 1)
                    } else {
                        None
                    }
                } else {
                    if let Some(inn) = right {
                        Self::find_closest_inner(&inn, find_key, depth + 1)
                    } else if let Some(ott) = left {
                        Self::find_closest_inner(&ott, find_key, depth + 1)
                    } else {
                        None
                    }
                }
            }
        }
    }

    pub fn find_closest(&self, key: K) -> Option<Node<K, V>> {
        Self::find_closest_inner(self.root.as_ref().unwrap().as_ref(), key, 0)
    }
}

#[derive(Clone, Debug)]
struct Location {
    country_code: String,
    country_name: String,
    province: String,
    city: String,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "{}: {}, {}, {}",
                self.country_code, self.country_name, self.city, self.province,
            )
            .as_str(),
        )
    }
}

pub fn run_day(input: String) {
    let mut map: RadixTrie<u32, Box<Location>> = RadixTrie::new();

    println!("Loading.");

    let mut i = 0;
    let v = input.lines().count();
    for line in input.lines() {
        let line: Vec<String> = line.split(',').map(|f| f.replace('"', "")).collect();

        let location = Location {
            country_code: line[2].clone(),
            country_name: line[3].clone(),
            province: line[4].clone(),
            city: line[5].clone(),
        };

        if i % 1000 == 0 {
            println!("{i}/{}", v);
        }
        i += 1;

        map.insert(line[0].parse::<u32>().unwrap(), Box::new(location.clone()));
        map.insert(line[1].parse::<u32>().unwrap(), Box::new(location.clone()));
    }

    println!("Done loading.");

    println!("\nEnter an ipv4 string or a number (or a blank line to quit).");

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 2 {
                    break;
                }
            }
            Err(_) => {
                break;
            }
        };

        if input.is_empty() {
            break;
        }

        let true_input: u32;

        input = input.replace("\n", "").replace("\r", "");

        let ip: String;

        if input.contains('.') {
            let new_input: Vec<u8> = input
                .split('.')
                .map(|f| f.parse::<u8>().unwrap_or(0))
                .collect();

            let octs: Vec<u8> = (0..4).map(|f| *new_input.get(f).unwrap_or(&0)).collect();

            ip = format!("{}.{}.{}.{}", octs[0], octs[1], octs[2], octs[3]);

            true_input = u32::from_str_radix(
                octs.iter()
                    .fold(String::from(""), |a, b| a + format!("{b:0>8b}").as_str())
                    .as_str(),
                2,
            )
            .unwrap();
        } else {
            match input.parse::<u32>() {
                Ok(val) => true_input = val,
                Err(_) => {
                    println!("{}: (INVALID, -: -, -, -)", u32::MAX);
                    continue;
                }
            }
            let binary = format!("{true_input:b}");

            let mut new_binary: String = binary.chars().collect();

            while new_binary.len() < 32 {
                new_binary = "0000".to_string() + &new_binary;
            }

            let binding = new_binary.chars().collect::<Vec<char>>();

            let mut octets = binding.chunks(8);

            ip = format!(
                "{}.{}.{}.{}",
                isize::from_str_radix(
                    octets.next().unwrap().iter().collect::<String>().as_str(),
                    2
                )
                .unwrap(),
                isize::from_str_radix(
                    octets.next().unwrap().iter().collect::<String>().as_str(),
                    2
                )
                .unwrap(),
                isize::from_str_radix(
                    octets.next().unwrap().iter().collect::<String>().as_str(),
                    2
                )
                .unwrap(),
                isize::from_str_radix(
                    octets.next().unwrap().iter().collect::<String>().as_str(),
                    2
                )
                .unwrap(),
            );
        }

        if let Some(v) = map.find_closest(true_input) {
            if let Node::Leaf { key, value } = v {
                println!("\n{}: ({}, {})", key, ip, value);
            }
        }
    }

    println!("exit");
}
