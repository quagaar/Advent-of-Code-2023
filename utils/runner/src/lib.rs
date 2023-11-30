use std::env;

pub fn run_solvers<R1, R2>(
    solver1: impl FnOnce(&str) -> R1,
    solver2: impl FnOnce(&str) -> R2,
    input: &str,
) where
    R1: std::fmt::Debug,
    R2: std::fmt::Debug,
{
    let args: Vec<String> = env::args().collect();
    if let Some(s) = args.get(1) {
        match s.as_str() {
            "1" | "part1" => {
                println!("{:?}", solver1(input));
                return;
            }
            "2" | "part2" => {
                println!("{:?}", solver2(input));
                return;
            }
            _ => (),
        }
    };
    println!("Part 1 => {:?}", solver1(input));
    println!("Part 2 => {:?}", solver2(input));
}
