use macroquad::prelude::*;
use macroquad::text::TextParams;

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
		let border = size * 0.04;
		draw_text(text,pos_x - border, pos_y + offs_y - border, size,bgcol);
		draw_text(text,pos_x + border, pos_y + offs_y - border, size,bgcol);
		draw_text(text,pos_x - border, pos_y + offs_y + border, size,bgcol);
		draw_text(text,pos_x + border, pos_y + offs_y + border, size,bgcol);
	}

	// 文字を描く
	draw_text(text, pos_x - size * 0.01, pos_y + offs_y, size,fgcol);
	draw_text(text, pos_x + size * 0.01, pos_y + offs_y, size,fgcol);
}

//--------------------------------------------------
// 文字列描画
//--------------------------------------------------
pub fn dr_text_ex(text: &str, pos_x:f32, pos_y:f32, size: f32, fg:&str, bg:&str, myfont: &Font) {

	let fgcol = mycol(fg);
	let offs_y = size * 0.85;

	// 背景色の指定がある場合だけ縁取り処理
	if &bg[6..8] != "00" {
		let bgcol = mycol(bg);

		// フォント情報を作成
		let txt_params = TextParams {
			font: Some(myfont),
			font_size: size as u16,
			color: bgcol,
			..Default::default()};

		// 輪郭を描画
		let border = 1.5;
		draw_text_ex(text, pos_x - border, pos_y + offs_y - border, txt_params.clone());
		draw_text_ex(text, pos_x + border, pos_y + offs_y - border, txt_params.clone());
		draw_text_ex(text, pos_x - border, pos_y + offs_y + border, txt_params.clone());
		draw_text_ex(text, pos_x + border, pos_y + offs_y + border, txt_params.clone());
	}

	// 文字を描く
	let txt_params = TextParams {
		font: Some(myfont),
		font_size: size as u16,
		color: fgcol,
		..Default::default()};
	draw_text_ex(text, pos_x, pos_y + offs_y, txt_params.clone());
}

fn mycol(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    let a = u8::from_str_radix(&hex[6..8], 16).unwrap();
    Color::from_rgba(r, g, b, a)
}