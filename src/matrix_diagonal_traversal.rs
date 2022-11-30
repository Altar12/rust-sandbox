use std::{fmt::Debug, str::FromStr};

// Diagonal traversal of a matrix
fn traverse(matrix: &Vec<Vec<i32>>) {
    is_matrix(matrix);
    let row_cnt = matrix.len();
    let col_cnt = matrix[0].len();
    let mut row;
    let mut col;
    for i in (0..col_cnt).rev() {
        row = 0;
        col = i;
        loop {
            println!("{}", matrix[row][col]);
            row += 1;
            col += 1;
            if row == row_cnt || col == col_cnt {
                break;
            }
        }
    }
    for i in 1..row_cnt {
        row = i;
        col = 0;
        loop {
            println!("{}", matrix[row][col]);
            row += 1;
            col += 1;
            if row == row_cnt || col == col_cnt {
                break;
            }
        }
    }
}

fn is_matrix(matrix: &Vec<Vec<i32>>) {
    assert!(matrix.len() > 0);
    let col_cnt = matrix[0].len();
    for i in 1..matrix.len() {
        assert!(matrix[i].len() == col_cnt);
    }
}

fn convert_to_nums(input: &str) -> Vec<i32> {
    let mut found_num = false;
    let mut is_neg = false;
    let mut num = 0;
    let mut res = Vec::new();
    for ch in input.chars() {
        if ch >= '0' && ch <= '9' {
            if !found_num {
                found_num = true;
                num = (ch as u8 - 48) as i32;
            } else {
                num = num * 10 + (ch as u8 - 48) as i32;
            }
        } else if ch == '-' {
            is_neg = true;
        } else {
            if found_num {
                res.push(if is_neg { -num } else { num });
                found_num = false;
                is_neg = false;
            }
        }
    }
    res
}

fn read_matrix(matrix: &mut Vec<Vec<i32>>, row: usize, col: usize) {
    assert!(matrix.len() == 0);
    let mut input = String::new();
    let mut single_row;
    let mut nums = vec![]; //dummy init
    for _ in 0..row {
        single_row = Vec::new();
        while single_row.len() < col && nums.len() > 0 {
            single_row.push(nums[0]);
            nums.remove(0);
        }
        while single_row.len() < col {
            input.clear();
            std::io::stdin()
                .read_line(&mut input)
                .expect("could not read input");
            nums = convert_to_nums(&input);
            while single_row.len() < col && nums.len() > 0 {
                single_row.push(nums[0]);
                nums.remove(0);
            }
        }
        matrix.push(single_row);
    }
}

fn read<T: FromStr>() -> T
where
    <T as FromStr>::Err: Debug,
{
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    input.pop();
    input.parse().expect("parse error, check your input")
}

fn main() {
    let mut matrix = Vec::new();
    let row;
    let col;
    println!("Enter matrix row & col count");
    row = read::<usize>();
    col = read::<usize>();
    println!("Enter matrix");
    read_matrix(&mut matrix, row, col);
    println!("Diagonal traversal of the matrix:");
    traverse(&matrix);
}
