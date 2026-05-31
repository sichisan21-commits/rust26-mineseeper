use macroquad::prelude::*;
mod panel;
mod gamemain;
use gamemain::GameMain;

//--------------------------------------------------
// 定数
//--------------------------------------------------
pub const PANEL_WIDTH: f32 = 50.0;
pub const PANEL_HEIGHT: f32 = 50.0;
pub const WALL_LEFT: f32 = 20.0;
pub const WALL_TOP: f32 = 20.0;
//--- 色 ---
// 盤面全体
pub const LAYOUT_COLOR: Color = Color::from_rgba(220, 220, 220, 255);
// パネル
pub const PANEL_COL_CLOSE: Color = Color::from_rgba(230, 230, 230, 255);

//--------------------------------------------------
// main
//--------------------------------------------------
#[macroquad::main("Test Window")]
async fn main() {
    let mut game_data = GameMain::new();
    game_data.set_gameinfo(10,10, 5);
    game_data.initial_game();
    loop {
        clear_background(WHITE);
        game_data.get_cursolpos();
        game_data.draw_table();
        next_frame().await;
    }
}
