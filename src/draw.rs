use macroquad::prelude::*;

//--------------------------------------------------
// 画面全体を塗りつぶし
//--------------------------------------------------
pub fn clear_window(col: Color) {
		clear_background(col);
}

//--------------------------------------------------
// 文字列描画
//--------------------------------------------------
pub fn dr_text(text: &str, pos_x:f32, pos_y:f32, size: f32,
	fg:(u8,u8,u8,u8),bg:(u8,u8,u8,u8)) {

	let fgcol = Color::from_rgba(fg.0, fg.1, fg.2, fg.3);
	let offs_y = size * 0.6;

	// 背景色の指定がある場合だけ縁取り処理
	if bg.3 > 0 {
		let bgcol = Color::from_rgba(bg.0, bg.1, bg.2, bg.3);

		// 輪郭を描画
		let border = size * 0.03;
		draw_text(text,pos_x - border, pos_y + offs_y - border, size,bgcol);
		draw_text(text,pos_x + border, pos_y + offs_y - border, size,bgcol);
		draw_text(text,pos_x - border, pos_y + offs_y + border, size,bgcol);
		draw_text(text,pos_x + border, pos_y + offs_y + border, size,bgcol);
	}

	// 文字を描く
	draw_text(text, pos_x, pos_y + offs_y, size,fgcol);
}
