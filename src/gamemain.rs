use macroquad::prelude::*;
use crate::draw::*;
use crate::chkbox::ChkBoxMng;
use crate::utils::*;
use crate::myconst::*;
use crate::gametable::GameTable;
use crate::gametable::MyCursol;
use crate::panel::Panel;
use crate::inference::InfTable;

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
	pos: PosTable,
	lefton: bool,
	righton: bool,
	is_left_click: bool,
	is_right_click: bool,
}

pub struct GameMain<'a> {					// ゲームメイン情報
	stat: GameStat,							// ゲームの状態
	screen: Vec2,							// ウインドウサイズ
	mouse: MouseTbl,	   	 	            // マウスカーソル位置
	cursol: MyCursol,                       // カーソル位置
	tm: MyTime,								// 時刻関連
	tb: TableInfo,                       	// 盤面情報
	chkbox: ChkBoxMng<'a,ChkBoxGame>,		// 自作チェックボックス
	myfont: &'a Font,						// フォント情報
}

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl<'a> GameMain<'a> {
	//------------------------------
	// 初期化
	//------------------------------
	pub fn new (myfont: &'a Font) -> GameMain<'a> {

		// 生成
		let mut gm = GameMain {
			stat: GameStat::Ready,
			screen: Vec2 {x: screen_width(), y: screen_height()},
			tm: MyTime {gamewait: 0.0, waitst: 0.0, playst: 0.0, played: 0.0},
			mouse: MouseTbl{
				pos: PosTable{x:0.0,y:0.0},
				lefton: false, is_left_click: false,
				righton: false, is_right_click: false},
			cursol: MyCursol {x: -1, y: -1, index: -1},
			tb: TableInfo {width: 0, height: 0, bom_num: 0,
				table: GameTable::new(0,0,0),
				offs: Vec2 { x: WALL_LEFT, y: WALL_TOP },
				zoom: Vec2 { x: MAX_ZOOMX, y: MAX_ZOOMY},
			},
			chkbox: ChkBoxMng::new(myfont),
			myfont,
		};

		// チェックボックス作成
        gm.chkbox.set_base(20.0,180.0,200.0, 30.0, 25.0,"000000FF", "FFFFFFFF");
		gm.chkbox.add(ChkBoxGame::CursolFlg, String::from("CURSOL FLAME"), false);
		gm.chkbox.add(ChkBoxGame::DragOpen, String::from("DRAG OPEN"), false);
		gm.chkbox.add(ChkBoxGame::UseBlueFlg, String::from("USE BLUEFLAG"), false);
		gm.chkbox.add(ChkBoxGame::BoldFlg, String::from("USE BOLD"), false);
		gm.chkbox.set_offs(ChkBoxGame::BoldFlg, 0.0, 60.0);
		gm.chkbox.add(ChkBoxGame::Inference, String::from("USE INFERENCE"), false);
		gm.chkbox.add(ChkBoxGame::UndoFlg, String::from("USE UNDO"), false);
		gm.chkbox.add(ChkBoxGame::Title, String::from(" [RETURN TITLE]"), false);
		gm.chkbox.set_col(ChkBoxGame::Title, "007777FF","");
		gm.chkbox.set_offs(ChkBoxGame::Title, 0.0, 10.0);
		
		// 子のチェックボックス作成
        gm.chkbox.set_base(20.0,180.0,200.0, 25.0, 20.0,"000000FF", "FFFFFFFF");
		gm.chkbox.addsub(ChkBoxGame::BoldSafeOn, ChkBoxGame::BoldFlg,String::from("SAFETY ON"), false);
		gm.chkbox.addsub(ChkBoxGame::SafeOn,ChkBoxGame::Inference, String::from("SAFETY ON"), false);
		gm.chkbox.addsub(ChkBoxGame::DangOn,ChkBoxGame::Inference, String::from("DANGER ON"), true);
		gm.chkbox.addsub(ChkBoxGame::DispAll,ChkBoxGame::Inference, String::from("All DISPLAY"), false);
		gm.chkbox.addsub(ChkBoxGame::BelieveFlag,ChkBoxGame::Inference, String::from("BELEVE FLAG"), false);

		gm.chkbox.view_hitbox(false);

		// タイトルへ戻るはボックス部分を非表示
		gm.chkbox.view_box(ChkBoxGame::Title, false);

		// 説明文追加
		gm.chkbox.set_help(ChkBoxGame::CursolFlg,"[CURSOL FLAME]\n９×９のカーソルを表示します。");
		gm.chkbox.set_help(ChkBoxGame::DragOpen,"[DRAG OPEN]\n押しっぱなしでまとめてパネルを開きます。");
		gm.chkbox.set_help(ChkBoxGame::UseBlueFlg,"[USE BLUEFLAG]\n「青色の旗」を使用します、赤色の旗と区別したい場合に使用してください。");
		gm.chkbox.set_help(ChkBoxGame::BoldFlg,"[USE BOLD]※初心者にお勧め\n「数字」と「周りの未開封パネル数」が一致していると強調表示されます。\n（正しく旗を立てると強調表示は消えます）");
		gm.chkbox.set_help(ChkBoxGame::BoldSafeOn,"[SAFETY ON]\n旗の周囲の安全パネルを表示します。");
		gm.chkbox.set_help(ChkBoxGame::Inference,"[USE INFERENCE]\n見えている数字から、危険／安全パネルを推論します。");
		gm.chkbox.set_help(ChkBoxGame::DangOn,"[DANGER ON]\n推論で危険パネルを表示します。");
		gm.chkbox.set_help(ChkBoxGame::SafeOn,"[SAFETY ON]\n推論で安全パネルを表示します。");
		gm.chkbox.set_help(ChkBoxGame::DispAll,"[ALL DISPLAY]\n全体に危険／安全パネルを表示します。");
		gm.chkbox.set_help(ChkBoxGame::BelieveFlag,"[BELIEVE FLAG]\nあなたの立てた旗を信じて推論します。");
		gm.chkbox.set_help(ChkBoxGame::UndoFlg,"[USE UNDO]\nUNDO（やり直し）を有効にします。");
		gm.chkbox.set_help(ChkBoxGame::Title,"[RETURN TITLE]\nタイトルに戻ります。");

		gm
	}

	//------------------------------
	// ゲームの情報の設定
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
		self.tb.table.initial(self.tb.width, self.tb.height);
		self.set_tablepos();

		// 待ち時間の指定があるなら設定
		self.tm.gamewait = wait;
		if wait != 0.0 {
			self.tm.waitst = get_time();
		}

		// クリック待ちへ遷移
		self.stat = GameStat::Ready;
	}

	//------------------------------
	// ウィンドウサイズの設定
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

	//------------------------------
	// 入力制御
	//------------------------------
	pub fn playcontrol(&mut self) -> GameMode {
		// マウスの情報を更新する
		self.get_mouse();

		// 待ち時間が設定されている場合、時間消化までなにもしない
		if self.tm.gamewait != 0.0 {
			if get_time() - self.tm.waitst < self.tm.gamewait {
				return GameMode::Game;
			}
			self.tm.gamewait = 0.0;
		}

		// 盤面の更新フラグを初期化
		let mut is_update = false;

		// マウス移動処理
		self.mouse_move();

		// マウスクリック判定
		is_update |= self.click_tbl_left();
		is_update |= self.click_tbl_right();

		// 「タイトルへ」が選択されたらタイトルへ戻る
		if self.chkbox.get_flg(ChkBoxGame::Title) {
			// 内部的にフラグを落としておく
			self.chkbox.set_flg(ChkBoxGame::Title, false);
			return GameMode::Title;
		}

		// キーボード入力処理
		let is_keyupdate = self.keycontrol();

		// ゲームが開始されているならプレイ時間更新
		if self.stat == GameStat::Playing {
			self.tm.played = get_time();
		}

		// アシスト機能
		if (is_update || is_keyupdate) && self.stat == GameStat::Playing{
			self.assist();
		}

		// 更新が発生した場合
		if is_update && self.stat == GameStat::Playing {
			// 今の盤面を保存する
			self.tb.table.undo_push();

			// 爆弾を除くパネルが全て開かれたか
			let close_num = self.tb.width * self.tb.height - self.tb.bom_num - self.tb.table.get_opennum() as i32;
			if close_num == 0 {
				self.stat = GameStat::Success;
				self.mouse.lefton = false;
				self.mouse.is_left_click = false;
			}

			// 爆弾が開かれた場合はステータスを変える
			if self.tb.table.open_bomnum() > 0 {
				self.stat = GameStat::Failed;
				self.mouse.lefton = false;
				self.mouse.is_left_click = false;
			}
		}

		GameMode::Game
	}

	//------------------------------
	// マウスの状態取得
	//------------------------------
	fn get_mouse(&mut self) {
		// マウス位置の取得
		let (x,y) = mouse_position();
		self.mouse.pos.x = x;
		self.mouse.pos.y = y;

		// 左クリック処理
		self.mouse.is_left_click = false;
		if is_mouse_button_pressed(MouseButton::Left) {
			// 今左クリックが押されたなら、クリックフラグオン
			if !self.mouse.lefton {
				self.mouse.is_left_click = true;
			}
			self.mouse.lefton = true;
		} else if is_mouse_button_released(MouseButton::Left) {
			// 左クリックが離された
			self.mouse.is_left_click = false;
			self.mouse.lefton = false;
		}

		// 右クリック処理
		self.mouse.is_right_click = false;
		if is_mouse_button_pressed(MouseButton::Right) {
			// 今右クリックが押されたなら、クリックフラグオン
			if !self.mouse.righton {
				self.mouse.is_right_click = true;
			}
			self.mouse.righton = true;
		} else if is_mouse_button_released(MouseButton::Right) {
			// 右クリックが離された
			self.mouse.is_right_click = false;
			self.mouse.righton = false;
		}
	}

	//------------------------------
	// アシスト機能
	//------------------------------
	fn assist(&mut self) {

		let bold_flg = self.chkbox.get_flg(ChkBoxGame::BoldFlg);
		let inference_flg = self.chkbox.get_flg(ChkBoxGame::Inference);

		// アシストオフならフラグをクリアして終了
		if !bold_flg && !inference_flg {
			self.tb.table.clear_help();
			return
		}

		// 推論ロジックへテーブルのコピーを渡す
		let edit_table = self.tb.table.tbl_backup();
		let mut inftbl = InfTable::new(edit_table,self.tb.width, self.tb.height);

	    // 太字処理か推論処理かでフラグ処理
		if bold_flg {
			let safe_on = self.chkbox.get_flg(ChkBoxGame::BoldSafeOn);
			inftbl.set_bold(safe_on);
		} else {
			let safe_on = self.chkbox.get_flg(ChkBoxGame::SafeOn);
			let dang_on = self.chkbox.get_flg(ChkBoxGame::DangOn);
			let believe_flg = self.chkbox.get_flg(ChkBoxGame::BelieveFlag);
			inftbl.inference(safe_on, dang_on, believe_flg);
		}

		// 処理結果を現在のテーブルへフィードバック
		let edit_table = inftbl.get_table();
		self.tb.table.tbl_restore(edit_table);
	}

	//------------------------------
	// マウス位置を更新
	//------------------------------
	fn mouse_move(&mut self) -> bool {
		let mut is_update = false;

		// 画面サイズの取得
		self.screen.x = screen_width();
		self.screen.y = screen_height();
	
		// 盤面にマウス位置を反映
		let tablepos = Vec2 {
			x: (self.mouse.pos.x - self.tb.offs.x) * (1.0 / self.tb.zoom.x),
			y: (self.mouse.pos.y - self.tb.offs.y) * (1.0 / self.tb.zoom.y),
		};
		let cursol = self.tb.table.set_mousepos(tablepos);

		// 押しっぱなしの場合はスクロールしない
		if self.mouse.lefton {
			return is_update;
		}

		// スクロール制御
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
		let is_update = false;

		//--- UNDO 処理 ---//
		if self.chkbox.get_flg(ChkBoxGame::UndoFlg) {
			if is_key_pressed(KeyCode::Left) {
				// UNDO情報の最新＝現在なので、UNDO中でなければ
				// １回余計に UNDO する
				if !self.tb.table.is_useundo() {
					self.tb.table.table_undo();
				}
				self.tb.table.table_undo();
				self.stat = GameStat::Playing;
				return false;
			}

			// REDO 処理
			if is_key_pressed(KeyCode::Right) {
				self.tb.table.table_redo();
				return false;
			}
		}

		// 上キーでズームアウト
		if is_key_pressed(KeyCode::Up) {
			self.tb.zoom.x += 0.2;
			self.tb.zoom.y += 0.2;
		}

		// 下キーでズームイン
		if is_key_pressed(KeyCode::Down) {
			if self.tb.zoom.x > MIN_ZOOM {
				self.tb.zoom.x -= 0.2;
				self.tb.zoom.y -= 0.2;
			}
		}

		// Ｆキーですべての危険パネルにフラグズームアウト
		if is_key_pressed(KeyCode::F) {
			self.tb.table.set_all_redflag();
		}
 
 		is_update
	}

	//------------------------------
	// 盤面右クリック処理
	// 変更があった場合 true、ない場合は false を返す
	//------------------------------
	fn click_tbl_right (&mut self) -> bool {
		let mut is_update = false;

		// マウス右クリックされていない、マウスが盤面上ではない、なら何もしない
		if !self.mouse.righton ||
			self.cursol.index == -1 {
			return false
		}

		// クリックしたことを盤面に伝える
		if self.mouse.is_right_click {
			is_update = self.tb.table.click_right(self.chkbox.get_flg(ChkBoxGame::UseBlueFlg));
			}
		is_update
	}

	//------------------------------
	// 盤面左クリック処理
	// 変更があった場合 true、ない場合は false を返す
	//------------------------------
	fn click_tbl_left (&mut self) -> bool {
		// マウス左クリックされていないなら何もしない
		if !self.mouse.lefton {
			return false
		}

		// 更新フラグの初期化
		let mut is_update = false;

		// チェックボックス判定
		if self.mouse.is_left_click {
			is_update |= self.chk_box_click();
		}
		
		// カーソルが盤面外ならなにもしない
		if self.cursol.index== -1 {
			return is_update
		}

		// ゲームが待機中なら初期化しなおす
		if self.stat == GameStat::Success || self.stat == GameStat::Failed {
			self.initial_game(START_WAIT);
			self.stat = GameStat::Ready;
			return true
		}

		// ゲームは開始し、クリック待ちなら爆弾を生成する
		if self.mouse.is_left_click &&
		   self.stat == GameStat::Ready {
			// ゲーム開始時刻を保持
			self.tm.playst = get_time();
			self.tm.played = self.tm.playst;

			// 盤面を作成
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
			self.stat = GameStat::Playing;
			return true
		}

		// クリックしたことを盤面に伝える
		if self.stat == GameStat::Playing {
			// 左クリック単発押し、または DRAG OPEN がオンなら
			if self.mouse.is_left_click ||
			   self.chkbox.get_flg(ChkBoxGame::DragOpen) {
				is_update |= self.tb.table.click_left();
			   }
		}

		is_update
	}

	//--------------------------------------------------
	// チェックボックスのクリック処理
	//--------------------------------------------------
	fn chk_box_click(&mut self) -> bool {
		let mut is_update = false;

		// チェックボックスのクリック処理
		if let Some((kind, flg)) =
			self.chkbox.click(self.mouse.pos.x, self.mouse.pos.y) {
			match kind {

				// 強調フラグが選択された場合
				ChkBoxGame::BoldFlg => {
					// 推論フラグをオフにする
					self.chkbox.set_flg(ChkBoxGame::CursolFlg, true);
					self.chkbox.set_flg(ChkBoxGame::Inference, false);
				}

				// 推論フラグが選択された場合
				ChkBoxGame::Inference => {
					// 強調フラグをオフにする
					self.chkbox.set_flg(ChkBoxGame::CursolFlg, true);
					self.chkbox.set_flg(ChkBoxGame::BoldFlg, false);
				}

				// 推論全表示が選択された場合
				ChkBoxGame::DispAll => {
					// 安全マス危険マス全部表示
					if flg {
						self.chkbox.set_flg(ChkBoxGame::SafeOn, true);
						self.chkbox.set_flg(ChkBoxGame::DangOn, true);
					}
				}

				// それ以外は何もしない
				_ => {}
			}
			is_update = true;

		}
		is_update
	}

	//------------------------------
	// ゲーム全体の描画
	//------------------------------
	pub fn draw(&self) {
		// 盤面全体を塗りつぶす
		clear_window(LAYOUT_COLOR);

		// 盤面描画
		self.draw_table();

		// メニュー表示
		draw_rectangle(0.0, 0.0, self.screen.x, WALL_TOP - 20.0, MENU_COLOR.with_alpha(0.6));
		draw_rectangle(0.0, WALL_TOP - 20.0, WALL_LEFT - 30.0, self.screen.y + 20.0, MENU_COLOR.with_alpha(0.6));

		let flag_num = self.tb.table.get_num_redflag();
		let text = format!("SIZE:{}x{}  BOMB:{}  RED FLAG:{}",
			self.tb.width, self.tb.height,
			self.tb.bom_num, flag_num);
		dr_text_ex(&text, 0.0, 0.0, FONT_SIZE,
			"A0A0FFFF", "000000FF", self.myfont);
		dr_text_ex("ZoomUp[↑] ZoomDown[↓] Undo[←] Redo[→]",
			WALL_LEFT - 30.0, WALL_TOP - 40.0, 20.0,
			"000000FF", "FFFFFFFF", self.myfont);

		// ゲームの状態表示
		let bg = String::from("000000FF");
		let (text, fg) =
			if self.stat == GameStat::Success {
				(" [CLEAR!!]", String::from("00FF00FF"))
			} else if self.stat == GameStat::Failed {
				("[FAILED!!]", String::from("FF0000FF"))
			} else {
				("", String::from("00000000"))
			};
		dr_text(text, 20.0, 120.0, FONT_SIZE_BIG, &fg, &bg);

		// チェックボックスを表示する
		dr_text_ex("-- ASSIST --", 20.0, 290.0, FONT_SIZE,
			"A0A0FFFF", "000000FF", self.myfont);
		self.chkbox.draw();
	
		// 経過時間を表示
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
		draw_rectangle(20.0, WALL_TOP - 40.0,
			WALL_LEFT - 70.0,FONT_SIZE_BIG, BLACK);
/*
			draw_rectangle_lines(
			20.0, WALL_TOP,
			WALL_LEFT - 70.0,FONT_SIZE,5.0, fg);
 */
		dr_text(&timestr,
			60.0,WALL_TOP - 35.0, FONT_SIZE_BIG * 1.2,
			&fg, &String::from("000000FF"));
		dr_text(&msec,
			50.0 + 140.0,WALL_TOP + 10.0, FONT_SIZE_BIG * 0.6,
			&fg, &String::from("000000FF"));
/*
		// デバッグ
		let mut pos_y = 400.0;
		let font_size = 30.0;
		let font_offs = 30.0;
		pos_y += font_offs;dr_text_ex(&format!("SCREEN:{},{} ZOOM:{},{}",self.screen.x,self.screen.y,self.tb.zoom.x, self.tb.zoom.y),
			0.0, pos_y,font_size,"FFFFFFFF", "000000FF",self.myfont);
		pos_y += font_offs;dr_text_ex(&format!("MOUSE:{},{}",self.mouse.pos.x,self.mouse.pos.y),
			0.0, pos_y,font_size,"FFFFFFFF", "000000FF",self.myfont);
		pos_y += font_offs;dr_text_ex(&format!("CURSOL:{},{}:{}",self.cursol.x,self.cursol.y,self.cursol.index),
			0.0, pos_y,font_size,"FFFFFFFF", "000000FF",self.myfont);
		pos_y += font_offs;dr_text_ex(&format!("TIME:{}",get_time()),
			0.0, pos_y,font_size,"FFFFFFFF", "000000FF",self.myfont);
*/
		self.draw_help();
	}

	//------------------------------
	// 盤面の描画
	//------------------------------
	fn draw_table(&self) {
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
		draw_rectangle( -offs, -offs,
			self.tb.width as f32 * PANEL_WIDTH + offs * 2.0,
			self.tb.height as f32 * PANEL_HEIGHT + offs * 2.0, BLUE);

		// 盤面の描画
		let is_dangon= self.chkbox.get_flg(ChkBoxGame::DangOn);
		let mut is_safeon = self.chkbox.get_flg(ChkBoxGame::SafeOn);
		is_safeon |= self.chkbox.get_flg(ChkBoxGame::BoldSafeOn);
		self.tb.table.draw_panel(self.chkbox.get_flg(ChkBoxGame::DispAll), is_dangon, is_safeon);

		// カーソル周りに枠を表示
		if self.chkbox.get_flg(ChkBoxGame::CursolFlg) {
			self.tb.table.draw_curasol();
		}

		// カメラをリセット
		set_default_camera();
	}

	//------------------------------
	// 盤面の描画
	//------------------------------
	fn draw_help(&self) {
		let fontsize = 20.0;
		let offs = 5.0;
		if let Some((typ, help_lines)) =
		   self.chkbox.gethelp(self.mouse.pos.x, self.mouse.pos.y) {
			// ヘルプが設定されていない場合何もしない
			if help_lines.len() == 0 {
				return;
			}
			// ヘルプ周囲を塗りつぶす
			let left = self.mouse.pos.x;
			let top = self.mouse.pos.y + 25.0;
			let height = help_lines.len() as f32 * (fontsize + offs);
			let width = 800.0;
			draw_rectangle(left - 5.0, top - 5.0,
				width + 10.0, height + 10.0, BLACK.with_alpha(0.5));
			// ヘルプテキストを表示する
			for (i, line) in help_lines.iter().enumerate() {
				dr_text_ex(line, left, top + i as f32 * (fontsize + offs),
					fontsize, "00FFFFFF", "000000FF", self.myfont);
			}
		}
	}
}

//------------------------------
// プレイ時間文字列を返却
//------------------------------
fn get_time_str(sttime:f64, nowtime: f64) -> (String,String) {
	let sec = nowtime - sttime;
	(format!("{:02}:{:02}", (sec / 60.0) as i32, (sec % 60.0) as i32),format!(".{:03}", (sec.fract() * 1000.0) as i32))
}

