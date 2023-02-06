#![allow(dead_code)] 
use std::io;

struct Playdata {
    num_play: i32,  //対戦数 
    num_player1_win: i32, // Player1の勝利数 
    num_player2_win: i32, // Player2の勝利数
    num_draw: i32, // 引き分け数
    ave_player1_win: f64, // Player1の勝率
    ave_player2_win: f64, // Player2の勝率
    player1: String, // Player1の名前
    player2: String, // Player2の名前
}

fn main() {
    let mut turn: i32 = 1;
    let mut board: [[i32; 3]; 3] = [[0,0,0],[0,0,0],[0,0,0]];
    let mut data = Playdata{
        num_play: 0,
        num_player1_win: 0,
        num_player2_win: 0,
        num_draw: 0,
        ave_player1_win: 0.0,
        ave_player2_win: 0.0,
        player1: String::new(),
        player2: String::new(),
    };
    println!("player1'name:");
    data.player1=get_input();
    println!("player2'name:");
    data.player2=get_input();
    println!("\n{} VS. {}\n",data.player1,data.player2);
    for i in 1..=9{
        if turn==1 {println!("{}のターンです(o)",data.player1);}else {println!("{}のターンです(x)",data.player2);}
        mark_board(&mut board,turn);
        print_board(&mut board);
    }
}

fn get_input() -> String {
    let mut name = String::new();
    io::stdin().read_line(&mut name).ok();
    return name.trim().to_string();
}

fn get_input_int() -> u32{
    let mut num = String::new();
    io::stdin().read_line(&mut num).ok();
    return num.trim().parse().ok().unwrap();
}

fn print_board(board: &mut [[i32; 3]; 3]) {
    for j in 0..3{
        print!("|");
        for i in 0..3{
            if board[i][j]==0 {print!(" |");}else if board[i][j]==1{print!("o|");}else{print!("x|");}
        }
        print!("\n");
    }
}

fn mark_board(board: &mut [[i32; 3]; 3],turn: i32) {
    let mut position_ver: u32;
    let mut position_hori: u32;
    let mut j=false;

    while j==false {
        println!("横の座標は何ですか？");
        position_hori =get_input_int();
        println!("縦の座標は何ですか？");
        position_ver =get_input_int();
        if position_ver>=0 && position_ver<=3 && position_hori>=0 && position_hori<=3{
            j=true;
        }else{
            println!("範囲外です");
        }
    }
    *board[position_hori as u32][position_ver]= if turn==1 {1} else {-1}
}

// fn input(text: &str) -> String {
//     println!("{}", text);
//     let mut a = String::new();
//     io::stdin().read_line(&mut a).expect("入力エラー");

//     return a
// }