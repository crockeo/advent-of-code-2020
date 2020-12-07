use std::fs::File;
use std::io;
use std::io::Read;

pub fn part1() -> io::Result<usize> {
    let mut contents = String::new();
    File::open("./data/06.txt")?.read_to_string(&mut contents)?;

    Ok(contents
        .split("\n\n")
        .map(|s| s.lines())
        .map(customs_declaration_form)
        .map(count_positive)
        .sum())
}

fn count_positive(vec: Vec<bool>) -> usize {
    vec.into_iter().filter(|x| *x).count()
}

fn customs_declaration_form<'a, I>(i: I) -> Vec<bool>
where
    I: Iterator<Item = &'a str>,
{
    let mut base = vec![false; 26];
    for c in i.flat_map(str::chars) {
        let idx = (c as usize) - 'a' as usize;
        base[idx] = true;
    }
    base
}
