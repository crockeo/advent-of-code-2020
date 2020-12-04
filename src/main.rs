pub mod problem_01;

use std::io;

fn main() -> io::Result<()> {
    let solution = problem_01::part1()?;
    println!("{}", solution);

    let solution2 = problem_01::part2()?;
    println!("{}", solution2);

    Ok(())
}
