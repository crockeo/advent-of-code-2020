use std::io;

pub fn part1() -> io::Result<i32> {
    Ok(0)
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
