use std::collections::BTreeMap;
use std::env;

pub mod problem_01;
pub mod problem_02;
pub mod problem_03;
pub mod problem_04;
pub mod problem_05;
pub mod problem_06;
pub mod problem_07;
pub mod problem_15;
pub mod problem_16;
pub mod solution;

use solution::ProblemSolution;
use std::io;

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();

    let mode;
    if args.len() < 2 {
        mode = "all";
    } else {
        mode = args[1].as_str();
    }

    exec_problem(mode)
}

fn exec_problem(s: &str) -> io::Result<()> {
    let mut map: BTreeMap<&str, &dyn ProblemSolution> = BTreeMap::new();

    map.insert("p1", &problem_01::Solution {});
    map.insert("p2", &problem_02::Solution {});
    map.insert("p3", &problem_03::Solution {});
    map.insert("p4", &problem_04::Solution {});
    map.insert("p5", &problem_05::Solution {});
    map.insert("p6", &problem_06::Solution {});
    map.insert("p7", &problem_07::Solution {});
    map.insert("p15", &problem_15::Solution {});
    map.insert("p16", &problem_16::Solution {});

    if s == "all" {
        for (_, solution) in map.into_iter() {
            solution.exec()?;
        }
    } else {
        map.get(s)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("no such solution: {}", s),
                )
            })?
            .exec()?;
    }

    Ok(())
}
