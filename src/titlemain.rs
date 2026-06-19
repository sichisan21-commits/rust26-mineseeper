use macroquad::prelude::*;
use crate::chkboxmng::ChkBoxMng;
use crate::myconst::*;
use crate::utils::*;
use crate::draw::*;
use crate::SharedSettings;

pub struct TitleMain {						// タイトル画面情報
	chkbox: ChkBoxMng<CBTitle>,			// チェックボックス
	mouse_pos: PosTable,                    // マウスカーソル位置
	setting: Option<SharedSettings>,		// ゲーム設定
}

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl TitleMain {								// タイトル画面
	//----------------------------------------
	// 初期化
	//----------------------------------------
    pub fn new() -> TitleMain {
        let mut gm = TitleMain {

			chkbox: ChkBoxMng::new(),
			mouse_pos: PosTable { x: 0.0, y: 0.0 },
			setting: None,
		};
		// チェックボックス
		gm.create_chkbox();

		gm
	}

	//----------------------------------------
	// チェックボックス初期化
	//----------------------------------------
	fn create_chkbox(&mut self) {
		// チェックボックス作成
        self.chkbox.set_base(50.0,120.0,250.0, FONT_SIZE*2.0, FONT_SIZE*1.5,"FFFF00FF", "0000A0A0");
		self.chkbox.add(CBTitle::Easy, String::from("EASY"),false);
		self.chkbox.add(CBTitle::Normal, String::from("NORMAL"),true);
		self.chkbox.add(CBTitle::Hard, String::from("HARD"), false);
		self.chkbox.add(CBTitle::Edit, String::from("EDIT"), false);
		// EDITは現在未対応
		self.chkbox.set_active_flg(CBTitle::Edit, false);

		// スタート／終了／設定
		// START
		self.chkbox.add(CBTitle::Start, String::from("[START]"), false);
		self.chkbox.view_box(CBTitle::Start, false);
		self.chkbox.set_col(CBTitle::Start, "7777FFFF", "");
		self.chkbox.set_offs(CBTitle::Start,300.0, -180.0);
		// QUIT
		self.chkbox.add(CBTitle::Quit, String::from("[QUIT]"),false);
		self.chkbox.set_col(CBTitle::Quit, "FF7777FF", "");
		self.chkbox.view_box(CBTitle::Quit, false);
		// Setting
		self.chkbox.add(CBTitle::Settings, String::from("[SETTING]"),false);
		self.chkbox.set_col(CBTitle::Settings, "77FFFFFF", "");
		self.chkbox.view_box(CBTitle::Settings, false);

		self.chkbox.view_hitbox(false);
	}

	//----------------------------------------
	// 設定画面オブジェクトの設定
	//----------------------------------------
	pub fn setting_obj(&mut self, setting:SharedSettings) {
        self.setting = Some(setting);
    }

	//----------------------------------------
	// タイトル制御
	//----------------------------------------
	pub fn titlecontrol(&mut self) -> GameMode {
		// マウス位置の更新
		let (x,y) = mouse_position();
		self.mouse_pos.x = x;
		self.mouse_pos.y = y;

		// 設定画面が開いている場合、設定画面操作のみ
		if self.setting.as_ref().unwrap().borrow().is_open() {
			self.setting.as_ref().unwrap().borrow_mut().set_mouse_pos(self.mouse_pos);
			self.setting.as_ref().unwrap().borrow_mut().click();
			return GameMode::Title
		}

		// 左クリック処理
		let is_update = self.click_left();
		if !is_update {
			return GameMode::Title;
		}

		// Quit が選択された場合終了
		if self.chkbox.get_flg(CBTitle::Quit) {
			return GameMode::Quit
		}

		// Start が選択された場合ゲームに遷移
		if self.chkbox.get_flg(CBTitle::Start) {
			// 内部的にフラグを落としておく
			self.chkbox.set_flg(CBTitle::Start, false);
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
				CBTitle::Start => {
					true
				}
				// 設定が押された場合はメニュー表示
				CBTitle::Settings => {
					self.setting.as_ref().unwrap().borrow_mut().open();
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
		if self.chkbox.get_flg(CBTitle::Easy) {
			(9,9,10)
		} else if self.chkbox.get_flg(CBTitle::Normal) {
			(16,16,40)
		} else {
			(30,16,99)
		}
	}

	//----------------------------------------
	// 画面描画
	//----------------------------------------
	pub fn draw(&self,myfont:&Font) {
		// 盤面全体を塗りつぶす
		clear_window(LAYOUT_COLOR);

		dr_rect(0.0, 60.0, 700.0, 30.0, 0.0, "0000FFFF","");

		dr_text_ex("Lets MINE SWEEPER", 20.0, 5.0, 70.0,
			&String::from("0000A0FF"),&String::from("FFFFFFFF"), myfont);
		dr_text_ex("'", 150.0, 10.0, 70.0,
			&String::from("0000A0FF"),&String::from("FFFFFFFF"), myfont);
		dr_text_ex("v1.0", 620.0, 40.0, 30.0,
			&String::from("0000A0FF"),&String::from("FFFFFFCC"), myfont);

		// チェックボックスを描く
		self.chkbox.draw(myfont);

		// 設定画面を描画
		self.setting.as_ref().unwrap().borrow().draw(myfont);

	}
}