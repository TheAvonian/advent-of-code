# Aven's Advent of Code Program

This is a development program meant to be used for quick running and creating of advent days.

While this is meant to be used primarily for me and for quick development; I am going to change it to be separate and run the days separately as opposed to having the compilation of the program depend on each day compiling.

## How to Use

If you would like to use this the easiest way is to do the following steps.

1. Install rust at [Rust-Lang](https://www.rust-lang.org/tools/install).
2. Delete all files and folders in the `advent-of-code/src/days/` folder.
3. Delete all files in the `advent-of-code/src/inputs/` folder.
4. Create a new day using the day and year of the puzzle you wish to do.
5. Grab the puzzle input and optionally sample input from the [Advent of Code](https://adventofcode.com).
6. Place inputs into the respected file in `advent-of-code/src/inputs/`.
7. Run day using day and year, tossing in `true` if you wish to use sample input instead.

### Create New Day

Depending on if you are running this through cargo or through the exe the command is the same but here they are anyways, just replace [day] and [year] with the actual values you desire.

Files created will be the day's rust file and two inputs, the actual input and the sample if you wish to use that. These are your puzzle input files.

#### Cargo

`cargo run new [day] [year]`

#### Exe

`advent-of-code.exe new [day] [year]`

### Run Day

Same as before but with the ability to run using either the sample file or the actual file.

`{sample}` is an optional parameter, default: `false`.

#### Cargo

`cargo run run [day] [year] {sample}`

#### Exe

`advent-of-code.exe run [day] [year] {sample}`
