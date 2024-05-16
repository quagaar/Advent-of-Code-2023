pub mod part1;
pub mod part2;

#[cfg(input_txt)]
pub const INPUT: &str = include_str!("../input.txt");
#[cfg(not(input_txt))]
pub const INPUT: &str = include_str!("../example.txt");
