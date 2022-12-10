//Knapsack problem (divisible items)

fn read_nums() -> Vec<f32> {
    let mut input = String::new();
    let mut str_num = String::new();
    let mut num;
    let mut nums = Vec::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("could not read input");
    for ch in input.chars() {
        if ch >= '0' && ch <= '9' || ch == '.' {
            str_num.push(ch);
        } else if str_num.len() > 0 {
            num = str_num.parse().expect("parse error, invalid input");
            nums.push(num);
            str_num.clear();
        }
    }
    nums
}

fn read(input: &mut String) {
    input.clear();
    std::io::stdin()
        .read_line(input)
        .expect("could not read input");
    input.pop();
}

fn main() {
    let mut items = Vec::new();
    let mut included = Vec::new();
    let item_cnt;
    let mut nums;
    let capacity;
    let mut left_capacity;
    let mut profit;
    let mut temp;
    let mut big;
    let input = &mut String::new();

    println!("Enter knapsack capacity");
    read(input);
    capacity = input.parse::<f32>().expect("parse error, check your input");
    println!("Enter total number of items");
    read(input);
    item_cnt = input
        .parse::<usize>()
        .expect("parse error, check your input");
    println!("Enter price & wt. of items, each item in one line");
    for i in 0..item_cnt {
        nums = read_nums();
        items.push((i, nums[0], nums[1], nums[0] / nums[1]));
    }
    for start in 0..item_cnt {
        big = start;
        for i in start + 1..item_cnt {
            if items[i].3 > items[big].3 {
                big = i;
            }
        }
        temp = items[start];
        items[start] = items[big];
        items[big] = temp;
    }
    left_capacity = capacity;
    profit = 0.0;
    for item in items {
        if item.2 <= left_capacity {
            included.push((item.0, None));
            profit += item.1;
            left_capacity -= item.2;
            if left_capacity == 0.0 {
                break;
            }
        } else {
            included.push((item.0, Some(left_capacity / item.2)));
            profit += left_capacity * item.3;
            left_capacity = 0.0;
            break;
        }
    }
    println!("You can include the following items with {left_capacity} capacity remaining and profit = {profit}");
    for item in included {
        match item.1 {
            None => println!("Item{} complete", item.0),
            Some(x) => println!("{} of item{}", x, item.0),
        }
    }
}
