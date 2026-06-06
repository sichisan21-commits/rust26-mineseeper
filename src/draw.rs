use macroquad::prelude::*;

// 画面全体を塗りつぶし
pub fn clear_window(col: Color) {
        clear_background(col);
}

// printlnっぽい文字列描画
pub fn drawtextln(text:&str, pos_x:i32, pos_y:i32, size: f32, textcol:Color) {
    let left = pos_x as f32 * size;
    let top = pos_y as f32 * size;
    draw_text(text, left, top, size, textcol);
    draw_text(text, left + 1.0, top, size, textcol);
}
