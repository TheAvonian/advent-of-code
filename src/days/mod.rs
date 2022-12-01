mod year_2020;
mod year_2021;
pub fn run_day(year: u16, day: u8, input: String) {
    match year {
        2020 => {
            match day {
                15 => {
                    year_2020::day_15::run_day(input);
                },
                16 => {
                    year_2020::day_16::run_day(input);
                },
                2 => {
                    year_2020::day_2::run_day(input);
                },
                3 => {
                    year_2020::day_3::run_day(input);
                },
                4 => {
                    year_2020::day_4::run_day(input);
                },
                5 => {
                    year_2020::day_5::run_day(input);
                },
                6 => {
                    year_2020::day_6::run_day(input);
                },
                7 => {
                    year_2020::day_7::run_day(input);
                },
                _ => ()
            }
        },
        2021 => {
            match day {
                23 => {
                    year_2021::day_23::run_day(input);
                },
                24 => {
                    year_2021::day_24::run_day(input);
                },
                _ => ()
            }
        },
        _ => ()
    }
}
