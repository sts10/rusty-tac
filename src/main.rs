use std::io;
use std::rand::{task_rng, Rng};

fn main() {
    let mut game_over = false;
    // let game_over = false;
    let mut turn_number = 1;
    let mut board = [0,0,0, 0,0,0, 0,0,0];


    while game_over != true {
        present_board(&board);
        let player;
        if turn_number % 2 == 1{
            player = 1;
        } else {
            player = 2;
        }

        println!("Player {}'s turn", player);
        let current_move = get_int_from_input();
        println!("I got {}", current_move);
        execute_player_move(current_move, player, &mut board);
        if check_for_win(&board)  {
            println!("Player {} wins!", player);
            game_over = true;
        } else {
            turn_number = turn_number + 1;
        }
    }
}

// Figuring out how pass this array was a BITCH. Errors, both intentional (board[2].what_the_fuck_type_is_this;) 
// and not, kept referring to "integer" or "Integer" rather than i32. Using "int" threw unhelpful
// error
fn present_board(b: &[i32]){
   // println!("board is {:?}", b);
   let mut i = 0;
   while i < b.len() {
       match b[i] {
           // if empty, print the number that a user would enter to move
           // to this space
           0 => print!("{}",i),
           1 => print!("X"),
           2 => print!("O"),
           10 => print!("O"),
           _ => break,
       }
       // and now, some decorators
       if i > 0 && (i+1) % 3 == 0{
           print!("\n");
       } else{
           print!(" | ");
       }
       i = i + 1;
   }
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
        None => return 11
    };
}

// https://play.rust-lang.org/?gist=182dc2ad8763bc3fa683d52749e202b4&version=stable
// woof this was a beast. Had to go to the irc channel. Eventually, the answer was to 
// transfer ownership, rather than a reference, to current_move and player; and to 
// convert this_move from an i32 to a usize with `as` when you want to use it as an index
fn execute_player_move(current_move: i32, player: i32, b: &mut [i32]) {
    let this_move = current_move;
    if player == 1{
        b[this_move as usize] = 1;
    } else if player == 2{
        b[this_move as usize] = 10;
    }
}

fn check_for_win(b: &[i32]) -> bool {
    let mut sums: [i32; 8] = [0; 8]; // mutable Array of 7 `usize`s, all with value of 0
    sums[0] = b[2]+b[4]+b[6];
    sums[1] = b[0]+b[3]+b[6];
    sums[2] = b[1]+b[4]+b[7];
    sums[3] = b[2]+b[5]+b[8];
    sums[4] = b[0]+b[4]+b[8];
    sums[5] = b[6]+b[7]+b[8];
    sums[6] = b[3]+b[4]+b[5];
    sums[7] = b[0]+b[1]+b[5];

    for v in &sums {
        match v {
            &3 => return true,
            &30 => return true,
            _ => continue,
        }
    }
    return false;
}

fn ed_pick(b: &[i32]) -> i32 {
// a number from [-40.0, 13000.0)
    while true {
        let num: i32 = task_rng().gen_range(0, 8);
        if b[num as usize] == 0{
            return num;
        }
    }
}
/*
fn calc_sums(b: &[i32]) -> [i32; 8] {
    let mut sums: [i32; 8] = [0; 8]; // mutable Array of 7 `usize`s, all with value of 0
    sums[0] = b[2]+b[4]+b[6];
    sums[1] = b[0]+b[3]+b[6];
    sums[2] = b[1]+b[4]+b[7];
    sums[3] = b[2]+b[5]+b[8];
    sums[4] = b[0]+b[4]+b[8];
    sums[5] = b[6]+b[7]+b[8];
    sums[6] = b[3]+b[4]+b[5];
    sums[7] = b[0]+b[1]+b[5];
    return sums;
}

fn alfred_pick(b: &[i32]) -> i32 {
  let sums = calc_sums(b);

  for v in &sums {
      match v {
          &20 -> break;

      }
  }
}
*/
