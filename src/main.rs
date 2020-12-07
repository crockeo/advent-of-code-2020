pub mod problem_01;
pub mod problem_02;
pub mod problem_03;
pub mod problem_04;
pub mod problem_05;

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

    let p31 = problem_03::part1()?;
    println!("p31: {}", p31);

    let p32 = problem_03::part2()?;
    println!("p32: {}", p32);

    let p41 = problem_04::part1()?;
    println!("p41: {}", p41);

    let p42 = problem_04::part2()?;
    println!("p42: {}", p42);

    Ok(())
}
