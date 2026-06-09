use macroquad::prelude::*;

// 画面全体を塗りつぶし
pub fn clear_window(col: Color) {
		clear_background(col);
}

// 文字列描画
pub fn dr_text(text: &str, pos_x:f32, pos_y:f32, size: f32,
	fg:(u8,u8,u8,u8),bg:(u8,u8,u8,u8)) {

	// 色を作成
	let fgcol = Color::from_rgba(fg.0, fg.1, fg.2, fg.3);
	let bgcol = Color::from_rgba(bg.0, bg.1, bg.2, bg.3);

	// 輪郭を描画
	let border = size * 0.04;
	let y_offs = size * 0.6;
	for x in -1..2 {
		draw_text(text,
			pos_x + x as f32 * border,
			pos_y + x as f32 * border + y_offs, size,
			bgcol);
	}
	draw_text(text, pos_x + size * 0.02, pos_y + y_offs, size,fgcol);
	draw_text(text, pos_x - size * 0.02, pos_y + y_offs, size,fgcol);
}

// printlnっぽい文字列描画
pub fn drawtextln(text:&str, pos_x:i32, pos_y:i32, size: f32, textcol:Color) {
	let left = pos_x as f32 * size;
	let top = pos_y as f32 * size;
	draw_text(text, left, top, size, textcol);
	draw_text(text, left + 1.0, top, size, textcol);
}
