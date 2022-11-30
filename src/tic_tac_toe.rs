// Tic-tac-toe

use std::{fmt::Debug, str::FromStr};

#[derive(Clone, Copy, PartialEq)]
enum Symbol {
    X,
    O,
}
enum Error {
    InvalidPosition,
    PositionAlreadyTaken,
}

fn check_winner(board: &[[Option<Symbol>; 3]; 3]) -> Option<Symbol> {
    for i in 0..3 {
        if board[i][i] != None {
            if board[i][0] == board[i][1] && board[i][1] == board[i][2] {
                return board[i][0];
            }
            if board[0][i] == board[1][i] && board[1][i] == board[2][i] {
                return board[0][i];
            }
        }
    }
    if board[0][0] != None && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return board[0][0];
    }
    if board[0][2] != None && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return board[1][1];
    }
    None
}

fn print_board(board: &[[Option<Symbol>; 3]; 3]) {
    println!("-------");
    for row in board {
        for symbol in row {
            match symbol {
                Some(s) => match s {
                    Symbol::X => print!("|X"),
                    Symbol::O => print!("|0"),
                },
                None => print!("| "),
            }
        }
        println!("|");
        println!("-------");
    }
}

fn place(
    board: &mut [[Option<Symbol>; 3]; 3],
    row: usize,
    col: usize,
    symbol: Symbol,
) -> Result<(), Error> {
    if row >= 3 || col >= 3 {
        return Err(Error::InvalidPosition);
    }
    if let Some(_) = board[row][col] {
        return Err(Error::PositionAlreadyTaken);
    }
    board[row][col] = Some(symbol);
    Ok(())
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
    let mut board = [[None; 3]; 3];
    let mut row;
    let mut col;
    let symbols = [Symbol::X, Symbol::O];
    let mut turns_taken = 0;
    let mut curr_player;
    loop {
        print_board(&board);
        if turns_taken >= 3 {
            match check_winner(&board) {
                Some(s) => {
                    match s {
                        Symbol::X => println!("Player 1 won the game"),
                        Symbol::O => println!("Player 2 won the game"),
                    }
                    break;
                }
                None => (),
            }
            if turns_taken == 9 {
                println!("Game draw");
                break;
            }
        }
        curr_player = turns_taken % 2;
        println!("Player{}'s turn", curr_player + 1);
        println!("Enter row & col");
        row = read::<usize>();
        col = read::<usize>();
        match place(&mut board, row, col, symbols[curr_player]) {
            Err(e) => match e {
                Error::InvalidPosition => println!("Invalid Position!!"),
                Error::PositionAlreadyTaken => println!("Position Already Taken!!"),
            },
            Ok(_) => turns_taken += 1,
        }
    }
}
