use macroquad::prelude::*;
use crate::chkboxmng::ChkBoxMng;
use crate::myconst::*;
use crate::utils::*;
use crate::draw::*;
use crate::SharedSettings;
use crate::txtbox::TextBox;

pub struct TitleMain {						// タイトル画面情報
	mouse_pos: PosTable,                    // マウスカーソル位置
	chkbox: ChkBoxMng<CBTitle>,				// チェックボックス
	setting: Option<SharedSettings>,		// ゲーム設定
	// EDIT 用のテキストボックス
	txtwidth: TextBox,						// 盤面の幅
	txtheight: TextBox,						// 盤面の高さ
	txtbom: TextBox,						// 爆弾の数
}

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl TitleMain {
	//----------------------------------------
	// 初期化
	//----------------------------------------
    pub fn new() -> TitleMain {
		// タイトル情報を生成する
		let mut gm = TitleMain {
			mouse_pos: PosTable { x: 0.0, y: 0.0 },
			chkbox: ChkBoxMng::new(),
			txtwidth: TextBox::new("10",150.0,360.0,70.0,30.0, 5, 35),
			txtheight: TextBox::new("10",150.0,400.0,70.0,30.0, 5, 35),
			txtbom: TextBox::new("15",150.0,440.0,70.0,30.0, 1, 900),
			setting: None,
		};

		// チェックボックスの生成
		gm.create_chkbox();
		
		gm
	}

	//----------------------------------------
	// チェックボックス初期化
	//----------------------------------------
	fn create_chkbox(&mut self) {
		// チェックボックスの基本情報設定
        self.chkbox.set_base(50.0,120.0,250.0, FONT_SIZE_TITLE,
			FONT_SIZE * 1.5,"FFFF00FF", "0000A0A0");

		// 難易度項目
		self.chkbox.add(CBTitle::Easy, String::from("EASY"),false);
		self.chkbox.add(CBTitle::Normal, String::from("NORMAL"),true);
		self.chkbox.add(CBTitle::Hard, String::from("HARD"), false);
		self.chkbox.add(CBTitle::Edit, String::from("EDIT"), false);

		// スタート
		self.chkbox.set_next_pos(PosTable{x:300.0,y:0.0});
		self.chkbox.add(CBTitle::Start, String::from("[START]"), false);
		self.chkbox.view_box(CBTitle::Start, false);
		self.chkbox.set_col(CBTitle::Start, "7777FFFF", "");

		// 終了
		self.chkbox.add(CBTitle::Quit, String::from("[QUIT]"),false);
		self.chkbox.set_col(CBTitle::Quit, "FF7777FF", "");
		self.chkbox.view_box(CBTitle::Quit, false);

		// 設定
		self.chkbox.add(CBTitle::Settings, String::from("[SETTING]"),false);
		self.chkbox.set_col(CBTitle::Settings, "77FFFFFF", "");
		self.chkbox.view_box(CBTitle::Settings, false);

		// 当たり判定表示
		self.chkbox.view_hitbox(false);
	}

	//----------------------------------------
	// 設定画面オブジェクトの保持
	//----------------------------------------
	pub fn setting_obj(&mut self, setting:SharedSettings) {
        self.setting = Some(setting);
    }

	//----------------------------------------
	// タイトル画面制御
	//----------------------------------------
	pub fn titlecontrol(&mut self) -> GameMode {
		// マウス位置の保持
		let (x,y) = mouse_position();
		self.mouse_pos.x = x;
		self.mouse_pos.y = y;

		// 設定画面が開いている場合、設定画面操作のみ行う
		if self.setting.as_ref().unwrap().borrow().is_open() {
			self.setting.as_ref().unwrap().borrow_mut().set_mouse_pos(self.mouse_pos);
			self.setting.as_ref().unwrap().borrow_mut().click();
			return GameMode::Title
		}

		// テキストボックスの処理
		self.txtbox_control();

		// 左クリック処理
		let is_update = self.click_left();
		if !is_update {
			return GameMode::Title;
		}

		// Quit が選択された場合ゲーム終了
		if self.chkbox.get_flg(CBTitle::Quit) {
			return GameMode::Quit
		}

		// Start が選択された場合ゲームに遷移
		if self.chkbox.get_flg(CBTitle::Start) {
			// 内部的にフラグを落としておく（タイトルに戻った時前回の判定で処理されないよう）
			self.chkbox.set_flg(CBTitle::Start, false);
			return GameMode::Game
		}

		// それ以外はタイトル画面継続
		GameMode::Title
	}

	//----------------------------------------
	// テキストボックス（EDIT用）の処理
	//----------------------------------------
	fn txtbox_control(&mut self) {		
		// 入力に応じて爆弾数の上限を設定する
		let width = self.txtwidth.get_value();
		let height = self.txtheight.get_value();
		self.txtbom.set_max(width * height - 1);
		
		// テキストボックス（幅）の制御
		let is_tab = self.txtwidth.update();
		// タブが入力されたらフォーカスを移す
		if is_tab {
			self.txtheight.focus();
			return;
		}

		// テキストボックス（縦）の制御
		let is_tab = self.txtheight.update();
		// タブが入力されたらフォーカスを移す
		if is_tab {
			self.txtbom.focus();			
			return;
		}

		// テキストボックス（爆弾数）の制御
		let is_tab = self.txtbom.update();
		// タブが入力されたらフォーカスを移す
		if is_tab {
			self.txtwidth.focus();			
			return;
		}
	}

	//----------------------------------------
	// 左クリック処理
	// 戻り値：true＝チェックボックスがクリックされた
	//----------------------------------------
	pub fn click_left(&mut self) -> bool {
		if !is_mouse_button_pressed(MouseButton::Left) {
			return false
		}

		// チェックボックスのクリック判定
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
	// 選ばれた難易度に応じて盤面の幅・高さ・爆弾数を返却
	// 戻り値：（幅、高さ、爆弾数）
	//----------------------------------------
	pub fn get_setting(&self) -> (i32,i32,i32) {
		if self.chkbox.get_flg(CBTitle::Easy) {
			(9,9,10)
		} else if self.chkbox.get_flg(CBTitle::Normal) {
			(16,16,40)
		} else if self.chkbox.get_flg(CBTitle::Hard) {
			(30,16,99)
		} else {
			let width = self.txtwidth.get_value();
			let height = self.txtheight.get_value();
			let bom = self.txtbom.get_value();
			(width, height, bom)
		}
	}

	//----------------------------------------
	// 画面描画
	//----------------------------------------
	pub fn draw(&self,myfont:&Font) {
		// 盤面全体を塗りつぶす
		clear_window(LAYOUT_COLOR);

		//--------------------------------------------------
		// タイトル描画
		//--------------------------------------------------
		dr_rect(0.0, 60.0, 700.0, 30.0, 0.0, "0000FFFF","");
		dr_text_ex("Lets MINE SWEEPER", 20.0, 5.0, 70.0,
			&String::from("0000A0FF"),&String::from("FFFFFFFF"), myfont);
		dr_text_ex("'", 150.0, 10.0, 70.0,
			&String::from("0000A0FF"),&String::from("FFFFFFFF"), myfont);
		dr_text_ex("v1.0", 620.0, 40.0, 30.0,
			&String::from("0000A0FF"),&String::from("FFFFFFCC"), myfont);

		//--------------------------------------------------
		// チェックボックス描画
		//--------------------------------------------------
		self.chkbox.draw(myfont);

		//--------------------------------------------------
		// テキストボックス描画（EDIT選択時）
		//--------------------------------------------------
		if self.chkbox.get_flg(CBTitle::Edit) {
			dr_text_ex(" WIDTH", 80.0, 363.0, 20.0,
				&String::from("000000FF"),&String::from("FFFFFFFF"), myfont);
			dr_text_ex("HEIGHT", 80.0, 403.0, 20.0,
				&String::from("000000FF"),&String::from("FFFFFFFF"), myfont);
			dr_text_ex("  BOMB", 80.0, 443.0, 20.0,
				&String::from("000000FF"),&String::from("FFFFFFFF"), myfont);
			self.txtwidth.draw();
			self.txtheight.draw();
			self.txtbom.draw();

			// 推奨爆弾数を表示
			let width = self.txtwidth.get_value();
			let height = self.txtheight.get_value();
			// 盤面の大きさから爆弾の割合を決める
			let par = match width * height {
    			0..=300 => 16.0,
			    _       => 20.7,
			};
			let rec= width * height * (par * 10.0) as i32 / 1000;
			let text = format!("(RECOMMEND:{})", rec);
			dr_text_ex(&text, 230.0, 443.0, 20.0,
				&String::from("000000FF"),&String::from("FFFFFFFF"), myfont);
		}

		//--------------------------------------------------
		// 設定画面描画
		//--------------------------------------------------
		self.setting.as_ref().unwrap().borrow().draw(myfont);

	}
}