use macroquad::prelude::*;
use crate::draw::*;
use crate::chkbox::{ChkBox,ChkBoxType};
use crate::utils::*;
use crate::myconst::*;
use crate::gametable::GameTable;
use crate::gametable::MyCursol;
use crate::panel::Panel;

#[derive(PartialEq)]
enum GameStat {								// ゲームの状態
	TITLE,									// タイトル
	START,									// ゲームスタート（入力待ち）
	PLAYNG,									// プレイ中
	FAILED,									// 爆弾を踏んだ
	SUCCESS,								// ステージクリア
}

struct TableInfo {							// 盤面の情報
	width: i32,                             // 盤面の幅
	height: i32,                            // 盤面の高さ
	bom_num: i32,                           // 爆弾の数
	table: GameTable,                       // 盤面テーブル
	offs: Vec2,                             // 画面オフセット
	zoom: Vec2,                             // 画面倍率
}

// ゲームメインデータ
pub struct GameMain {
	stat: GameStat,							// 0:初期画面/1:開始待ち/2:プレイ中
	scr_width: f32,                         // ウインドウ幅
	scr_height: f32,                        // ウインドウ幅
	helplv: i32,                            // アシストレベル
	mouse_pos: PosTable,                    // マウスカーソル位置
	cursol: MyCursol,                       // カーソル位置
	tbldt: TableInfo,                       // 盤面情報
	chkbox: Vec<ChkBox>,					// 自作チェックボックス
}

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl GameMain {
	//------------------------------
	// 初期化
	//------------------------------
	pub fn new () -> GameMain {
		let mut gm = GameMain {
			stat: GameStat::TITLE,
			scr_width: screen_width(),
			scr_height: screen_height(),
			helplv: 0,
			mouse_pos: PosTable{ x:0.0, y:0.0},
			cursol: MyCursol {x: -1, y: -1, index: -1},
			tbldt: TableInfo {
				width: 0,
				height: 0,
				bom_num: 0,
				table: GameTable::new(0,0,0),
				offs: Vec2 { x: WALL_LEFT, y: WALL_TOP },
				zoom: Vec2 { x: MAX_ZOOMX, y: MAX_ZOOMY},
			},
			chkbox: Vec::new(),
		};

		// チェックボックスを作成
		let mut pos_y = 30.0;
		let fgcol: (u8,u8,u8,u8) = (0,0,0,255);
		let bgcol: (u8,u8,u8,u8) = (255,255,255,255);

		// カーソル表示
		pos_y += 30.0; gm.chkbox.push(ChkBox::new(
			ChkBoxType::CursolFlg, String::from("CURSOL FLAME"),
			1.0, pos_y, SUB_FONT_SIZE, fgcol, bgcol, false));
		// UNDO使用
		pos_y += 30.0; gm.chkbox.push(ChkBox::new(
			ChkBoxType::UndoFlg, String::from("USE UNDO"),
			1.0, pos_y, SUB_FONT_SIZE, fgcol, bgcol, false));
		// BOLD使用
		pos_y += 30.0; gm.chkbox.push(ChkBox::new(
			ChkBoxType::BoldFlg, String::from("USE BOLD"),
			1.0, pos_y, SUB_FONT_SIZE, fgcol, bgcol, false));
		// 安全マス表示
		pos_y += 30.0; gm.chkbox.push(ChkBox::new(
			ChkBoxType::SafeOn, String::from("SAFETY PANEL"),
			1.0, pos_y, SUB_FONT_SIZE, fgcol, bgcol, false));
		// 危険マス表示
		pos_y += 30.0; gm.chkbox.push(ChkBox::new(
			ChkBoxType::DangOn, String::from("DANGER PANEL"),
			1.0, pos_y, SUB_FONT_SIZE, fgcol, bgcol, false));
		// 前面ヒントマス表示
		pos_y += 30.0; gm.chkbox.push(ChkBox::new(
			ChkBoxType::DispAll, String::from("DISPLAY ALL"),
			1.0, pos_y, SUB_FONT_SIZE, fgcol, bgcol, false));

		gm
	}

	//------------------------------
	// ゲームの情報の設定
	//------------------------------
	pub fn set_gameinfo(&mut self, width: i32, height: i32, bom_num: i32) {
		self.tbldt.width = width;
		self.tbldt.height = height;
		self.tbldt.bom_num = bom_num;
		self.set_winsize();
	}

	//------------------------------
	// 盤面の初期化
	//------------------------------
	pub fn initial_game(&mut self) {
		// 盤面を初期化する
		self.tbldt.table = GameTable::new(self.tbldt.width, self.tbldt.height, self.tbldt.bom_num);
		self.tbldt.table.initial(self.tbldt.width, self.tbldt.height);
		self.set_tablepos();
		// クリック待ち
		self.stat = GameStat::START;
	}

	//------------------------------
	// ウィンドウサイズの設定
	//------------------------------
	fn set_winsize(&mut self) {
		// 倍率を初期化する
		self.tbldt.zoom.x = MAX_ZOOMX;
		self.tbldt.zoom.y = MAX_ZOOMY;

		// 盤面のリアルサイズを求める
		for x in 0..100 {
			let real_width = self.tbldt.width as f32 * PANEL_WIDTH * self.tbldt.zoom.x + WALL_LEFT + WALL_RIGHT;
			let real_height = self.tbldt.height as f32 * PANEL_HEIGHT * self.tbldt.zoom.y + WALL_TOP+ WALL_BOTTOM;

			// はみ出し量の大きいほうで判断
			let over_sz = (real_width - WIN_MIN_X).max(real_height - WIN_MIN_Y);

			// はみ出しサイズで倍率変更
			if over_sz > 0.0 {
				self.tbldt.zoom.x -= 0.1;
				self.tbldt.zoom.y -= 0.1;
			} else {
				break;
			}

			// 初期化時は倍率は最小 0.5 とする
			if self.tbldt.zoom.x <= 0.5 {
				break
			}
		}

		// ウインドウサイズに還元する
		self.scr_width = self.tbldt.width as f32 * PANEL_WIDTH * self.tbldt.zoom.x + WALL_LEFT + WALL_RIGHT;
		self.scr_height = self.tbldt.height as f32 * PANEL_HEIGHT * self.tbldt.zoom.y + WALL_TOP + WALL_BOTTOM;
		set_winsize(self.scr_width, self.scr_height);
	}

	//------------------------------
	// 入力制御
	//------------------------------
	pub fn playcontrol(&mut self) {
		let mut is_update = false;

		// マウス移動処理
		self.mouse_move();

		// マウスクリック判定
		is_update |= self.click_tbl_left();
		is_update |= self.click_tbl_right();

		// キーボード入力処理
		let is_keyupdate = self.keycontrol();

		// ヒントを作成する
		let bold_flg = self.get_chkbox_flg(ChkBoxType::BoldFlg);
		let safe_on = self.get_chkbox_flg(ChkBoxType::SafeOn);
		let dang_on = self.get_chkbox_flg(ChkBoxType::DangOn);
		println!("is_update={},SafeOn={},DangOn={}",is_update,safe_on,dang_on);
		if is_update || is_keyupdate {
			self.tbldt.table.clear_help(self.helplv);
			if bold_flg {
				// 強調表示オン
				self.tbldt.table.set_bold(bold_flg, dang_on, safe_on);
			} else if dang_on || safe_on {
				// 強調表示オフで安全マス表示または危険マス表示
				self.tbldt.table.auto_flag(dang_on, safe_on);
			}
		}

		// 更新が発生した場合
		if is_update {
			// 今の盤面を保存する
			self.tbldt.table.undo_push();

			// 爆弾を除くパネルが全て開かれたか
			let close_num = self.tbldt.width * self.tbldt.height - self.tbldt.bom_num - self.tbldt.table.get_opennum() as i32;
			if close_num == 0 {
				self.stat = GameStat::SUCCESS;
			}

			// 爆弾が開かれた場合はステータスを変える
			if self.tbldt.table.open_bomnum() > 0 {
				self.stat = GameStat::FAILED;
			}
		}
	}

	//------------------------------
	// チェックボックスからフラグを取得
	//------------------------------
	fn get_chkbox_flg(&self, mytype: ChkBoxType) -> bool {
		for index in 0..self.chkbox.len() {
			if self.chkbox[index].get_type() == mytype {
				return self.chkbox[index].get_flg()
			}
		}
		false
	}

	//------------------------------
	// マウス位置をテーブルへ反映
	//------------------------------
	fn mouse_move(&mut self) -> bool {
		let mut is_update = false;

		// 画面サイズの取得
		self.scr_width = screen_width();
		self.scr_height = screen_height();
		
		// マウス位置の取得
		let (x,y) = mouse_position();
		self.mouse_pos.x = x;
		self.mouse_pos.y = y;

		// 盤面にマウス位置を反映
		let tablepos = Vec2 {
			x: (self.mouse_pos.x - self.tbldt.offs.x) * (1.0 / self.tbldt.zoom.x),
			y: (self.mouse_pos.y - self.tbldt.offs.y) * (1.0 / self.tbldt.zoom.y),
		};
		let cursol = self.tbldt.table.set_mousepos(tablepos);

		// スクロール制御
		// 盤面のリアルサイズを求める
		let real_width = self.tbldt.width as f32 * PANEL_WIDTH * self.tbldt.zoom.x;
		let real_height = self.tbldt.height as f32 * PANEL_HEIGHT * self.tbldt.zoom.y;

		// 画面からはみ出すサイズを求める
		let over_size_x = real_width + WALL_LEFT + WALL_RIGHT - self.scr_width;
		let over_size_y = real_height + WALL_TOP + WALL_BOTTOM - self.scr_height;

		// カーソルがある程度進んだらスクロールを開始する
		let mousepos_x = (self.mouse_pos.x - WALL_LEFT - SCROLL_LEFT).max(0.0);
		let mousepos_y = (self.mouse_pos.y - WALL_TOP - SCROLL_TOP).max(0.0);

		// カーソルが移動できる幅を求める
		let mouse_move_x = self.scr_width - SCROLL_LEFT * 2.0 - WALL_LEFT;
		let mouse_move_y = self.scr_height - SCROLL_TOP * 2.0 - WALL_BOTTOM;
		
		// カーソルの移動速度を求める
		let move_x = over_size_x / mouse_move_x;
		let move_y = over_size_y / mouse_move_y;

		// 原点側にスクロールしすぎないよう、盤面の幅から最小座標を求める
		let min_left= self.scr_width - real_width - WALL_RIGHT;
		let min_top = self.scr_height - real_height - WALL_BOTTOM;

		// オフセットに反映する
		// このとき盤面が壁のサイズ（WALL_XXXX）を超えてスクロールしないよう制御する
		self.tbldt.offs.x = ((WALL_LEFT - mousepos_x * move_x).max(min_left)).min(WALL_LEFT);
		self.tbldt.offs.y = ((WALL_TOP - mousepos_y * move_y).max(min_top)).min(WALL_TOP);

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
			x: (self.mouse_pos.x - self.tbldt.offs.x) * (1.0 / self.tbldt.zoom.x),
			y: (self.mouse_pos.y - self.tbldt.offs.y) * (1.0 / self.tbldt.zoom.y),
		};
		self.cursol = self.tbldt.table.set_mousepos(tablepos);
	}

	//------------------------------
	// キーボード入力処理
	//------------------------------
	fn keycontrol(&mut self) -> bool {
		let mut is_update = false;

		//--- UNDO 処理 ---//
		if self.get_chkbox_flg(ChkBoxType::UndoFlg) {
			if is_key_pressed(KeyCode::U) {
				// UNDO情報の最新＝現在なので、UNDO中でなければ
				// １回余計に UNDO する
				if !self.tbldt.table.is_useundo() {
					self.tbldt.table.table_undo();
				}
				self.tbldt.table.table_undo();
				self.stat = GameStat::PLAYNG;
				return false;
			}

			// REDO 処理
			if is_key_pressed(KeyCode::R) {
				self.tbldt.table.table_redo();
				return false;
			}
		}

		// 上キーでズームアウト
		if is_key_pressed(KeyCode::Up) {
			self.tbldt.zoom.x += 0.2;
			self.tbldt.zoom.y += 0.2;
		}
		// 下キーでズームイン
		if is_key_pressed(KeyCode::Down) {
			if self.tbldt.zoom.x > MIN_ZOOM {
				self.tbldt.zoom.x -= 0.2;
				self.tbldt.zoom.y -= 0.2;
			}
		}

		is_update
	}

	//------------------------------
	// 盤面右クリック処理
	// 変更があった場合 true、ない場合は false を返す
	//------------------------------
	fn click_tbl_right (&mut self) -> bool {

		// マウス右クリックされていない、マウスが盤面上ではない、なら何もしない
		if !is_mouse_button_pressed(MouseButton::Right) ||
			self.cursol.index == -1 {
			return false
		}

		// クリックしたことを盤面に伝える
		let result = self.tbldt.table.click_right();
		result
	}

	//------------------------------
	// 盤面左クリック処理
	// 変更があった場合 true、ない場合は false を返す
	//------------------------------
	fn click_tbl_left (&mut self) -> bool {
		let mut is_update = false;

		// マウス左クリックされていないなら何もしない
		if !is_mouse_button_pressed(MouseButton::Left) {
			return is_update
		}

		// チェックボックス判定
		for index in 0..self.chkbox.len() {
			is_update |= self.chkbox[index].click(self.mouse_pos.x, self.mouse_pos.y);
		}

		// カーソルが盤面外ならなにもしない
		if self.cursol.index== -1 {
			return is_update
		}

		// ゲームが待機中なら初期化しなおす
		if self.stat == GameStat::SUCCESS || self.stat == GameStat::FAILED {
			self.initial_game();
			self.stat = GameStat::START;
			return true
		}

		// ゲームは開始し、クリック待ちなら爆弾を生成する
		if self.stat == GameStat::START {
			let mut table_backup: Vec<Panel> = Vec::new();

			// 初手ある程度開かせる
			let target = self.tbldt.width * self.tbldt.height * 5 / 100;
			let mut max = 0;
			for x in 0..100 {
				self.tbldt.table.setting_bom(self.tbldt.bom_num);
				self.tbldt.table.click_left();
				let opennum = self.tbldt.table.get_opennum();
				// 最も開いたパターンを保持しておく
				if max < opennum {
					max = opennum;
					table_backup = self.tbldt.table.tbl_backup();
				}
				if target <= opennum as i32 {
					break;
				}
				self.initial_game();
			}
			// 最も開いた盤面を復旧
			self.tbldt.table.tbl_restore(table_backup);

			// 今の盤面を保存する
			self.tbldt.table.undo_push();
			self.stat = GameStat::PLAYNG;
		}

		// クリックしたことを盤面に伝える
		is_update |= self.tbldt.table.click_left();
		is_update
	}

	//------------------------------
	// ゲーム全体の描画
	//------------------------------
	pub fn draw(&mut self) {
		// 盤面全体を塗りつぶす
		clear_window(LAYOUT_COLOR);

		// 盤面描画
		self.draw_table();

		// メニュー表示
		draw_rectangle(0.0, 0.0, self.scr_width, WALL_TOP - 10.0, LAYOUT_COLOR.with_alpha(0.8));
		draw_rectangle(0.0, WALL_TOP - 10.0, WALL_LEFT - 10.0, self.scr_height + 20.0, LAYOUT_COLOR.with_alpha(0.8));
		let flag_num = self.tbldt.table.get_num_redflag();
		let text = format!("SIZE:{}x{}  BOMB:{}  RED FLAG:{}",
			self.tbldt.width, self.tbldt.height,
			self.tbldt.bom_num, flag_num);
		dr_text(&text, 0.0, 0.0, FONT_SIZE, (200,200,255,255), (0,0,0,255));

		// ゲームの状態表示
		let bg = (0,0,0,255);
		let (text, fg):(&str, (u8,u8,u8,u8)) =
			if self.stat == GameStat::SUCCESS {
				("[ CLEAR!! ]", (0,255,0,255))
			} else if self.stat == GameStat::FAILED {
				("[ FAILED!! ]", (255,0,0,255))
			} else {
				("", (0,0,0,0))
			};
		dr_text(text, 0.0, 35.0, FONT_SIZE, fg, bg);

		// チェックボックスを表示する
		for chkbox in &self.chkbox {
			chkbox.draw();
		}
	
		// UNDO がオン
		if self.get_chkbox_flg(ChkBoxType::UndoFlg) {
			drawtextln(&format!("'U'=UNDO/'R'=REDO"), 1, 17, FONT_SIZE, BLACK);
		}

		// マウス位置表示
		draw_circle(self.mouse_pos.x, self.mouse_pos.y, 5.0, BLACK);

		// デバッグ
		dr_text(&format!("SCREEN:{},{} ZOOM:{},{}",self.scr_width,self.scr_height,self.tbldt.zoom.x, self.tbldt.zoom.y),
			0.0,500.0,FONT_SIZE,(255,255,255,255),(0,0,0,255));
		dr_text(&format!("MOUSE:{},{}",self.mouse_pos.x,self.mouse_pos.y),
			0.0,530.0,FONT_SIZE,(255,255,255,255),(0,0,0,255));
		dr_text(&format!("CURSOL:{},{}:{}",self.cursol.x,self.cursol.y,self.cursol.index),
			0.0,560.0,FONT_SIZE,(255,255,255,255),(0,0,0,255));

	}

	//------------------------------
	// 盤面の描画
	//------------------------------
	fn draw_table(&self) {
		// カメラをセット
		let zoom = Vec2 {
			x: self.tbldt.zoom.x * 2.0 / screen_width(),
			y: self.tbldt.zoom.y * 2.0 / screen_height(),
		};
		let offset = Vec2 {
			x: self.tbldt.offs.x * 2.0 / screen_width() - 1.0,
			y: - (self.tbldt.offs.y * 2.0 / screen_height()) + 1.0,
		};
		let camera = Camera2D {
			zoom, offset,
			..Default::default()
		};
		set_camera(&camera);

		// 縁取り
		let offs = 10.0;
		draw_rectangle( -offs, -offs,
			self.tbldt.width as f32 * PANEL_WIDTH + offs * 2.0,
			self.tbldt.height as f32 * PANEL_HEIGHT + offs * 2.0, BLUE);

		// 盤面の描画
		self.tbldt.table.draw_panel(self.get_chkbox_flg(ChkBoxType::DispAll));

		// カーソル周りに枠を表示
		if self.get_chkbox_flg(ChkBoxType::CursolFlg) {
			self.tbldt.table.draw_curasol();
		}

		// カメラをリセット
		set_default_camera();
	}
}