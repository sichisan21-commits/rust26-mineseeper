use macroquad::prelude::*;
mod panel;
mod gamemain;
use gamemain::GameMain;

//--------------------------------------------------
// 定数
//--------------------------------------------------
pub const PANEL_WIDTH: f32 = 25.0;
pub const PANEL_HEIGHT: f32 = 25.0;
pub const WALL_LEFT: f32 = 20.0;
pub const WALL_TOP: f32 = 20.0;
//--- 色 ---
// 盤面全体
pub const LAYOUT_COLOR: Color = Color::from_rgba(220, 220, 220, 255);
// パネル
pub const PANEL_COL_CLOSE: Color = Color::from_rgba(180, 180, 180, 255);
pub const PANEL_COL_OPEN: Color = Color::from_rgba(220, 220, 220, 255);
pub const PANEL_COL_DANGER: Color = Color::from_rgba(220, 180, 180, 255);
pub const PANEL_COL_SAFETY: Color = Color::from_rgba(180, 220, 220, 255);
pub const PALNE_FONT_SIZE: i32 = 25;

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
