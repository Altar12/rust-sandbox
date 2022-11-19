//towers of Hanoi
use std::process::exit;

fn move_disks(src: char, dest: char, aux: char, disk_amount: u8) {
    if disk_amount == 0 {
        println!("invalid input");
        return;
    }
    if disk_amount == 1 {
        println!("Move 1 disk from {} to {}", src, dest);
    } else {
        move_disks(src, aux, dest, disk_amount - 1);
        move_disks(src, dest, aux, 1);
        move_disks(aux, dest, src, disk_amount - 1);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("Expected 1 argument, found {}", args.len());
        exit(1);
    }
    let disk_amt: u8 = args[0].parse().expect("invalid input");
    move_disks('A', 'B', 'C', disk_amt);
}
