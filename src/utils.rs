use macroquad::prelude::*;
use crate::myconst::*;
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

// printlnっぽい文字列描画
pub fn drawtextln(text:&str, pos_x:i32, pos_y:i32, textcol:Color) {
    let left = pos_x as f32 * PANEL_FONT_SIZE;
    let top = pos_y as f32 * PANEL_FONT_SIZE;
    draw_text(text, left, top, PANEL_FONT_SIZE, textcol);
    draw_text(text, left + 1.0, top, PANEL_FONT_SIZE, textcol);
}
