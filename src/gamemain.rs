use macroquad::prelude::*;
use crate::draw::*;
use crate::chkboxmng::ChkBoxMng;
use crate::utils::*;
use crate::myconst::*;
use crate::gametable::GameTable;
use crate::gametable::MyCursol;
use crate::panel::Panel;
use crate::inference::InfTable;
use crate::SharedSettings;

struct TableInfo {							// 盤面の情報
	width: i32,                             // 盤面の幅
	height: i32,                            // 盤面の高さ
	bom_num: i32,                           // 爆弾の数
	table: GameTable,                       // 盤面テーブル
	offs: Vec2,                             // 画面オフセット
	zoom: Vec2,                             // 画面倍率
}

struct MyTime {								// ゲーム内の時間制御
	gamewait: f64,							// 入力を受け付けない時間 
	waitst: f64,							// 受け付けない時間の開始
	playst: f64,							// プレイ開始時刻
	played: f64,							// プレイ終了時刻
}

struct MouseTbl {
	pos: PosTable,							// マウス座標
	lefton: bool,							// 左クリックオン
	righton: bool,							// 右クリックオン
	lefton_now: bool,						// 今、右クリックが押されたか
	righton_now: bool,						// 今、左クリックが押されたか
	leftoff_now: bool,						// 今、右クリックが離されたか
	rightoff_now: bool,						// 今、左クリックが離されたか
}

pub struct GameMain {						// ゲームメイン情報
	stat: GameStat,							// ゲームの状態
	screen: Vec2,							// ウインドウサイズ
	mouse: MouseTbl,	   	 	            // マウスカーソル位置
	cursol: MyCursol,                       // カーソル位置
	death_cnt: i32,							// 死んだ回数
	tm: MyTime,								// 時刻関連
	tb: TableInfo,                       	// 盤面情報
	chkbox: ChkBoxMng<CBGame>,				// 自作チェックボックス
	setting: Option<SharedSettings>,		// ゲーム設定
}

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl GameMain {
	//------------------------------
	// 初期化
	//------------------------------
	pub fn new () -> GameMain {

		// 生成
		let mut gm = GameMain {
			stat: GameStat::Ready,
			screen: Vec2 {x: screen_width(), y: screen_height()},
			tm: MyTime {gamewait: 0.0, waitst: 0.0, playst: 0.0, played: 0.0},
			death_cnt: 0,
			mouse: MouseTbl{
				pos: PosTable{x:0.0,y:0.0},
				lefton: false, lefton_now: false, leftoff_now: false,
				righton: false, righton_now: false, rightoff_now: false,
			},
			cursol: MyCursol {x: -1, y: -1, index: -1},
			tb: TableInfo {width: 0, height: 0, bom_num: 0,
				table: GameTable::new(0,0,0),
				offs: Vec2 { x: WALL_LEFT, y: WALL_TOP },
				zoom: Vec2 { x: MAX_ZOOMX, y: MAX_ZOOMY},
			},
			chkbox: ChkBoxMng::new(),
			setting: None,
		};

		gm.mk_chkbox();

		// 情報を返却
		gm
	}


	//------------------------------
	// チェックボックスを作成する
	//------------------------------
	fn mk_chkbox(&mut self) {
		// チェックボックス作成
        self.chkbox.set_base(10.0,220.0,160.0, 40.0, 20.0,"000000FF", "000077FF");
		// リセットボタン作成
		self.chkbox.add(CBGame::Reset, String::from("[RESET]"), false);
		self.chkbox.set_col(CBGame::Reset, "FFFF77FF","");
		self.chkbox.view_box(CBGame::Reset, false);
		// 設定ボタン作成
		self.chkbox.add(CBGame::Settings, String::from("[SETTINGS]"), false);
		self.chkbox.set_col(CBGame::Settings, "77FFFFFF","");
		self.chkbox.view_box(CBGame::Settings, false);
		// タイトルへ戻るボタン作成
		self.chkbox.add(CBGame::Title, String::from("[TITLE]"), false);
		self.chkbox.set_col(CBGame::Title, "FF7777FF","");
		self.chkbox.view_box(CBGame::Title, false);
		// 操作方法
		self.chkbox.add(CBGame::HowTo, String::from("[HOW TO]"), false);
		self.chkbox.set_col(CBGame::HowTo, "FFFFFFFF","");
		self.chkbox.view_box(CBGame::HowTo, false);
		// 値判定表示の指定
		self.chkbox.view_hitbox(false);
	}

	//----------------------------------------
	// 設定画面オブジェクトの設定
	//----------------------------------------
	pub fn setting_obj(&mut self, setting:SharedSettings) {
        self.setting = Some(setting);
    }

	//------------------------------
	// ゲームの情報の設定
	// width＝盤面の幅
	// height＝満面の高さ
	// bom_num＝爆弾の数
	//------------------------------
	pub fn set_gameinfo(&mut self, width: i32, height: i32, bom_num: i32) {
		self.tb.width = width;
		self.tb.height = height;
		self.tb.bom_num = bom_num;
		self.set_winsize();
	}

	//------------------------------
	// 盤面の初期化
	//------------------------------
	pub fn initial_game(&mut self, wait: f64) {

		// 盤面を初期化する
		self.tb.table = GameTable::new(self.tb.width, self.tb.height, self.tb.bom_num);
		self.tb.table.initial();
		self.set_tablepos();

		// 待ち時間の指定があるなら設定
		self.tm.gamewait = wait;
		if wait != 0.0 {
			self.tm.waitst = get_time();
		}

		// 死んだ回数をリセット
		self.death_cnt = 0;

		// クリック待ちへ遷移
		self.stat = GameStat::Ready;
		self.setting.as_ref().unwrap().borrow_mut().set_playing_flg(false);
	}

	//------------------------------
	// 入力制御
	//------------------------------
	pub fn playcontrol(&mut self) -> GameMode {
		//--------------------------------------------------
		// 事前処理
		//--------------------------------------------------
		// 戻り値を初期化
		let ret_code = GameMode::Game;

		//更新有無を初期化
		let mut is_update = false;

		// マウスの情報を取得
		self.get_mouse();

		// 待ち時間が設定されている場合、時間消化までなにもしない
		if self.tm.gamewait != 0.0 {
			if get_time() - self.tm.waitst < self.tm.gamewait {
				return ret_code;
			}
			self.tm.gamewait = 0.0;
		}

		//--------------------------------------------------
		// 設定画面処理（表示中なら）
		//--------------------------------------------------
		if self.setting.as_ref().unwrap().borrow().is_open() {
			self.setting_control();
			// 設定画面を継続している場合、以降の処理は行わない
			if self.setting.as_ref().unwrap().borrow().is_open() {
				return ret_code
			}
			is_update = true;
		}

		//--------------------------------------------------
		// 操作説明画面処理（表示中なら）
		//--------------------------------------------------
		if self.chkbox.get_flg(CBGame::HowTo) {
			// 左クリックされたら閉じる
			if self.mouse.lefton_now {
				self.chkbox.set_flg(CBGame::HowTo, false);
				self.mouse_click_clear();
			}
			return ret_code
		}

		//--------------------------------------------------
		// メニュー（チェックボックス）処理
		//--------------------------------------------------
		self.chkbox_click();

		// リセットボタンが選択されたら盤面をリセット
		if self.chkbox.get_flg(CBGame::Reset) {
			// 内部的にフラグを落としておく
			self.chkbox.set_flg(CBGame::Reset, false);
			self.initial_game(START_WAIT);
			self.stat = GameStat::Ready;
			return ret_code
		}
		
		// 「タイトルへ」が選択されたらタイトルへ戻る
		if self.chkbox.get_flg(CBGame::Title) {
			// 内部的にフラグを落としておく
			self.chkbox.set_flg(CBGame::Title, false);
			self.setting.as_ref().unwrap().borrow_mut().set_playing_flg(false);
			return GameMode::Title;
		}

		//--------------------------------------------------
		// 盤面処理
		//--------------------------------------------------
		// ゲームが開始されているならプレイ時間更新
		if self.stat == GameStat::Playing {
			self.tm.played = get_time();
		}

		// マウス移動処理
		self.mouse_move();

		// キーボード入力処理
		self.keycontrol();

		// マウスクリック処理
		is_update |= self.click_tbl_left();
		is_update |= self.click_tbl_right();

		// 更新が発生していない場合処理を終える
		if !is_update || self.stat != GameStat::Playing {
			return ret_code
		}

		//----------------------------------------
		// 以降、盤面に更新があった場合の処理
		//----------------------------------------
		// 今の盤面を保存する
		self.tb.table.undo_push();

		// クリア条件を満たした場合
		let close_num = self.tb.width * self.tb.height - self.tb.bom_num - self.tb.table.get_opennum() as i32;
		if close_num == 0 {
			self.stat = GameStat::Success;
			self.mouse_click_clear();
		}

		// 爆弾が開かれた場合
		if self.tb.table.open_bomnum() > 0 {
			self.stat = GameStat::Failed;
			self.mouse_click_clear();
			self.death_cnt += 1;
		} else {
			// アシスト機能を実施
			self.assist();
		}

		// 状態を返却する
		ret_code
	}

	//------------------------------
	// 設定画面制御
	//------------------------------
	fn setting_control(&mut self) {
		// マウス位置を伝える
		self.setting.as_ref().unwrap().borrow_mut().set_mouse_pos(self.mouse.pos);

		// クリック判定を行う
		self.setting.as_ref().unwrap().borrow_mut().click();

		// 設定画面操作後も設定画面が閉じていない場合処理を抜ける
		if self.setting.as_ref().unwrap().borrow().is_open() {
			return
		}

		//--------------------------------------------------
		// 以降、設定画面を閉じられたときの処理
		//--------------------------------------------------
		// 青旗がオフになっているなら青旗をクリアする
		if !self.get_chkbox_flg(CBSetting::UseBlueFlg) {
			self.tb.table.clear_blue_flag();				
		}

		// 閉じられた際のクリックはゲームに伝搬させない
		self.mouse_click_clear();
	}

	//------------------------------
	// マウスの状態取得
	//------------------------------
	fn get_mouse(&mut self) {
		//--------------------------------------------------
		// マウス位置の取得
		//--------------------------------------------------
		let (x,y) = mouse_position();
		self.mouse.pos.x = x;
		self.mouse.pos.y = y;

		//--------------------------------------------------
		// 左クリック保持
		//--------------------------------------------------
		self.mouse.lefton_now = false;
		self.mouse.leftoff_now = false;
		if is_mouse_button_pressed(MouseButton::Left) {
			// 今、左クリックが押されたなら、クリックフラグオン
			if !self.mouse.lefton {
				self.mouse.lefton_now = true;
			}
			self.mouse.lefton = true;
		} else if is_mouse_button_released(MouseButton::Left) {
			// 今、左クリックが離されたなら、クリックフラグオン
			if self.mouse.lefton {
				self.mouse.leftoff_now = true;
			}
			self.mouse.lefton = false;
		}

		//--------------------------------------------------
		// 右クリック保持
		//--------------------------------------------------
		self.mouse.righton_now = false;
		self.mouse.rightoff_now = false;
		if is_mouse_button_pressed(MouseButton::Right) {
			// 今、右クリックが押されたなら、クリックフラグオン
			if !self.mouse.righton {
				self.mouse.righton_now = true;
			}
			self.mouse.righton = true;
		} else if is_mouse_button_released(MouseButton::Right) {
			// 今、右クリックが離されたなら、クリックフラグオン
			if self.mouse.righton {
				self.mouse.rightoff_now = true;
			}
			self.mouse.righton = false;
		}
	}

	//----------------------------------------
	// マウスクリックをクリアする
	//----------------------------------------
	fn mouse_click_clear(&mut self) {
		self.mouse.lefton = false;
		self.mouse.lefton_now = false;
		self.mouse.leftoff_now = false;
		self.mouse.righton = false;
		self.mouse.righton_now = false;
		self.mouse.rightoff_now = false;
	}

	//------------------------------
	// アシスト機能
	//------------------------------
	fn assist(&mut self) {

		// アシストオフならフラグをクリアして終了
		let bold_flg = self.get_chkbox_flg(CBSetting::BoldFlg);
		let inference_flg = self.get_chkbox_flg(CBSetting::Inference);
		if !bold_flg && !inference_flg {
			self.tb.table.assist_clear();
			return
		}

		// 推論ロジックへテーブルのコピーを渡す
		let edit_table = self.tb.table.tbl_backup();
		let mut inftbl = InfTable::new(edit_table,self.tb.width, self.tb.height);

	    // 太字処理か推論処理かで処理
		if bold_flg {
			// 強調表示
			let safe_on = self.get_chkbox_flg(CBSetting::BoldSafeOn);
			inftbl.set_bold(safe_on);
		} else {
			// 推論処理
			let safe_on = self.get_chkbox_flg(CBSetting::SafeOn);
			let dang_on = self.get_chkbox_flg(CBSetting::DangOn);
			let believe_flg = self.get_chkbox_flg(CBSetting::BelieveFlag);
			inftbl.inference(safe_on, dang_on, believe_flg);
		}

		// 処理結果を現在のテーブルへフィードバック
		let edit_table = inftbl.get_table();
		self.tb.table.tbl_restore(edit_table);
	}

	//------------------------------
	// マウス位置に合わせて画面更新
	//------------------------------
	fn mouse_move(&mut self) -> bool {
		let mut is_update = false;

		//--------------------------------------------------
		// マウス位置の取得と盤面への反映
		//--------------------------------------------------
		// 画面サイズの取得
		self.screen.x = screen_width();
		self.screen.y = screen_height();
	
		// 盤面にマウス位置を反映
		let tablepos = Vec2 {
			x: (self.mouse.pos.x - self.tb.offs.x) * (1.0 / self.tb.zoom.x),
			y: (self.mouse.pos.y - self.tb.offs.y) * (1.0 / self.tb.zoom.y),
		};
		let cursol = self.tb.table.set_mousepos(tablepos);

		// DRAG OPEN 有効で押しっぱなしの場合はスクロール制御しない
		if self.mouse.lefton && self.get_chkbox_flg(CBSetting::DragOpen) {
			return is_update;
		}

		//--------------------------------------------------
		// スクロール制御
		//--------------------------------------------------
		// 盤面のリアルサイズを求める
		let real_width = self.tb.width as f32 * PANEL_WIDTH * self.tb.zoom.x;
		let real_height = self.tb.height as f32 * PANEL_HEIGHT * self.tb.zoom.y;

		// 画面からはみ出すサイズを求める
		let over_size_x = real_width + WALL_LEFT + WALL_RIGHT - self.screen.x;
		let over_size_y = real_height + WALL_TOP + WALL_BOTTOM - self.screen.y;

		// カーソルがある程度進んだらスクロールを開始する
		let mousepos_x = (self.mouse.pos.x - WALL_LEFT - SCROLL_LEFT).max(0.0);
		let mousepos_y = (self.mouse.pos.y - WALL_TOP - SCROLL_TOP).max(0.0);

		// カーソルが移動できる幅を求める
		let mouse_move_x = self.screen.x - SCROLL_LEFT * 2.0 - WALL_LEFT;
		let mouse_move_y = self.screen.y - SCROLL_TOP * 2.0 - WALL_BOTTOM;
		
		// カーソルの移動速度を求める
		let move_x = over_size_x / mouse_move_x;
		let move_y = over_size_y / mouse_move_y;

		// 原点側にスクロールしすぎないよう、盤面の幅から最小座標を求める
		let min_left= self.screen.x - real_width - WALL_RIGHT;
		let min_top = self.screen.y - real_height - WALL_BOTTOM;

		// オフセットに反映する
		// このとき盤面が壁のサイズ（WALL_XXXX）を超えてスクロールしないよう制御する
		self.tb.offs.x = ((WALL_LEFT - mousepos_x * move_x).max(min_left)).min(WALL_LEFT);
		self.tb.offs.y = ((WALL_TOP - mousepos_y * move_y).max(min_top)).min(WALL_TOP);

		// カーソルは動いたか
		is_update |= self.cursol.index != cursol.index;
		self.cursol = cursol;

		// 更新有無を返却
		is_update
		
	}

	//------------------------------
	// テーブルにマウス位置を伝える
	//------------------------------
   fn set_tablepos(&mut self) {
		let tablepos = Vec2 {
			x: (self.mouse.pos.x - self.tb.offs.x) * (1.0 / self.tb.zoom.x),
			y: (self.mouse.pos.y - self.tb.offs.y) * (1.0 / self.tb.zoom.y),
		};
		self.cursol = self.tb.table.set_mousepos(tablepos);
	}

	//------------------------------
	// キーボード入力処理
	//------------------------------
	fn keycontrol(&mut self) -> bool {
		let mut is_update = false;

		//--------------------------------------------------
		// UNDO 処理
		//--------------------------------------------------
		if self.get_chkbox_flg(CBSetting::UndoFlg) {
			// 左キーで UNDO
			if is_key_pressed(KeyCode::Left) {
				// UNDO情報の最新＝現在なので、UNDO中でなければ
				// １回余計に UNDO する
				if !self.tb.table.is_useundo() {
					self.tb.table.table_undo();
				}
				self.tb.table.table_undo();
				self.assist();
				// 死んでたら復活させる
				self.stat = GameStat::Playing;
				return is_update;
			}

			// 右キーで REDO
			if is_key_pressed(KeyCode::Right) {
				self.tb.table.table_redo();
				self.assist();
				return is_update;
			}
		}

		//--------------------------------------------------
		// ZOOM 処理
		//--------------------------------------------------
		// 上キーでズームアウト
		if is_key_pressed(KeyCode::Up) {
			self.tb.zoom.x += 0.2;
			self.tb.zoom.y += 0.2;
			is_update = true;
		}

		// 下キーでズームイン
		if is_key_pressed(KeyCode::Down) {
			if self.tb.zoom.x > MIN_ZOOM {
				self.tb.zoom.x -= 0.2;
				self.tb.zoom.y -= 0.2;
				is_update = true;
			}
		}

		//--------------------------------------------------
		// Ｆキーですべての危険パネルにフラグを立てる
		//--------------------------------------------------
		if is_key_pressed(KeyCode::F) {
			self.tb.table.set_all_redflag();
			is_update = true;
		}
 
 		is_update
	}

	//------------------------------
	// 盤面右クリック処理
	// 変更があった場合 true、ない場合は false を返す
	//------------------------------
	fn click_tbl_right (&mut self) -> bool {
		let mut is_update = false;

		// マウス右クリックされていない、マウスが盤面上ではないなら何もしない
		if !self.mouse.righton || self.cursol.index == -1 ||
		   self.stat != GameStat::Playing {
			return is_update
		}

		// クリックしたことを盤面に伝える
		if self.mouse.righton_now {
			is_update = self.tb.table.click_right(
				self.get_chkbox_flg(CBSetting::UseBlueFlg),
				self.get_chkbox_flg(CBSetting::BlueFlgFirst));
		}

		is_update
	}

	//------------------------------
	// 盤面左クリック処理
	// 変更があった場合 true、ない場合は false を返す
	//------------------------------
	fn click_tbl_left (&mut self) -> bool {
		let mut is_update = false;

		// カーソルが盤面外ならなにもしない
		if self.cursol.index== -1 {
			return is_update
		}

		//--------------------------------------------------
		// ゲームは開始しクリック待ちならゲーム開始処理を行う
		//--------------------------------------------------
		if self.mouse.lefton && self.stat == GameStat::Ready {
			// ゲーム開始時刻を保持
			self.tm.playst = get_time();
			self.tm.played = self.tm.playst;

			// 盤面保持用のワークを作成
			let mut table_backup: Vec<Panel> = Vec::new();

			// 初手ある程度開かせる
			let target = self.tb.width * self.tb.height * 5 / 100;
			let mut max = 0;
			for _ in 0..100 {
				self.tb.table.setting_bom(self.tb.bom_num);
				self.tb.table.click_left();
				let opennum = self.tb.table.get_opennum();
				// 最も開いたパターンを保持しておく
				if max < opennum {
					max = opennum;
					table_backup = self.tb.table.tbl_backup();
				}
				if target <= opennum as i32 {
					break;
				}
				self.initial_game(0.0);
			}
			// 最も開いた盤面を復旧
			self.tb.table.tbl_restore(table_backup);

			// 今の盤面を保存する
			self.tb.table.undo_push();

			// ゲームを開始する
			self.stat = GameStat::Playing;
			self.setting.as_ref().unwrap().borrow_mut().set_playing_flg(true);
			return true
		}

		//--------------------------------------------------
		// ゲームプレイ中：クリックしたことを盤面に伝える
		//--------------------------------------------------
		if self.stat != GameStat::Playing {
			return is_update;
		}
		if self.mouse.leftoff_now {
			// マウスが「離された」時の処理
			is_update |= self.tb.table.clickoff_left();
			is_update |= self.tb.table.click_left();
		} else if self.mouse.lefton {
			// マウスが「押された」時の処理
			if self.get_chkbox_flg(CBSetting::DragOpen) {
				// DRAG OPEN がオンなら引きずりながら開く
				is_update |= self.tb.table.click_left();
				is_update |= self.tb.table.clickoff_left();
			}
		}

	is_update
	}

	//------------------------------
	// 盤面からウィンドウサイズを自動設定
	//------------------------------
	fn set_winsize(&mut self) {
		// 倍率を初期化する
		self.tb.zoom.x = MAX_ZOOMX;
		self.tb.zoom.y = MAX_ZOOMY;

		// 盤面のリアルサイズを求める
		for _ in 0..100 {
			let real_width = self.tb.width as f32 * PANEL_WIDTH * self.tb.zoom.x + WALL_LEFT + WALL_RIGHT;
			let real_height = self.tb.height as f32 * PANEL_HEIGHT * self.tb.zoom.y + WALL_TOP+ WALL_BOTTOM;

			// はみ出し量の大きいほうで判断
			let over_sz = (real_width - WIN_MIN_X).max(real_height - WIN_MIN_Y);

			// はみ出しサイズで倍率変更
			if over_sz > 0.0 {
				self.tb.zoom.x -= 0.1;
				self.tb.zoom.y -= 0.1;
			} else {
				break;
			}

			// 初期化時は倍率は最小 0.5 とする
			if self.tb.zoom.x <= 0.5 {
				break
			}
		}

		// ウインドウサイズに還元する
		self.screen.x = self.tb.width as f32 * PANEL_WIDTH * self.tb.zoom.x + WALL_LEFT + WALL_RIGHT;
		self.screen.y = self.tb.height as f32 * PANEL_HEIGHT * self.tb.zoom.y + WALL_TOP + WALL_BOTTOM;
		set_winsize(self.screen.x, self.screen.y);
	}

	//--------------------------------------------------
	// チェックボックスのクリック処理
	//--------------------------------------------------
	fn chkbox_click(&mut self) {
		// 左クリックされていないなら何もしない
		if !self.mouse.lefton_now {
			return
		}

		// チェックボックスのクリック処理
		if let Some((kind, _)) =
			self.chkbox.click(self.mouse.pos) {
			match kind {
				// 設定ボタンなら設定画面を開く
				CBGame::Settings => {
					self.setting.as_ref().unwrap().borrow_mut().open();					
				}
				
				// それ以外個別処理は特に不要
				_ => {}
			}
		}
	}

	//----------------------------------------
	// 設定画面から情報を取得
	//----------------------------------------
	fn get_chkbox_flg(&self, mytype:CBSetting) -> bool {
		self.setting.as_ref().unwrap().borrow().get_flg(mytype)
	}

	//------------------------------
	// ゲーム全体の描画
	//------------------------------
	pub fn draw(&self, myfont:&Font) {
		// 盤面全体を塗りつぶす
		clear_window(LAYOUT_COLOR);

		// HIDE レベルを取得する
		let mut hide_lv = 0;
		if self.get_chkbox_flg(CBSetting::HideLv1) {
			hide_lv = 1;
		} else if self.get_chkbox_flg(CBSetting::HideLv2) {
			hide_lv = 2;
		} if self.get_chkbox_flg(CBSetting::HideLv3) {
			hide_lv = 3;
		}
		let mut hidenum_lv = 0;
		if self.get_chkbox_flg(CBSetting::HideNumLv1) {
			hidenum_lv = 1;
		} else if self.get_chkbox_flg(CBSetting::HideNumLv2) {
			hidenum_lv = 2;
		} if self.get_chkbox_flg(CBSetting::HideNumLv3) {
			hidenum_lv = 3;
		}

		//--------------------------------------------------
		// 盤面描画
		//--------------------------------------------------
		self.draw_table(hide_lv, hidenum_lv);

		//--------------------------------------------------
		// メニュー描画
		//--------------------------------------------------
		// メニュー範囲を塗りつぶす
		dr_rect(0.0, 0.0, self.screen.x, WALL_TOP - 20.0,
			0.0, MENU_COLOR, "");
		dr_rect(0.0, WALL_TOP - 20.0, WALL_LEFT - 30.0, self.screen.y + 20.0,
			0.0, MENU_COLOR, "");
		dr_text_ex("--- MENU ---", 20.0, WALL_TOP + 100.0, 20.0,
			&String::from("000000FF"), &String::from("FFFFFFFF"),myfont);

		// 盤面サイズなどの表示
		let flag_num = self.tb.table.get_num_redflag();
		let text = format!("SIZE:{}x{}  BOMB:{}  RED FLAG:{}",
			self.tb.width, self.tb.height,
			self.tb.bom_num, flag_num);
		dr_text_ex(&text, 0.0, 0.0, FONT_SIZE,
			"A0A0FFFF", "000000FF", myfont);

		// ZOOM や UNDO 操作の表示
		let mut text = "ZoomUp[↑] ZoomDown[↓]";
		if self.get_chkbox_flg(CBSetting::UndoFlg) {
			text = "ZoomUp[↑] ZoomDown[↓] Undo[←] Redo[→]"
		}
		dr_text_ex(text,
			WALL_LEFT - 30.0, WALL_TOP - 40.0, 20.0,
			"000000FF", "FFFFFFFF", myfont);

		// 状態の表示
		let bg = String::from("000000FF");
		let (text, fg) =
			if self.stat == GameStat::Ready {
				("[ READY ]", String::from("00FFFFFF"))
			} else if self.stat == GameStat::Playing {
				if  self.tm.played - self.tm.playst < 1.0 {
					("[  GO!! ]", String::from("00FFFFFF"))
				} else {
					("",String::from("000000FF"))
				}
			} else if self.stat == GameStat::Success {
				("[CLEAR!!]", String::from("FFFF00FF"))
			} else if self.stat == GameStat::Failed {
				("[FAILED!]", String::from("FF0000FF"))
			} else {
				("", String::from("00000000"))
			};
		dr_text(text, 10.0, 130.0, FONT_SIZE * 1.4, &fg, &bg);

		//--------------------------------------------------
		// チェックボックス描画
		//--------------------------------------------------
		self.chkbox.draw(myfont);
	
		//--------------------------------------------------
		// 経過時間を描画
		//--------------------------------------------------
		let ((timestr, msec),fg) =
			if self.stat == GameStat::Ready {
				// ゲームが始まってなければ灰色表示
				(get_time_str(0.0,0.0), String::from("777777FF"))
			} else {
				// ステータスに応じて文字色を変更
				let fg = match self.stat {
	        	    GameStat::Playing => String::from("00FFFFFF"),
    	        	GameStat::Success => String::from("FFFF00FF"),
					_                 => String::from("FF0000FF"),
				};
				(get_time_str(self.tm.playst, self.tm.played), fg)
			};
		dr_rect(10.0, WALL_TOP - 20.0,
			WALL_LEFT - 60.0,FONT_SIZE_BIG - 10.0, 5.00, "000000FF", &fg);
		dr_text(&timestr,
			20.0,WALL_TOP - 18.0, FONT_SIZE_BIG,
			&fg, &String::from("000000FF"));
		dr_text(&msec,
			130.0,WALL_TOP + 20.0, FONT_SIZE_BIG * 0.5,
			&fg, &String::from("000000FF"));

		//--------------------------------------------------
		// HIDE レベルの表示
		//--------------------------------------------------
		let left = 0.0;
		let top = 370.0;
		let mut text = "";
		if hide_lv > 0 || hidenum_lv > 0 {
			dr_rect(left, top, WALL_LEFT -30.0, 60.0, 0.0, "FF000055", "");
		}
		if hide_lv > 0 {
			text = "HIDE BORD"
		} else if hidenum_lv > 0 {
			text = "HIDE NUMBER"
		}
		dr_text(text, left + 20.0, top + 10.0, FONT_SIZE,
			&String::from("FF0000FF"), &String::from("000000FF"));
		if hide_lv == 1 || hidenum_lv == 1 {
			text = "LEVEL 1"
		} else if hide_lv == 2 || hidenum_lv == 2 {
			text = "LEVEL 2"
		} else if hide_lv == 3 || hidenum_lv == 3 {
			text = "LEVEL 3"
		}
		dr_text(text, left + 20.0, top + 30.0, FONT_SIZE,
			&String::from("FF0000FF"), &String::from("000000FF"));

		//--------------------------------------------------
		// 死んだ回数を表示
		//--------------------------------------------------
		self.draw_death_cnt();
/*
			// デバッグ
		let mut pos_y = 400.0;
		let font_size = 30.0;
		let font_offs = 30.0;
		pos_y += font_offs;dr_text_ex(&format!("SCREEN:{},{} ZOOM:{},{}",self.screen.x,self.screen.y,self.tb.zoom.x, self.tb.zoom.y),
			0.0, pos_y,font_size,"FFFFFFFF", "000000FF",myfont);
		pos_y += font_offs;dr_text_ex(&format!("MOUSE:{},{}",self.mouse.pos.x,self.mouse.pos.y),
			0.0, pos_y,font_size,"FFFFFFFF", "000000FF",myfont);
		pos_y += font_offs;dr_text_ex(&format!("CURSOL:{},{}:{}",self.cursol.x,self.cursol.y,self.cursol.index),
			0.0, pos_y,font_size,"FFFFFFFF", "000000FF",myfont);
		pos_y += font_offs;dr_text_ex(&format!("TIME:{}",get_time()),
			0.0, pos_y,font_size,"FFFFFFFF", "000000FF",myfont);
*/
		//--------------------------------------------------
		// カーソル周辺に「CLEAR」を表示
		//--------------------------------------------------
		if self.stat == GameStat::Success && get_time() - self.tm.played < 1.0 {
			dr_text("CLEAR!!", self.mouse.pos.x - 60.0, self.mouse.pos.y -40.0,
				FONT_SIZE_BIG, "FFFF00FF", "000000AA");
		}

		//--------------------------------------------------
		// 設定画面表示
		//--------------------------------------------------
		self.setting.as_ref().unwrap().borrow().draw(myfont);

		//--------------------------------------------------
		// 操作方法の表示
		//--------------------------------------------------
		if self.chkbox.get_flg(CBGame::HowTo) {
			self.draw_howto(myfont);
		}

	}

	//------------------------------
	// 操作方法のの描画
	//------------------------------
	fn draw_howto(&self, myfont: &Font) {
		let left = 20.0;
		let top = 20.0;
		let font_size = 20.0;
		let offs = 10.0;

		// 枠を描画
		let lines = HOWTO.lines().count();
		dr_rect(left, top, 750.0, lines as f32 * (font_size + offs) + 60.0,
			3.0, "000000CC", "FF0000FF");

		// 操作方法表示
		dr_text_ex_multi(HOWTO, left + 20.0, top + 20.0,
			font_size, 10.0, "FFFFFFFF", "777700FF", myfont);
	}

	//------------------------------
	// 盤面の描画
	//------------------------------
	fn draw_table(&self, hide_lv:i32, hidenum_lv:i32) {
		// カメラをセット
		let zoom = Vec2 {
			x: self.tb.zoom.x * 2.0 / screen_width(),
			y: self.tb.zoom.y * 2.0 / screen_height(),
		};
		let offset = Vec2 {
			x: self.tb.offs.x * 2.0 / screen_width() - 1.0,
			y: - (self.tb.offs.y * 2.0 / screen_height()) + 1.0,
		};
		let camera = Camera2D {
			zoom, offset,
			..Default::default()
		};
		set_camera(&camera);

		// 縁取り
		let offs = 10.0;
		dr_rect( -offs, -offs,
			self.tb.width as f32 * PANEL_WIDTH + offs * 2.0,
			self.tb.height as f32 * PANEL_HEIGHT + offs * 2.0,
			0.0, "0000FFFF", "");
			
		// 盤面の描画
		let is_dangon= self.get_chkbox_flg(CBSetting::DangOn);
		let mut is_safeon = self.get_chkbox_flg(CBSetting::SafeOn);
		is_safeon |= self.get_chkbox_flg(CBSetting::BoldSafeOn);
		self.tb.table.draw_panel(self.get_chkbox_flg(CBSetting::DispAll),
			is_dangon, is_safeon, hide_lv, hidenum_lv,
			self.stat == GameStat::Playing || self.stat == GameStat::Ready);

		// カーソル周りに枠を表示
		if self.cursol.index != -1 && self.get_chkbox_flg(CBSetting::CursolFlg) {
			self.tb.table.draw_curasol();
		}

		// カメラをリセット
		set_default_camera();
	}

	//------------------------------
	// 死んだ回数を表示する
	//------------------------------
	fn draw_death_cnt(&self) {
		// １度も死んでいないまたは１回目の死亡の瞬間は表示しない
		if self.death_cnt == 0 ||
		   self.death_cnt == 1 && self.stat == GameStat::Failed {
			return
		}

		let origin_x = 10.0;
		let origin_y = 500.0;
		let offs = 15.0;
		let mut draw_cnt = 0;
		'root: for y in 0..65535 {
			for x in 0..10 {
				dr_rect(origin_x + x as f32 * offs, origin_y + y as f32 * offs,
					 9.0, 9.0, 1.0, "FF4444FF", "000000FF");
				dr_rect(origin_x + x as f32 * offs + 2.0, origin_y + y as f32 * offs + 2.0,
					 2.0, 3.0, 0.0, "000000FF", "");
				dr_rect(origin_x + x as f32 * offs + 6.0, origin_y + y as f32 * offs + 2.0,
					 2.0, 3.0, 0.0, "000000FF", "");
				dr_rect(origin_x + x as f32 * offs + 1.0, origin_y + y as f32 * offs + 6.0,
					 2.0, 3.0, 0.0, "00000077", "");
				draw_cnt += 1;
				if draw_cnt >= self.death_cnt {
					break 'root;
				}
			}
		}
	}
}

//------------------------------
// プレイ時間文字列を返却
//------------------------------
fn get_time_str(sttime:f64, nowtime: f64) -> (String,String) {
	let sec = nowtime - sttime;
	(format!("{:03}:{:02}", (sec / 60.0) as i32, (sec % 60.0) as i32),format!(".{:03}", (sec.fract() * 1000.0) as i32))
}