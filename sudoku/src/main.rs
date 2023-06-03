use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}


fn valid_solution(sudoku: &[[u8;9]; 9]) -> bool {
    for line in sudoku {
        if !all_unique(*line) {
            return false;
        }

        for digit in line {
            if digit < &1 || digit > &9 {
                return false;
            }
        }

    }

    for i in 0..9 {
        let mut row = Vec::new();
        
        for line in sudoku {
            row.push(line[i]);
        }
        let hs = row.into_iter().collect::<HashSet<u8>>();

        if hs.len() != 9 {
            return false;
        }
    }

    // for

    true
}


fn all_unique(vec: [u8;9]) -> bool {
    let hs = vec.into_iter().collect::<HashSet<u8>>();
    hs.len() == 9
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod sample_tests {
    use super::valid_solution;

    #[test]
    fn valid_sudoku() {
        let puzzle = [[7, 6, 9, 5, 3, 8, 1, 2, 4],
                      [2, 4, 3, 7, 1, 9, 6, 5, 8],
                      [8, 5, 1, 4, 6, 2, 9, 7, 3],
                      [4, 8, 6, 9, 7, 5, 3, 1, 2],
                      [5, 3, 7, 6, 2, 1, 4, 8, 9],
                      [1, 9, 2, 8, 4, 3, 7, 6, 5],
                      [6, 1, 8, 3, 5, 4, 2, 9, 7],
                      [9, 7, 4, 2, 8, 6, 5, 3, 1],
                      [3, 2, 5, 1, 9, 7, 8, 4, 6]];
        let actual = valid_solution(&puzzle);
        assert_eq!(actual, true, "\nYour result (left) did not match expected result (right).");
    }
    
    #[test]
    fn invalid_sudoku() {
        let puzzle = [[7, 6, 9, 5, 3, 8, 1, 2, 4],
                      [2, 4, 3, 7, 1, 9, 6, 5, 8],
                      [8, 5, 1, 4, 6, 2, 9, 7, 3],
                      [4, 8, 6, 9, 7, 5, 3, 1, 2],
                      [5, 3, 7, 6, 2, 1, 4, 8, 9],
                      [1, 9, 2, 8, 4, 3, 7, 6, 5],
                      [6, 1, 8, 3, 5, 4, 2, 9, 7],
                      [9, 7, 4, 2, 8, 6, 5, 3, 1],
                      [3, 2, 5, 1, 9, 7, 8, 4, 9]];
        let actual = valid_solution(&puzzle);
        assert_eq!(actual, false, "\nYour result (left) did not match expected result (right).");
    }
    
    #[test]
    fn invalid_with_zeroes() {
        let puzzle = [[3, 1, 5, 8, 4, 7, 6, 2, 9],
                      [4, 7, 8, 2, 9, 6, 3, 5, 0],
                      [2, 9, 6, 3, 5, 1, 7, 8, 4],
                      [7, 4, 2, 9, 6, 8, 5, 1, 3],
                      [6, 8, 9, 5, 1, 3, 4, 7, 2],
                      [5, 0, 1, 4, 7, 2, 8, 9, 6],
                      [1, 2, 4, 6, 8, 5, 9, 3, 7],
                      [8, 6, 3, 7, 2, 9, 0, 4, 5],
                      [9, 5, 7, 1, 3, 4, 2, 6, 8]];
        let actual = valid_solution(&puzzle);
        assert_eq!(actual, false, "\nYour result (left) did not match expected result (right).");
    }

    #[test]
    fn invalid_sudoku_4() {
        let puzzle = [
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [2, 3, 1, 5, 6, 4, 8, 9, 7],
            [3, 1, 2, 6, 4, 5, 9, 7, 8],
            [4, 5, 6, 7, 8, 9, 1, 2, 3],
            [5, 6, 4, 8, 9, 7, 2, 3, 1],
            [6, 4, 5, 9, 7, 8, 3, 1, 2],
            [7, 8, 9, 1, 2, 3, 4, 5, 6],
            [8, 9, 7, 2, 3, 1, 5, 6, 4],
            [9, 7, 8, 3, 1, 2, 6, 4, 5]];
        let actual = valid_solution(&puzzle);
        assert_eq!(actual, false, "\nкубики не корректны");

    }

}
