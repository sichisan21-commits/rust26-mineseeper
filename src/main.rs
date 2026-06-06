mod myconst;
mod draw;
mod chkbox;
mod utils;
mod panel;
mod gamemain;
mod gametable;

use macroquad::prelude::*;
use gamemain::GameMain;

//--------------------------------------------------
// main
//--------------------------------------------------
#[macroquad::main("Test Window")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let width = 10;
    let height = 10;
    let bom_num = width * height * 10 /100;

    let mut game_data = GameMain::new();
    game_data.set_gameinfo(width, height, bom_num);
    game_data.initial_game();
    loop {
        // ゲーム制御
        game_data.playcontrol();
        game_data.draw();
        next_frame().await;
    }
}
