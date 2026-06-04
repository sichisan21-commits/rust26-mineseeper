mod myconst;
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

    let width = 20;
    let height = 20;
    let bom_num = width * height * 15 /100;

    let mut game_data = GameMain::new();
    game_data.set_gameinfo(height, width, bom_num);
    game_data.initial_game();
    loop {
        clear_background(WHITE);
        game_data.playcontrol();
        game_data.draw_table();
        next_frame().await;
    }
}
