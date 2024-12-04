const SEARCH_WORD: &str = "XMAS";
const SEARCH_WORD_CHARS: &[char] = &['X', 'M', 'A', 'S'];
const SEARCH_WORD_CHARS_REVERSED: &[char] = &['S', 'A', 'M', 'X'];
const SEARCH_WORD_MAS: &[char] = &['M', 'A', 'S'];
const SEARCH_WORD_MAS_REVERSED: &[char] = &['S', 'A', 'M'];

pub fn first(input: String) -> String {
    let matrix = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut total: usize = 0;

    // horizontal
    for row in &matrix {
        for start in 0..cols - SEARCH_WORD.len() + 1 {
            if &row[start..start + SEARCH_WORD.len()] == SEARCH_WORD_CHARS
                || &row[start..start + SEARCH_WORD.len()] == SEARCH_WORD_CHARS_REVERSED
            {
                total += 1;
            }
        }
    }

    // vertical
    for col in 0..cols {
        for row_start in 0..rows - SEARCH_WORD.len() + 1 {
            let possible = &matrix[row_start..row_start + SEARCH_WORD.len()]
                .iter()
                .map(|row| row[col])
                .collect::<Vec<_>>();
            if possible.as_slice() == SEARCH_WORD_CHARS
                || possible.as_slice() == SEARCH_WORD_CHARS_REVERSED
            {
                total += 1;
            }
        }
    }

    // top left to down right diagonal
    for col_start in 0..cols - SEARCH_WORD.len() + 1 {
        for row_start in 0..rows - SEARCH_WORD.len() + 1 {
            let possible = matrix[row_start..row_start + SEARCH_WORD.len()]
                .iter()
                .enumerate()
                .map(|(i, col)| col[col_start + i])
                .collect::<Vec<_>>();
            if possible.as_slice() == SEARCH_WORD_CHARS
                || possible.as_slice() == SEARCH_WORD_CHARS_REVERSED
            {
                total += 1;
            }
        }
    }

    // top right to left down diagonal
    for col_start in SEARCH_WORD.len() - 1..cols {
        for row_start in 0..rows - SEARCH_WORD.len() + 1 {
            let possible = matrix[row_start..row_start + SEARCH_WORD.len()]
                .iter()
                .enumerate()
                .map(|(i, col)| col[col_start - i])
                .collect::<Vec<_>>();
            if possible.as_slice() == SEARCH_WORD_CHARS
                || possible.as_slice() == SEARCH_WORD_CHARS_REVERSED
            {
                total += 1;
            }
        }
    }

    total.to_string()
}

pub fn second(input: String) -> String {
    let matrix = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut total: usize = 0;
    for col_start in 0..cols - 2 {
        for row_start in 0..rows - 2 {
            let rows = &matrix[row_start..row_start + 3];
            // top left to down right diagonal
            let first = rows
                .iter()
                .enumerate()
                .map(|(i, row)| row[col_start + i])
                .collect::<Vec<_>>();

            // top right to down left diagonal
            let second = rows
                .iter()
                .enumerate()
                .map(|(i, row)| row[col_start + 2 - i])
                .collect::<Vec<_>>();

            if (first.as_slice() == SEARCH_WORD_MAS || first.as_slice() == SEARCH_WORD_MAS_REVERSED)
                && (second.as_slice() == SEARCH_WORD_MAS
                    || second.as_slice() == SEARCH_WORD_MAS_REVERSED)
            {
                total += 1;
            }
        }
    }

    total.to_string()
}
