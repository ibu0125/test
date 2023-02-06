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
    println!("\n{} VS. {}",data.player1,data.player2);
}

fn get_input() ->String {
    let mut name = String::new();
    io::stdin().read_line(&mut name).ok();
    return name.trim().to_string();
}

// fn input(text: &str) -> String {
//     println!("{}", text);
//     let mut a = String::new();
//     io::stdin().read_line(&mut a).expect("入力エラー");

//     return a
// }