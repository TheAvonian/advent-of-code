mod day1_2022;
mod day15_2020;
mod day16_2020;
mod day2_2020;
mod day3_2020;
mod day4_2020;
pub fn run_day(year: u16, day: u8, input: String) {
    match year {
        2022 => {
            match day {
                1 => {
                    day1_2022::run_day(input);
                },
                _ => ()
            }
        },
        2020 => {
            match day {
                15 => {
                    day15_2020::run_day(input);
                },
                16 => {
                    day16_2020::run_day(input);
                },
                2 => {
                    day2_2020::run_day(input);
                },
                3 => {
                    day3_2020::run_day(input);
                },
                4 => {
                    day4_2020::run_day(input);
                },
                _ => ()
            }
        },
        _ => ()
    }
}
