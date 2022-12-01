pub fn run_day(input: String) {
    let saturated_input = input.replace("\r", "");

    let mut inputs = saturated_input
        .split('\n')
        .map(|x| Seat::new(x.to_string()))
        .collect::<Vec<Seat>>();

    inputs.sort_by(|a, b| b.seat_id.cmp(&a.seat_id));

    let part1 = inputs[0].seat_id;

    println!("Part 1: {}", part1);

    let mut part2 = 0;

    let mut inputs = saturated_input
        .split('\n')
        .map(|x| Seat::new(x.to_string()))
        .filter(|x| x.row != 0 && x.row != 127)
        .collect::<Vec<Seat>>();

    inputs.sort_by(|a, b| b.seat_id.cmp(&a.seat_id));

    for seat_num in
        inputs.iter().next_back().unwrap().seat_id + 1..inputs.iter().next().unwrap().seat_id
    {
        if inputs.iter().find(|x| x.seat_id == seat_num).is_none()
            && inputs.iter().find(|x| x.seat_id == seat_num - 1).is_some()
            && inputs.iter().find(|x| x.seat_id == seat_num + 1).is_some()
        {
            part2 = seat_num;
            break;
        }
    }

    println!("Part 2: {}", part2);
}

struct Seat {
    row: i32,
    seat_id: i32,
}

impl Seat {
    // FFFFFFFRLR
    fn new(input: String) -> Self {
        let mut val = 64;
        let mut low = 0;
        for x in input[..7].chars() {
            match x {
                'B' => {
                    low += val;
                }
                _ => (),
            }

            val /= 2;
        }

        let mut val_col = 4;
        let mut low_col = 0;
        for x in input[7..10].chars() {
            match x {
                'R' => {
                    low_col += val_col;
                }
                _ => (),
            }

            val_col /= 2;
        }

        Self {
            row: low,
            seat_id: low * 8 + low_col,
        }
    }
}
