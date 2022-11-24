//Determinant of a matrix

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

fn remove_row_col(matrix: &Vec<Vec<i32>>, row: usize, col: usize) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut single_row;
    for i in 0..matrix.len() {
        if i == row {
            continue;
        }
        single_row = Vec::new();
        for j in 0..matrix[i].len() {
            if j == col {
                continue;
            }
            single_row.push(matrix[i][j]);
        }
        res.push(single_row);
    }
    res
}

fn determinant(matrix: &Vec<Vec<i32>>) -> i32 {
    assert!(matrix.len() > 0);
    let row_cnt = matrix.len();
    let col_cnt = matrix[0].len();
    assert!(row_cnt == col_cnt);
    for i in 1..row_cnt {
        assert!(matrix[i].len() == col_cnt);
    }
    if row_cnt == 1 {
        return matrix[0][0];
    }
    let mut res = 0;
    let mut sign = 1;
    let mut temp;
    for j in 0..col_cnt {
        temp = remove_row_col(matrix, 0, j);
        res += sign * matrix[0][j] * determinant(&temp);
        sign = -sign;
    }
    res
}

fn main() {
    let mut matrix = Vec::new();
    let row_cnt;
    let mut input = String::new();
    println!("Enter dimension of square matrix");
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    input.pop();
    row_cnt = input.parse().expect("invalid input");
    println!("Enter matrix");
    read_matrix(&mut matrix, row_cnt, row_cnt);
    println!("Determinant = {}", determinant(&matrix));
}
