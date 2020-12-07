use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub fn part1() -> io::Result<i32> {
    BufReader::new(File::open("./data/05.txt")?)
        .lines()
        .flat_map(|locator| find_seat(128, 8, locator.unwrap().as_str()))
        .map(|(row, col)| seat_id(row, col))
        .max()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "invalid locator"))
}

pub fn part2() -> io::Result<i32> {
    let mut sorted_seat_ids = BufReader::new(File::open("./data/05.txt")?)
        .lines()
        .flat_map(|locator| find_seat(128, 8, locator.unwrap().as_str()))
        .map(|(row, col)| seat_id(row, col))
        .collect::<Vec<i32>>();
    sorted_seat_ids.sort_unstable();

    let mut it = sorted_seat_ids.iter();
    it.next();

    for (seat_id, next_seat_id) in sorted_seat_ids.iter().zip(it) {
        if next_seat_id - seat_id == 2 {
            return Ok(seat_id + 1);
        }
    }

    Ok(0)
}

fn seat_id(row: i32, col: i32) -> i32 {
    row * 8 + col
}

fn find_seat(rows: i32, cols: i32, locator: &str) -> Option<(i32, i32)> {
    let mut row_min = 0;
    let mut row_max = rows;
    let mut col_min = 0;
    let mut col_max = cols;

    for c in locator.chars() {
        match c {
            'F' => row_max = (row_min + row_max) / 2,
            'B' => row_min = (row_min + row_max) / 2,
            'L' => col_max = (col_min + col_max) / 2,
            'R' => col_min = (col_min + col_max) / 2,
            _ => return None,
        }
    }

    if row_min == row_max - 1 && col_min == col_max - 1 {
        Some((row_min, col_min))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_seat() {
        let locator = "FBFBBFFRLR";
        let location = find_seat(128, 8, locator).unwrap();
        assert_eq!(location, (44, 5));
    }

    #[test]
    fn test_find_seat_failure() {
        let locators = vec![
            "fake locator", // poorly structured
            "FBBFRLR",      // incomplete rows
            "FBFBBFFRL",    // incomplete columns
        ];
        for locator in locators.into_iter() {
            assert_eq!(find_seat(128, 8, locator), None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_id() {
        let (row, col) = find_seat(128, 8, "BFFFBBFRRR").unwrap();
        let id = seat_id(row, col);

        assert_eq!(id, 567);
    }
}
