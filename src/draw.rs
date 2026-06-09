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
pub fn dr_text(text: &str, pos_x:f32, pos_y:f32, size: f32, fg:&String, bg:&String) {

	let fgcol = mycol(fg);
	let offs_y = size * 0.6;

	// 背景色の指定がある場合だけ縁取り処理
	if &bg[6..8] != "00" {
		let bgcol = mycol(bg);
		
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

fn mycol(hex: &String) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    let a = u8::from_str_radix(&hex[6..8], 16).unwrap();
    Color::from_rgba(r, g, b, a)
}