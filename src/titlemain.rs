use macroquad::prelude::*;
use crate::chkbox::ChkBoxMng;
use crate::myconst::*;
use crate::utils::*;
use crate::draw::*;

pub struct TitleMain <'a> {					// タイトル画面情報
	chkbox: ChkBoxMng<'a,ChkBoxTitle>,      // チェックボックス
	mouse_pos: PosTable,                    // マウスカーソル位置
	myfont: &'a Font,						// フォント情報
}

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl<'a> TitleMain<'a> {								// タイトル画面
	//----------------------------------------
	// 初期化
	//----------------------------------------
    pub fn new(myfont: &'a Font) -> TitleMain<'a> {
        let mut gm = TitleMain {
			chkbox: ChkBoxMng::new(myfont),
			mouse_pos: PosTable { x: 0.0, y: 0.0 },
			myfont,
		};

		// チェックボックス作成
        gm.chkbox.set_base(50.0,120.0,250.0, FONT_SIZE*2.0, FONT_SIZE*1.5,"FFFF00FF", "0000A0A0");
		gm.chkbox.add(ChkBoxTitle::Easy, String::from("EASY"),false);
		gm.chkbox.add(ChkBoxTitle::Normal, String::from("NORMAL"),true);
		gm.chkbox.add(ChkBoxTitle::Hard, String::from("HARD"), false);
		gm.chkbox.add(ChkBoxTitle::Edit, String::from("EDIT"), false);
		// EDITは現在未対応
		gm.chkbox.active(ChkBoxTitle::Edit, false);

		// スタート／終了
		// START
		gm.chkbox.add(ChkBoxTitle::Start, String::from("[START]"), false);
		gm.chkbox.view_box(ChkBoxTitle::Start, false);
		gm.chkbox.set_col(ChkBoxTitle::Start, "7777FFFF", "");
		gm.chkbox.set_offs(ChkBoxTitle::Start,300.0, -180.0);
		// QUIT
		gm.chkbox.add(ChkBoxTitle::Quit, String::from("[QUIT]"),false);
		gm.chkbox.set_col(ChkBoxTitle::Quit, "FF7777FF", "");
		gm.chkbox.view_box(ChkBoxTitle::Quit, false);

		gm.chkbox.view_hitbox(false);

		gm
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

		// Quit が選択された場合終了
		if self.chkbox.get_flg(ChkBoxTitle::Quit) {
			return GameMode::Quit
		}

		// Start が選択された場合ゲームに遷移
		if self.chkbox.get_flg(ChkBoxTitle::Start) {
			// 内部的にフラグを落としておく
			self.chkbox.set_flg(ChkBoxTitle::Start, false);
			return GameMode::Game
		}

		// それ以外はタイトル画面継続
		GameMode::Title
	}

	//----------------------------------------
	// 左クリック処理
	//----------------------------------------
	pub fn click_left(&mut self) -> bool {
		if !is_mouse_button_pressed(MouseButton::Left) {
			return false
		}

		// チェックボックスのクリック処理
		if let Some((kind, _flg)) = self.chkbox.click(self.mouse_pos.x, self.mouse_pos.y) {
			match kind {
				// スタートが押された場合は何もせず真を返す
				ChkBoxTitle::Start => {
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
		if self.chkbox.get_flg(ChkBoxTitle::Easy) {
			(9,9,10)
		} else if self.chkbox.get_flg(ChkBoxTitle::Normal) {
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

		draw_rectangle(0.0, 60.0, 700.0, 20.0, BLUE);

		dr_text_ex("Lets MINE SWEEPER", 0.0, 10.0, 70.0,
			&String::from("0000A0FF"),&String::from("FFFFFFFF"), self.myfont);
		dr_text_ex("'", 130.0, 0.0, 70.0,
			&String::from("0000A0FF"),&String::from("FFFFFFFF"), self.myfont);
		dr_text_ex("v1.0", 600.0, 40.0, 30.0,
			&String::from("0000A0FF"),&String::from("FFFFFFCC"), self.myfont);

		// チェックボックスを描く
		self.chkbox.draw();
	}
}