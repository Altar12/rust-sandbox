// Matrix multiplication
fn multiply_matrix(matrix_a: &Vec<Vec<i32>>, matrix_b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    assert!(matrix_a.len() > 0 && matrix_b.len() > 0);
    for i in 0..matrix_b.len() - 1 {
        assert!(matrix_b[i].len() == matrix_b[i + 1].len());
    }
    let mut res = Vec::new();
    let mut sum;
    let mut single_row;
    let b_row_cnt = matrix_b.len();
    let b_col_cnt = matrix_b[0].len();
    for row_a in matrix_a {
        assert!(row_a.len() == b_row_cnt);
        single_row = Vec::new();
        for j in 0..b_col_cnt {
            sum = 0;
            for k in 0..b_row_cnt {
                sum += row_a[k] * matrix_b[k][j];
            }
            single_row.push(sum);
        }
        res.push(single_row);
    }
    res
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

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    let mut single_row;
    for i in 0..matrix.len() {
        single_row = &matrix[i];
        for j in 0..single_row.len() {
            print!("{}\t", single_row[j]);
        }
        println!("");
    }
}

fn read(input: &mut String) {
    input.clear();
    std::io::stdin()
        .read_line(input)
        .expect("could not read input");
    input.pop();
}

fn main() {
    let mut matrix_a = Vec::new();
    let mut matrix_b = Vec::new();
    let res;
    let input = &mut String::new();
    let row_a;
    let col_a;
    let row_b;
    let col_b;
    println!("Enter row & col count of first matrix");
    read(input);
    row_a = input.parse().expect("invalid input");
    read(input);
    col_a = input.parse().expect("invalid input");
    println!("Enter row & col count of second matrix");
    read(input);
    row_b = input.parse().expect("invalid input");
    read(input);
    col_b = input.parse().expect("invalid input");
    println!("Enter first matrix");
    read_matrix(&mut matrix_a, row_a, col_a);
    println!("Enter second matrix");
    read_matrix(&mut matrix_b, row_b, col_b);
    res = multiply_matrix(&matrix_a, &matrix_b);
    println!("Multiplication result");
    print_matrix(&res);
}
