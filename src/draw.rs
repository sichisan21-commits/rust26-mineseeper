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
// text＝文字列
// pos_x＝表示位置（横）
// pos_y＝表示位置（縦）
// size＝フォントサイズ
// fg＝文字色
// bg＝文字縁色
//--------------------------------------------------
pub fn dr_text(text: &str, pos_x:f32, pos_y:f32, size: f32, fg:&str, bg:&str) {

	let fgcol = mycol(fg);
	let offs_y = size * 0.6;

	// 背景色の指定がある場合だけ縁取り処理
	if bg.len() > 0 && &bg[6..8] != "00" {
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
// text＝文字列
// pos_x＝表示位置（横）
// pos_y＝表示位置（縦）
// size＝フォントサイズ
// fg＝文字色
// bg＝文字縁色
// myfont＝フォントオブジェクト
//--------------------------------------------------
pub fn dr_text_ex(text: &str, pos_x:f32, pos_y:f32, size: f32, fg:&str, bg:&str, myfont: &Font) {
	let fgcol = mycol(fg);
	let offs_y = size * 1.0;

	// 背景色の指定がある場合だけ縁取り処理
	if bg.len() > 0 && &bg[6..8] != "00" {
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

//--------------------------------------------------
// 矩形の描画
// left＝左位置
// top＝上位置
// width＝幅
// height＝高さ
// tick＝輪郭幅（0.0指定で「なし」）
// col＝塗りつぶし色
// bcol＝輪郭色（tickで0.0を指定した場合参照されない）
//--------------------------------------------------
pub fn dr_rect(left:f32, top:f32, width:f32, height:f32, tick: f32, col:&str, bcol: &str) {
	// 塗りつぶし色の指定がある場合だけ塗り潰し
	if col.len() >= 8 && &col[6..8] != "00" {
		draw_rectangle(left, top, width, height, mycol(col));
	}
	// 線幅の制定がある時だけ枠表示
	if tick > 0.0 {
		draw_rectangle_lines(left, top, width, height, tick, mycol(bcol));
	}
}

//--------------------------------------------------
// RRGGBBAA形式文字列をColor形式へ変換
//--------------------------------------------------
fn mycol(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    let a = u8::from_str_radix(&hex[6..8], 16).unwrap();
    Color::from_rgba(r, g, b, a)
}