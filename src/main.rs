pub mod problem_01;
pub mod problem_02;

use std::io;

fn main() -> io::Result<()> {
    let p11 = problem_01::part1()?;
    println!("p11: {}", p11);

    let p12 = problem_01::part2()?;
    println!("p12: {}", p12);

    let p21 = problem_02::part1()?;
    println!("p21: {}", p21);

    let p22 = problem_02::part2()?;
    println!("p22: {}", p22);

    Ok(())
}
