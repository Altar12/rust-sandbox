use std::{fmt::Debug, str::FromStr};

// L traversal of a matrix
fn traverse(matrix: &Vec<Vec<i32>>) {
    is_matrix(matrix);
    let row_cnt = matrix.len();
    let col_cnt = matrix[0].len();
    let mut row = 0;
    let mut col = 0;
    while row < row_cnt && col < col_cnt {
        if row == col {
            for i in row..row_cnt {
                println!("{}", matrix[i][col]);
            }
            col += 1;
        } else {
            for i in col..col_cnt {
                println!("{}", matrix[row][i]);
            }
            row += 1;
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
