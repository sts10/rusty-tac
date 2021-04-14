extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    let mut game_over = false;
    let mut turn_number = 1;
    let mut board = [0, 0, 0, 0, 0, 0, 0, 0, 0];

    while game_over != true {
        let player;
        if turn_number % 2 == 1 {
            player = 1;
        } else {
            player = 2;
        }

        println!("Player {}'s turn", player);
        present_board(&board);

        if player == 1 {
            execute_player_move(get_int_from_input(), player, &mut board);
        } else {
            // execute_player_move(random_pick(&board), player, &mut board);
            execute_player_move(alfred_pick(&board), player, &mut board);
        }

        if check_for_win(&board) {
            present_board(&board);
            println!("Player {} wins!", player);
            game_over = true;
        } else if check_if_board_full(&board) {
            println!("Board is full-- it's a tie");
            game_over = true;
        } else {
            turn_number = turn_number + 1;
        }
    }
}

// Figuring out how pass this array was real tricky to figure out. Errors, both intentional (board[2].what_the_fuck_type_is_this;)
// and not, kept referring to "integer" or "Integer" rather than i32. Using "int" threw unhelpful
// error
fn present_board(b: &[i32]) {
    println!("---------");
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            // if empty, print the number that a user would enter to move
            // to this space
            0 => print!("{}", i),
            1 => print!("X"),
            2 => print!("O"),
            10 => print!("O"),
            _ => break,
        }
        // and now, some decorators
        if i > 0 && (i + 1) % 3 == 0 {
            print!("\n");
        } else {
            print!(" | ");
        }
        i = i + 1;
    }
    println!("---------");
}

fn get_int_from_input() -> i32 {
    let reader: io::Stdin = io::stdin();
    let mut input_text: String = String::new();
    let result: Result<usize, io::Error> = reader.read_line(&mut input_text);
    if result.is_err() {
        println!("failed to read from stdin");
        // return;
    }
    let trimmed: &str = input_text.trim();
    let option: Option<i32> = trimmed.parse::<i32>().ok();
    match option {
        Some(i) => return i,
        None => return 11,
    };
}

// https://play.rust-lang.org/?gist=182dc2ad8763bc3fa683d52749e202b4&version=stable
// woof this was a beast. Had to go to the irc channel. Eventually, the answer was to
// transfer ownership, rather than a reference, of this_move and player; and to
// convert this_move from an i32 to a usize with `as` when you want to use it as an index
fn execute_player_move(this_move: i32, player: i32, b: &mut [i32]) -> bool {
    if b[this_move as usize] == 0 {
        match player {
            1 => b[this_move as usize] = 1,
            2 => b[this_move as usize] = 10,
            _ => return false,
        }
        return true;
    } else {
        return false;
    }
}

fn is_open(desired_move: i32, b: &[i32]) -> bool {
    match b[desired_move as usize] {
        0 => return true,
        _ => return false,
    }
}

fn find_an_open(a: i32, b: i32, c: i32, board: &[i32]) -> i32 {
    // Given three integers representing spaces on the board, find the first that
    // is open
    // Wonder if this could be changed to a match statement...
    if is_open(a, board) {
        return a;
    } else if is_open(b, board) {
        return b;
    } else if is_open(c, board) {
        return c;
    } else {
        return 11;
    }
}

fn check_for_win(b: &[i32]) -> bool {
    let sums = calc_sums(b);
    for v in &sums {
        match v {
            &3 => return true,
            &30 => return true,
            _ => continue,
        }
    }
    return false;
}

fn check_if_board_full(b: &[i32]) -> bool {
    let sum: i32 = b.iter().sum();
    match sum {
        45 => return true,
        _ => return false,
    }
}

fn random_pick(b: &[i32]) -> i32 {
    // a number from [-40.0, 13000.0)
    let mut num: i32 = rand::thread_rng().gen_range(0, 8);
    loop {
        if b[num as usize] == 0 {
            break;
        } else {
            num = rand::thread_rng().gen_range(0, 8);
        }
    }
    return num;
}

fn calc_sums(b: &[i32]) -> [i32; 8] {
    let mut sums: [i32; 8] = [0; 8]; // mutable Array of 7 `usize`s, all with value of 0
    sums[0] = b[2] + b[4] + b[6];
    sums[1] = b[0] + b[3] + b[6];
    sums[2] = b[1] + b[4] + b[7];
    sums[3] = b[2] + b[5] + b[8];
    sums[4] = b[0] + b[4] + b[8];
    sums[5] = b[6] + b[7] + b[8];
    sums[6] = b[3] + b[4] + b[5];
    sums[7] = b[0] + b[1] + b[2];
    return sums;
}

fn alfred_pick(b: &[i32]) -> i32 {
    let line = alfred_find_line(b);
    match line {
        0 => find_an_open(2, 4, 6, &b),
        1 => find_an_open(0, 3, 6, &b),
        2 => find_an_open(1, 4, 7, &b),
        3 => find_an_open(2, 5, 8, &b),
        4 => find_an_open(0, 4, 8, &b),
        5 => find_an_open(6, 7, 8, &b),
        6 => find_an_open(3, 4, 5, &b),
        7 => find_an_open(0, 1, 2, &b),
        _ => random_pick(&b),
    }
}

// helper function
fn alfred_find_line(b: &[i32]) -> i32 {
    let sums = calc_sums(b);

    let mut i = 0;

    for v in &sums {
        if v == &20 {
            println!("found a 20 at {}", i);
            return i;
        }
        i = i + 1;
    }

    i = 0;
    for v in &sums {
        if v == &2 {
            println!("found a 2 to block");
            return i;
            // println!("I should never see this!!");
        }
        i = i + 1;
    }

    i = 0;
    for v in &sums {
        if v == &10 {
            println!("found a 10-- I'll take it");
            return i;
            // println!("I should never see this!!");
        }
        i = i + 1;
    }

    println!("picking randomly");
    return random_pick(&b);
}
