pub fn run_day(input: String) {
    let saturated_input = input.replace("\r", "");

    let mut part1 = 0;

    let map = Map::new(saturated_input);

    let (mut x, mut y) = (0, 0);

    while y < map.height {
        if map.get(x, y) {
            part1 += 1;
        }

        x += 3;
        y += 1;
    }

    println!("Part 1: {}", part1);

    let mut part2: i64 = 1;

    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for slope in slopes {
        let (mut x, mut y) = (0, 0);
        let mut temp = 0;
        while y < map.height {
            if map.get(x, y) {
                temp += 1;
            }

            x += slope.0;
            y += slope.1;
        }
        part2 *= temp;
    }

    println!("Part 2: {}", part2);
}

struct Map {
    // y, x
    grid: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

impl Map {
    fn new(pattern: String) -> Self {
        let grid = pattern
            .split('\n')
            .map(|i| i.chars().map(|c| c == '#').collect::<Vec<bool>>())
            .collect::<Vec<Vec<bool>>>();
        let height = grid.len();
        let width = grid[0].len();
        Self {
            grid,
            height,
            width,
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        return self.grid[y % self.height][x % self.width];
    }
}
