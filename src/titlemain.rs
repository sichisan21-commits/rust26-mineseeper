use macroquad::prelude::*;
use crate::chkbox::ChkBoxMng;
use crate::myconst::*;
use crate::utils::*;
use crate::draw::*;

pub struct TitleMain {
	chkbox: ChkBoxMng,                      // チェックボックス
	mouse_pos: PosTable,                    // マウスカーソル位置
}

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl TitleMain {                                // タイトル画面
	//----------------------------------------
	// 初期化
	//----------------------------------------
	pub fn new() -> TitleMain {
		let mut tm = TitleMain {
			chkbox: ChkBoxMng::new(),
			mouse_pos: PosTable{ x:0.0, y:0.0},
		};

		// チェックボックス作成
		let pos_x = 30.0;
		let mut pos_y = 30.0;
		let fgcol: (u8,u8,u8,u8) = (255,255,0,255);
		let bgcol: (u8,u8,u8,u8) = (0,0,0,255);
		let fontsize = 60.0;
		let offs = 60.0;

		pos_y += offs; tm.chkbox.add(
			ChkBoxType::Easy, String::from("EASY"),
			pos_x, pos_y, fontsize, fgcol, bgcol, false);
		pos_y += offs; tm.chkbox.add(
			ChkBoxType::Normal, String::from("NORMAL"),
			pos_x, pos_y, fontsize, fgcol, bgcol, true);
		pos_y += offs; tm.chkbox.add(
			ChkBoxType::Hard, String::from("HARD"),
			pos_x, pos_y, fontsize, fgcol, bgcol, false);
		pos_y += offs; tm.chkbox.add(
			ChkBoxType::Edit, String::from("EDIT"),
			pos_x, pos_y, fontsize, fgcol, bgcol, false);

		// START
		let mut pos_y = 30.0;
		let pos_x = 300.0;
		pos_y += offs; tm.chkbox.add(
			ChkBoxType::Start, String::from("[START]"),
			pos_x, pos_y, fontsize, (0,255,0,255), bgcol, false);
		tm.chkbox.view_box(ChkBoxType::Start, false);
			// QUIT
		pos_y += offs; tm.chkbox.add(
			ChkBoxType::Quit, String::from("[QUIT]"),
			pos_x, pos_y, fontsize, (255,0,0,255), bgcol,false);
		tm.chkbox.view_box(ChkBoxType::Quit, false);

		tm
	}

	//----------------------------------------
	// 初期設定
	//----------------------------------------
	pub fn initial(&mut self)  {
		self.chkbox.clear_flg();
		self.chkbox.set_flg(ChkBoxType::Normal, true);
	}

	//----------------------------------------
	// タイトル制御
	//----------------------------------------
	pub fn titlecontrol(&mut self) -> GameMode {
		// マウス位置の更新
		let (x,y) = mouse_position();
		self.mouse_pos.x = x;
		self.mouse_pos.y = y;

		// 左クリック処理
		let is_update = self.click_left();
		if !is_update {
			return GameMode::Title;
		}
		println!("Easy={}", self.chkbox.get_flg(ChkBoxType::Easy));

		println!("Normal={}", self.chkbox.get_flg(ChkBoxType::Normal));
		println!("Hard={}", self.chkbox.get_flg(ChkBoxType::Hard));

		// Quit が選択された場合終了
		if self.chkbox.get_flg(ChkBoxType::Quit) {
			return GameMode::Quit
		}

		// Start が選択された場合ゲームに遷移
		if self.chkbox.get_flg(ChkBoxType::Start) {
			return GameMode::Game
		}

		// それ以外はタイトル画面継続
		GameMode::Title
	}

	//----------------------------------------
	// 左クリック処理
	//----------------------------------------
	pub fn click_left(&mut self) -> bool {
		let mut  is_update = false;
		if !is_mouse_button_pressed(MouseButton::Left) {
			return is_update;
		}

		// チェックボックスのクリック処理
		if let Some((kind, _flg)) = self.chkbox.click(self.mouse_pos.x, self.mouse_pos.y) {
			match kind {
				// スタートが押された場合は何もせず真を返す
				ChkBoxType::Start => {
					true
				}
				// それ以外はそのチェックボックスだけをオンにする
				_ => {
					// 対象のチェックボックスだけオン
					self.chkbox.clear_flg();
					self.chkbox.set_flg(kind, true);
					true
				}
			}
		} else {
			false
		}
	}

	//----------------------------------------
	// タイトル制御
	//----------------------------------------
	pub fn get_setting(&self) -> (i32,i32,i32) {
		println!("Easy={}", self.chkbox.get_flg(ChkBoxType::Easy));
		println!("Normal={}", self.chkbox.get_flg(ChkBoxType::Normal));
		if self.chkbox.get_flg(ChkBoxType::Easy) {
			(9,9,10)
		} else if self.chkbox.get_flg(ChkBoxType::Normal) {
			(16,16,40)
		} else {
			(30,16,99)
		}
	}

	//----------------------------------------
	// 画面描画
	//----------------------------------------
	pub fn draw(&self) {
		// 盤面全体を塗りつぶす
		clear_window(LAYOUT_COLOR);
		self.chkbox.draw();
	}
}