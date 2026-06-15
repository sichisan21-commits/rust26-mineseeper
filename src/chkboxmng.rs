use macroquad::prelude::*;
use crate::utils::*;
use crate::chkbox::ChkBox;

pub struct ChkBoxMng<'a,T> {					// 管理テーブル
	chkboxs: Vec<ChkBox<T>>,					// チェックボックス配列
	pos: PosTable,								// 起点座標
	size: PosTable,								// 縦横サイズ
	fsize: f32,									// フォントサイズ
	fgcol: String,								// 基本色（前）
	bgcol: String,								// 基本色（後）
	myfont: &'a Font,							// フォント情報
}

//--------------------------------------------------
// チェックボックス管理テーブル
//--------------------------------------------------
impl<'a,T> ChkBoxMng<'a,T>
    where
        T: std::fmt::Debug,
        T: Copy + PartialEq,
	{
	//--------------------------------------------------
	// 初期化
	//--------------------------------------------------
	pub fn new(myfont:&'a Font) -> ChkBoxMng<'a,T> {
		ChkBoxMng {
			chkboxs: Vec::new(),
			pos: PosTable{x: 0.0, y:0.0},
			size: PosTable{x:0.0, y:0.0},
			fsize: 0.0,
			fgcol: String::new(),
			bgcol: String::new(),
			myfont,
		}
	}

	//--------------------------------------------------
	// チェックボックス追加
	//--------------------------------------------------
	pub fn set_base(&mut self, left: f32, top: f32, width: f32, height: f32, fsize: f32, fgcol:&str, bgcol:&str) {
		self.pos = PosTable{x: left, y: top};
		self.size = PosTable{x:width, y:height};
		self.fsize = fsize;
		self.fgcol = fgcol.to_string();
		self.bgcol = bgcol.to_string();
	}

	//--------------------------------------------------
	// チェックボックス追加
	//--------------------------------------------------
	pub fn add(&mut self, mytype:T, text:String, flg: bool) {
		// チェックボックスの座標を決める
		let mut pos = PosTable{x: self.pos.x, y:self.pos.y};
		pos.y += self.size.y * self.chkboxs.len() as f32;

		// 生成
		let chkbox = ChkBox::new(
            mytype,
            None,
            true,
			text,
			self.fsize,
			flg,
			pos,
			self.size,
			self.fgcol.clone(),
			self.bgcol.clone(),
		);			
		self.chkboxs.push(chkbox);

		// チェックボックスの座標更新
		self.calc_position();
	}

	//--------------------------------------------------
	// 子のチェックボックス追加
	//--------------------------------------------------
	pub fn addsub(&mut self, mytype:T, parent: T, text:String, flg: bool) {

		// 初期値の設定
		let mut is_active = false;
		for chkbox in &self.chkboxs {
			if chkbox.get_type() == parent {
				is_active = chkbox.get_flg();				
			}
		}

		// チェックボックスの座標を決める
		let mut pos = PosTable{x: self.pos.x, y:self.pos.y};
		pos.x += 30.0;
		pos.y += self.size.y * self.chkboxs.len() as f32;

		// 生成
		let chkbox = ChkBox::new(
			mytype,
			Some(parent),
			is_active,
			text,
			self.fsize,
			flg,
			pos,
			self.size,
			self.fgcol.clone(),
			self.bgcol.clone(),
		);
		self.chkboxs.push(chkbox);

		// チェックボックスの座標更新
		self.calc_position();
	}

	//--------------------------------------------------
	// チェックボックスの縦座標を計算しなおす
	//--------------------------------------------------
	fn calc_position(&mut self) {
		let mut pos_x = self.pos.x;
		let mut pos_y = self.pos.y;

		for parent in 0..self.chkboxs.len() {
			// 無効の場合なにもしない
			if !self.chkboxs[parent].is_active() {
				continue;
			}

			// 子のチェックボックスの場合何もしない
			if let Some(_) = self.chkboxs[parent].get_parent() {
				continue
			}

			// 自分自身の座標を更新する
			let mut pos = self.chkboxs[parent].get_pos();
			let offs = self.chkboxs[parent].get_offs();
			let size = self.chkboxs[parent].get_size();
			pos.x = pos_x;
			pos.y = pos_y;
			self.chkboxs[parent].set_pos(pos);
			pos_x += offs.x;
			pos_y += size.y + offs.y;

			// 子供のチェックボックスの座標更新
			for child in 0..self.chkboxs.len() {
				if !self.chkboxs[child].is_active() {
					continue;
				}
				if let Some(parent_type) = self.chkboxs[child].get_parent() {
					if !(parent_type == self.chkboxs[parent].get_type()) {
						continue;
					} 
				// 子の座標を更新する
				let mut pos = self.chkboxs[child].get_pos();
				let size = self.chkboxs[child].get_size();
				pos.y = pos_y;
				self.chkboxs[child].set_pos(pos);
				pos_y += size.y;
				}
			}
		}
	}

	//------------------------------
	// チェックボックスのチェックマークオン／オフ
	//------------------------------
	pub fn view_hitbox(&mut self, flg: bool) {
		for chkbox in &mut self.chkboxs {
			chkbox.view_hitbox(flg);
		}
	}

	//------------------------------
	// チェックボックスのチェックマークオン／オフ
	//------------------------------
	pub fn view_box(&mut self, mytype: T, boxon: bool) {
		for chkbox in &mut self.chkboxs {
			if chkbox.get_type() == mytype {
				chkbox.view_box(boxon);
			}
		}
	}

	//------------------------------
	// 色の設定
	//------------------------------
	pub fn set_col(&mut self, mytype: T, fgcol: &str, bgcol: &str) {
		for chkbox in &mut self.chkboxs {
			if chkbox.get_type() == mytype {
				chkbox.set_col(fgcol.to_string(), bgcol.to_string());
			}
		}
	}
		
	//------------------------------
	// 色の設定
	//------------------------------
	pub fn set_offs(&mut self, mytype: T, offs_x: f32, offs_y: f32) {
		for chkbox in &mut self.chkboxs {
			if chkbox.get_type() == mytype {
				chkbox.set_offs(PosTable{x:offs_x, y:offs_y});
			}
		}
	}

	//------------------------------
	// ヘルプテキストの設定
	//------------------------------
	pub fn set_help(&mut self, mytype: T, help_txt:&str) {
		for chkbox in &mut self.chkboxs {
			if chkbox.get_type() == mytype {
				let lines: Vec<String> = help_txt
					.split('\n')
    				.map(|s| s.to_string())
    				.collect();
				chkbox.set_help(lines);
			}
		}
	}

	//------------------------------
	// チェックボックスの有効無効変更
	//------------------------------
	pub fn active(&mut self, mytype: T, flg: bool) {
		for chkbox in &mut self.chkboxs {
			if chkbox.get_type() == mytype {
				chkbox.active(flg);
			}
		}
	}

	//------------------------------
	// チェックボックスからフラグを取得
	//------------------------------
	pub fn get_flg(&self, mytype: T) -> bool {
		for chkbox in &self.chkboxs {
			if chkbox.get_type() == mytype {
				return chkbox.get_flg()
			}
		}
		false
	}

	//------------------------------
	// チェックボックスへフラグ設定
	//------------------------------
	pub fn set_flg(&mut self, mytype: T, flg: bool) {
		for index in 0..self.chkboxs.len() {
			if self.chkboxs[index].get_type() == mytype {
				self.chkboxs[index].set_flg(flg);
				self.child_onoff(index);
			}
		}
	}

	//------------------------------
	// 全チェックボックスのフラグクリア
	//------------------------------
	pub fn clear_flg(&mut self) {
		for chkbox in &mut self.chkboxs {
			chkbox.set_flg(false);
		}
	}

	//------------------------------
	// クリック判定
	//------------------------------
	pub fn click(&mut self, mouse_x: f32, mouse_y: f32) -> Option<(T, bool)> {
		// 全てのチェックボックスのクリック判定
		for parent in 0..self.chkboxs.len() {
			// 対象のチェックボックスがクリックされた
			if self.chkboxs[parent].is_mouse_over(mouse_x, mouse_y) {
				self.chkboxs[parent].click();
				// 子のチェックボックスへ連携
				self.child_onoff(parent);
				// クリック判定された場合タイプとフラグを返却
				return Some((self.chkboxs[parent].get_type(), self.chkboxs[parent].get_flg()));
			}
		}
		None
	}

	//------------------------------
	// マウスオーバーしているチェックボックスのヘルプを取得する
	//------------------------------
	pub fn gethelp(&self, mouse_x: f32, mouse_y: f32) -> Option<(T, &[String])> {
		// 全てのチェックボックスのクリック判定
		for parent in 0..self.chkboxs.len() {
			// マウスオーバーを判定する
			if self.chkboxs[parent].is_mouse_over(mouse_x, mouse_y) {
				// クリック判定された場合タイプとフラグを返却
				return Some((self.chkboxs[parent].get_type(), self.chkboxs[parent].get_help()));
			}
		}
		None
	}

	//------------------------------
	// 子のチェックボックス有効・無効
	//------------------------------
	fn child_onoff(&mut self, parent: usize) {
		let mut is_update = false;
		let flg = self.chkboxs[parent].get_flg();

		// 子供のチェックボックスを有効化
		for child in 0..self.chkboxs.len() {
			if let Some(parent_type) = self.chkboxs[child].get_parent() {
				if parent_type == self.chkboxs[parent].get_type(){
					self.chkboxs[child].active(flg);
					is_update = true;
				}
			}
		}

		// クリック判定された場合座標を更新
		if is_update {
			self.calc_position();
		}
	}

	//------------------------------
	// 全チェックボックス描画
	//------------------------------
	pub fn draw(&self) {
		for chkbox in &self.chkboxs {
			chkbox.draw(self.myfont);
		}
	}
}