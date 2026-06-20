use macroquad::prelude::*;
use crate::panel::Panel;
use crate::utils::*;
use crate::myconst::*;

#[derive(Debug, Copy, Clone)]
pub struct MyCursol {                               // カーソル情報
	pub x: i32,                                     // 横位置
	pub y: i32,                                     // 縦位置
	pub index: i32,                                 // インデックス
}

// ゲーム情報
pub struct GameTable {
	width: i32,                             // 盤面の幅
	height: i32,                            // 盤面の高さ
	num_bom: i32,                           // 爆弾の数
	mouse_pos: Vec2,                        // マウス位置
	cursol: MyCursol,                       // カーソル位置
	panels: Vec<Panel>,                     // 盤面データ
	panels_undo: Vec<Vec<Panel>>,			// 盤面データ（履歴）
	useundo: usize,                         // 使用されるundo番号
}

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl GameTable {
	//------------------------------
	// 初期化
	// width＝盤面の幅
	// height＝満面の高さ
	// bom_num＝爆弾の数
	//------------------------------
	pub fn new(width:i32, height:i32, num_bom:i32) -> GameTable {
		GameTable {
			width,
			height,
			num_bom,
			mouse_pos: Vec2 {x:0.0, y:0.0},
			cursol: MyCursol {x:0, y:0, index:0},
			panels: Vec::new(),
			panels_undo: Vec::new(),
			useundo: 0,
		}
	}

	//------------------------------
	// 盤面の初期化
	//------------------------------
	pub fn initial(&mut self) {
		// 盤面を初期化する
		self.panels.clear();
		for y in 0..self.height {
			for x in 0..self.width {
				self.panels.push(Panel::new(x, y, self.width, self.height));
			}
		}
	}

	//------------------------------
	// カーソル位置をセットする
	//------------------------------
	pub fn set_mousepos(&mut self, mouse_pos: Vec2) -> MyCursol {
		self.mouse_pos = mouse_pos;
		let cursol_x = (mouse_pos.x / PANEL_WIDTH) as i32;
		let cursol_y = (mouse_pos.y / PANEL_HEIGHT) as i32;
		self.cursol = MyCursol {
			x: cursol_x, y: cursol_y,
			index: get_index(cursol_x, cursol_y, self.width, self.height),
		};
		self.cursol
	}

/*
	//------------------------------
	// 爆弾を配置する(テスト用)
	//------------------------------
	pub fn setting_bom(&mut self, num_bom: i32) {
		self.bomon(0, 0);
		self.bomon(7, 0);
		self.bomon(2, 3);
		self.bomon(4, 3);
		self.bomon(7, 3);
	}

	//------------------------------
	// 爆弾を配置する(テスト用)
	//------------------------------
	pub fn _setting_bom(&mut self, num_bom: i32) {
		for x in 0..self.width {
			for y in 0..self.height {
				if y > 2  || x > 6 {
					if x % 3 == 0 || x > 6{
						self.bomon(x, y);
					}
				}
			}
		}
	}
 */

	//------------------------------
	// 爆弾を配置する
	//------------------------------
	pub fn setting_bom(&mut self, num_bom: i32) {
		self. num_bom = num_bom;

		// 爆弾をランダム生成する
		for _ in 0..self.num_bom{
			loop {
				// 爆弾位置をランダム生成する
				let land_x = rand::gen_range(0, self.width);
				let land_y = rand::gen_range(0, self.height);

				// カーソル位置と一致する位置には配置しない
				if land_x == self.cursol.x && land_y == self.cursol.y {
					continue;
				}

				// 爆弾を配置
				if self.bomon(land_x, land_y) {
					break;
				}
			}
		}
		self.panels_undo.clear();
		self.useundo = 0;
	}

	//------------------------------
	// 指定のマスに爆弾を置く
	//------------------------------
	fn bomon(&mut self, pos_x:i32, pos_y:i32) -> bool {
		// 座標不正または配置済みの場合 false
		let tblpos = get_index(pos_x, pos_y, self.width, self.height);
		if tblpos == -1 || self.panels[tblpos as usize].is_bom() {
			return false;
		}

		// 該当パネルに爆弾を置き
		self.panels[tblpos as usize].bomon();

		// 周囲のパネルのカウントを増やす
		let around = self.panels[tblpos as usize].get_around_tbl();      
		for index in around.into_iter().flatten() {
			if index != -1 {
				self.panels[index as usize].num_up();
			}
		}
		true
	}

	//------------------------------
	// 開かれたパネルの数を取得
	//------------------------------
	pub fn get_opennum(&self) -> usize {
		self.panels
			.iter()
			.filter(|p| p.is_open())
			.count()
	}

	//------------------------------
	// 赤い旗の数を取得
	//------------------------------
	pub fn get_num_redflag(&self) -> usize {
		self.panels
			.iter()
			.filter(|p| p.get_userflg() == UserFlg::RedFlg)
			.count()
	}

	//------------------------------
	// すべての危険マスに旗を立てる
	//------------------------------
	pub fn set_all_redflag(&mut self) {
		for index in 0..self.width * self.height {
			if self.panels[index as usize].get_autoflag() == AutoSts::Danger {
				self.panels[index as usize].set_userflg(UserFlg::RedFlg);
			}
		}
	}

	//------------------------------
	// 盤面のバックアップ
	//------------------------------
	pub fn tbl_backup(&self) -> Vec<Panel> {
		self.panels.clone()
	}

	//------------------------------
	// バックアップから復帰
	//------------------------------
	pub fn tbl_restore(&mut self, table:Vec<Panel>) {
		self.panels = table.clone();
	}

	//------------------------------
	// 今の盤面をアンドゥ領域に保持
	//------------------------------
	pub fn undo_push(&mut self) {
		// 今参照されている UNDO 情報より後ろは破棄
		self.panels_undo.truncate(self.useundo + 1);

		// 最後に保存した UNDO 情報と今の盤面が一致するなら UNDO 情報は保持しない
		if let Some(last) = self.panels_undo.last() {
			if last == &self.panels {
				return;
			}
		}

		// UNDO 情報を追加
		self.panels_undo.push(self.panels.clone());
		self.useundo = self.panels_undo.len();
	}

	//------------------------------
	// テーブルをUNDOする
	//------------------------------
	pub fn table_undo(&mut self) {
		if self.useundo > 0 {
			// 一番最後の履歴へ戻す
			self.useundo -= 1;
			self.panels = self.panels_undo[self.useundo].clone();
		}
	}

	//------------------------------
	// テーブルをREDOする
	//------------------------------
	pub fn table_redo(&mut self) {
		if self.useundo < self.panels_undo.len() -1 {
			// 次の履歴へ戻す
			self.useundo += 1;
			self.panels = self.panels_undo[self.useundo].clone();
		}
	}

	//------------------------------
	// UNDO 実行中か
	//------------------------------
	pub fn is_useundo(&self) -> bool {
		self.useundo < self.panels_undo.len()
	}

	//------------------------------
	// 踏まれた爆弾数を取得
	//------------------------------
	pub fn open_bomnum(&mut self) -> usize {
		// 開かれている爆弾を数える
		self.panels
			.iter()
			.filter(|p| p.is_bomopen())
			.count()
	}

	//------------------------------
	// 左クリック処理
	// 変更があった場合 true、ない場合は false を返す
	//------------------------------
	pub fn click_left(&mut self) -> bool { 
		// クリック位置が盤面外、あるいはすでに開かれているなら何もしない
		// 赤い旗の立っているマスも開かせない
		if self.cursol.index == -1 ||
		   self.panels[self.cursol.index as usize].is_open() ||
		   self.panels[self.cursol.index as usize].get_userflg() == UserFlg::RedFlg {
			return false
		}

		// クリック位置を開く
		self.panels[self.cursol.index as usize].open();
		
		// クリック位置が爆弾の場合終了
		if self.panels[self.cursol.index as usize].is_bom() {
			return true
		}

		// クリック位置からパネルを連鎖的に開く
		self.openchain(self.cursol.index);
		true
	}

	//------------------------------
	// 左クリック（離す）処理
	// コードオープンを実施する
	//------------------------------
	pub fn clickoff_left(&mut self) -> bool { 
		// クリック位置が盤面外、あるいは開かれていないなら何もしない
		if self.cursol.index == -1 ||
		   !self.panels[self.cursol.index as usize].is_open() || 
		   self.panels[self.cursol.index as usize].is_bom() {
			return false
		}

		// クリック位置の周囲を開く
		let around_num = self.panels[self.cursol.index as usize].get_around_num();
		let around = self.panels[self.cursol.index as usize].get_around_tbl();

		// まず立っている赤の旗の数を数える
		let mut flg_num = 0;
		for index in around.into_iter().flatten() {
			if index == -1 {
				continue;
			}
			if self.panels[index as usize].get_userflg() == UserFlg::RedFlg {
				flg_num += 1;
			}
		}

		// 周囲の爆弾数と立てられている旗の数が同じでなければ何もしない
		if flg_num != around_num {
			return false
		}

		// 周囲の未開封マスを開封する
		for index in around.into_iter().flatten() {
			if index != -1 && self.panels[index as usize].get_userflg() != UserFlg::RedFlg {
				self.panels[index as usize].open();
				self.openchain(index);
			}
		}

		true
	}

	//------------------------------
	// 右クリック処理
	// 変更があった場合 true、ない場合は false を返す
	//------------------------------
	pub fn click_right(&mut self, use_blueflg: bool, blueflg_first:bool) -> bool {
		// 座標をインデックスに変換
		// クリック位置が盤面外、あるいはすでに開かれているなら何もしない
		if self.cursol.index == -1 || self.panels[self.cursol.index as usize].is_open() {
			return false
		}

		// 旗の操作
		self.panels[self.cursol.index as usize].userflg(use_blueflg, blueflg_first);
		true
	}

	//------------------------------
	// 全てのアシスト表示をクリアする
	//------------------------------
	pub fn assist_clear(&mut self) {
		for index in 0..self.width * self.height {
			self.panels[index as usize].bold_off();
			self.panels[index as usize].set_autoflag(AutoSts::None, false);
		}
	}

	//------------------------------
	// 全ての補助フラグをクリアする
	//------------------------------
	pub fn clear_blue_flag(&mut self) {
		for index in 0..self.width * self.height {
			if self.panels[index as usize].get_userflg() == UserFlg::BlueFlg {
				self.panels[index as usize].set_userflg(UserFlg::None);
			}
		}
	}

	//------------------------------
	// 連鎖的に開く
	// 盤面が大きくなると再起呼び出しが深くなるので現在最大 30x30 を上限にしている
	// 再起呼び出ししない方式へ要検討
	//------------------------------
	fn openchain(&mut self, cursol_index: i32) {
		// クリック位置を開く
		self.panels[cursol_index as usize].open();

		// 周囲の爆弾数が０でなければ抜ける
		if self.panels[cursol_index as usize].get_around_num() != 0{
			return;
		}

		// 周囲をチェックし開いていく
		// 自身の座標情報と周りのインデックス配列を取得
		let around = self.panels[cursol_index as usize].get_around_tbl();
		for index in around.into_iter().flatten() {
			// 盤面外ならスキップ
			// 爆弾マスはスキップ（自動で開かない）
			// 既に開かれている
			if index == -1 || self.panels[index as usize].is_bom() ||
			   self.panels[index as usize].is_open() {
				continue;
			}

			// 参照位置を開く
			self.openchain(index);
		}
	}

	//------------------------------
	// 盤面を描画する
	//------------------------------
	pub fn draw_panel(&self, is_allhint:bool, is_dangon:bool, is_safeon:bool, hide_lv:i32, hidenum_lv:i32, is_playing:bool) {
		// 盤面を表示
		for panel in &self.panels {
			panel.draw_panel(self.cursol.x, self.cursol.y, is_allhint, is_dangon, is_safeon, hide_lv, hidenum_lv, is_playing);
		}
	}

	//------------------------------
	// カーソルを描画する
	//------------------------------
	pub fn draw_curasol(&self) {
		let border = 6.0;
		// 一旦上下左右のパネル位置を求める
		let left  = self.cursol.x.clamp(0,self.width - 1);
		let top  = self.cursol.y.clamp(0,self.height - 1);

		// カーソル枠の描画
		draw_rectangle_lines(
			(left - 1) as f32 * PANEL_WIDTH, (top - 1) as f32 * PANEL_HEIGHT,
			PANEL_WIDTH * 3.0, PANEL_HEIGHT * 3.0,
			border + 3.0, RED);
	}
}