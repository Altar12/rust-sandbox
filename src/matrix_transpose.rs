// Transpose of a matrix

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

fn transpose(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if matrix.len() == 0 {
        return Vec::new();
    }
    let mut res = Vec::new();
    let row_cnt = matrix.len();
    let mut single_row;
    for i in 0..row_cnt - 1 {
        assert!(matrix[i].len() == matrix[i + 1].len());
    }
    for j in 0..matrix[0].len() {
        single_row = Vec::new();
        for i in 0..row_cnt {
            single_row.push(matrix[i][j]);
        }
        res.push(single_row);
    }
    res
}

fn read(input: &mut String) {
    input.clear();
    std::io::stdin()
        .read_line(input)
        .expect("could not read input");
    input.pop();
}

fn main() {
    let mut matrix = Vec::new();
    let res;
    let row;
    let col;
    let input = &mut String::new();
    println!("Enter row & col count of the matrix");
    read(input);
    row = input.parse().expect("invalid input");
    read(input);
    col = input.parse().expect("invalid input");
    println!("Enter the matrix");
    read_matrix(&mut matrix, row, col);
    res = transpose(&matrix);
    println!("Transpose");
    print_matrix(&res);
}
