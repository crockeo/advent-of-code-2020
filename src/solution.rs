use std::io;

pub trait ProblemSolution {
    fn name(&self) -> &'static str;

    fn part1(&self) -> io::Result<i64>;
    fn part2(&self) -> io::Result<i64>;

    fn exec(&self) -> io::Result<()> {
        println!("{}::p1 {}", self.name(), self.part1()?);
        println!("{}::p2 {}", self.name(), self.part2()?);
        Ok(())
    }
}
