use macroquad::prelude::*;

mod panel;
mod gamemain;
mod keymanager;
use gamemain::GameMain;

#[macroquad::main("Test Window")]
async fn main() {
    let mut game_data = GameMain::new();
    game_data.set_gameinfo(10,10, 5);
    game_data.initial_game();
    loop {
        clear_background(WHITE);
        game_data.draw_table();
        next_frame().await;
    }
}
