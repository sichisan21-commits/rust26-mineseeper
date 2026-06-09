use macroquad::prelude::*;
use macroquad::miniquad::window;
use crate::myconst::*;

// 座標データ
pub struct PosTable {
    pub x: f32,
    pub y: f32,
}

//------------------------------
// 画面サイズを設定する
//------------------------------
pub fn set_winsize(width: f32, height: f32) {
    // 必要な画面サイズを求める
    // 最小・最大範囲に補正して画面サイズへ反映
    let win_width = width.clamp(WIN_MIN_X, WIN_MAX_X);
    let win_height = height.clamp(WIN_MIN_Y, WIN_MAX_Y);
    window::set_window_size(win_width as u32, win_height as u32);
}

//------------------------------
// 座標をインデックスへ変換
//------------------------------
pub fn get_index(cursol_x:i32, cursol_y:i32, width:i32, height:i32) -> i32 {
    if cursol_x < 0 || cursol_x >= width ||
       cursol_y < 0 || cursol_y >= height {
        return -1;
       }
    cursol_y * width + cursol_x
}