use std::fs;

fn read_rows(file: &str) -> Vec<Vec<bool>> {
    let file_content = fs::read_to_string(file).expect("Failed to read file");
    file_content
        .lines()
        .map(|line| line.bytes().map(|b| b == b'@').collect())
        .collect()
}

fn is_in(rows: &Vec<Vec<bool>>, i: isize, j: isize) -> bool {
    return i >= 0 && (i as usize) < rows.len() && j >= 0 && (j as usize) < rows[0].len();
}

fn is_accessible(rows: &Vec<Vec<bool>>, i: isize, j: isize) -> bool {
    let mut cnt = 0;
    for x in (i - 1)..=(i + 1) {
        for y in (j - 1)..=(j + 1) {
            if x == i && y == j {
                continue;
            }
            if !is_in(rows, x, y) {
                continue;
            }
            cnt += rows[x as usize][y as usize] as u16;
        }
    }
    cnt < 4
}

fn demolish(rows: &Vec<Vec<bool>>) -> (Vec<Vec<bool>>, u16) {
    let mut new_rows = rows.clone();
    let nrows = rows.len();
    let ncols = rows[0].len();

    let mut cleaned_cnt = 0;
    for i in 0..nrows {
        for j in 0..ncols {
            if rows[i][j] && is_accessible(rows, i as isize, j as isize) {
                cleaned_cnt += 1;
                new_rows[i][j] = false;
            }
        }
    }
    (new_rows, cleaned_cnt)
}

fn main() {
    let mut rows = read_rows("day4_ex.txt");
    let mut part_1 = 0;
    let mut part_2 = 0;

    for i in 1..10000 {
        let (new_rows, cleaned_cnt) = demolish(&rows);
        if i == 1 {
            part_1 = cleaned_cnt;
        }
        part_2 += cleaned_cnt;

        if cleaned_cnt == 0 {
            break;
        }
        rows = new_rows;
    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
