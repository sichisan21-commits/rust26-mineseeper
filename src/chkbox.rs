use macroquad::prelude::*;
use crate::draw::*;
use crate::utils::*;

struct ChkBox<T>{
	is_active: bool,							// 有効／無効
	mytype: T,									// チェックボックスのタイプ
	parent: Option<T>,							// 親となるチェックボックス
	text: String,								// 表示文字列
	flg: bool,									// チェックの状態
	pos: PosTable,								// 実座標
	size: PosTable,								// 当たり判定のサイズ
	offs: PosTable,								// オフセット（横）
	fsize: f32,									// フォントサイズ
	fgcol: String,								// 前面色
	bgcol: String,								// 輪郭色
	viewbox: bool,								// [*] の表示有無
	hitbox: bool,								// 当たり判定表示
	help_txt: Vec<String>,						// 説明文
}

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

		// 初期値の設定
		let viewbox = true;
		
		// チェックボックスの座標を決める
		let mut pos = PosTable{x: self.pos.x, y:self.pos.y};
		pos.y += self.size.y * self.chkboxs.len() as f32;

		// 生成
		self.chkboxs.push(ChkBox {
			mytype,
			parent: None,
			is_active: true,
			viewbox,
			text,
			offs: PosTable{x: 0.0, y: 0.0},
			fsize: self.fsize,
			fgcol: self.fgcol.clone(),
			bgcol: self.bgcol.clone(),
			flg,
			pos,
			size: self.size,
			hitbox: false,
			help_txt: Vec::new(),
			});
		// チェックボックスの座標更新
		self.calc_position();
	}

	//--------------------------------------------------
	// 子のチェックボックス追加
	//--------------------------------------------------
	pub fn addsub(&mut self, mytype:T, parent: T, text:String, flg: bool) {

		// 初期値の設定
		let viewbox = true;
		let mut is_active = false;
		for chkbox in &self.chkboxs {
			if chkbox.get_type() == parent {
				is_active = chkbox.get_flg();				
			}
		}

		// 生成
		self.chkboxs.push(ChkBox {
			mytype,
			parent: Some(parent),
			is_active,
			viewbox,
			text,
			offs: PosTable{x: 30.0, y: 0.0},
			fsize: self.fsize,
			fgcol: self.fgcol.clone(),
			bgcol: self.bgcol.clone(),
			flg,
			pos: PosTable{x: self.pos.x, y: 0.0},
			size: self.size,
			hitbox: false,
			help_txt: Vec::new(),
			});
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

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl<T> ChkBox<T>
    where
        T: Copy + PartialEq,
	{
	//--------------------------------------------------
	// マウスオーバー判定
	//--------------------------------------------------
	fn is_mouse_over(&self, mouse_x:f32, mouse_y: f32) -> bool {
		// 無効化されている場合判定しない
		if !self.is_active {
			return false
		}
		let left = self.pos.x + self.offs.x;
		let top = self.pos.y + self.offs.y;
		let right = left + self.size.x;
		let bottom = top + self.size.y;
		if mouse_x >= left && mouse_x <= right &&
		   mouse_y >= top && mouse_y <= bottom {
			return true
		}
		false
	}

	//--------------------------------------------------
	// チェックボックスをクリック（座標が一致していれば）
	//--------------------------------------------------
	fn click(&mut self) {
		if self.is_active {
			self.flg ^= true;
		}
	}

	//--------------------------------------------------
	// 色の設定
	//--------------------------------------------------
	pub fn set_col(&mut self, fgcol: String, bgcol: String) {
		if fgcol != "" {
			self.fgcol = fgcol.to_string();
		}
		if bgcol != "" {
			self.bgcol = bgcol.to_string();
		}
	}

	//--------------------------------------------------
	// 色の設定
	//--------------------------------------------------
	pub fn set_help(&mut self, help_txt: Vec<String>) {
		self.help_txt = help_txt;
	}

	//------------------------------
	// ヘルプテキストの返却
	//------------------------------
	pub fn get_help(&self) -> &[String] {
		&self.help_txt
	}

	//--------------------------------------------------
	// 座標を取得する
	//--------------------------------------------------
	pub fn get_pos(&self) -> PosTable  {
		self.pos
	}

	//--------------------------------------------------
	// 座標を設定する
	//--------------------------------------------------
	pub fn set_pos(&mut self, pos:PosTable)  {
		self.pos = pos;
	}

	//--------------------------------------------------
	// 上方向の余白を設定する
	//--------------------------------------------------
	pub fn set_offs(&mut self, offs: PosTable)  {
		self.offs = offs;
	}

	//--------------------------------------------------
	// 上方向の余白を設定する
	//--------------------------------------------------
	pub fn get_offs(&self) -> PosTable {
		self.offs
	}

	//--------------------------------------------------
	// サイズを取得する
	//--------------------------------------------------
	pub fn get_size(&self) -> PosTable  {
		self.size
	}

	//--------------------------------------------------
	// 有効無効を設定する
	//--------------------------------------------------
	pub fn active(&mut self, flg: bool)  {
		self.is_active = flg;
	}

	//--------------------------------------------------
	// 有効無効を返却する
	//--------------------------------------------------
	pub fn is_active(&self) -> bool {
		self.is_active
	}

	//--------------------------------------------------
	// 当たり判定表示
	//--------------------------------------------------
	pub fn view_hitbox(&mut self, flg: bool)  {
		self.hitbox = flg;
	}

	//--------------------------------------------------
	// タイプを返却する
	//--------------------------------------------------
	pub fn view_box(&mut self, viewbox: bool)  {
		self.viewbox = viewbox;
	}

	//--------------------------------------------------
	// タイプを返却する
	//--------------------------------------------------
	pub fn get_type(&self) -> T {
		self.mytype
	}

	//--------------------------------------------------
	// 親のタイプを返却する
	//--------------------------------------------------
	pub fn get_parent(&self) -> Option<T> {
		self.parent
	}

	//--------------------------------------------------
	// フラグを返却する
	//--------------------------------------------------
	pub fn get_flg(&self) -> bool {
		// 無効の場合「偽」を返す
		if !self.is_active {
			return false
		}
		self.flg
	}

	//--------------------------------------------------
	// フラグを返却する
	//--------------------------------------------------
	pub fn set_flg(&mut self, flg: bool) {
		self.flg = flg;
	}

	//--------------------------------------------------
	// 描画
	//--------------------------------------------------
	pub fn draw(&self, myfont: &Font) {
		// 無効ならなにもしない
		if !self.is_active {
			return
		}

		// チェック表示あり
		let check = {
			if !self.viewbox {
				""
			} else if self.flg {
				"[*]"
			} else {
				"[ ]"
			}
		};

		// 無効の場合薄くする
		let mut fg = self.fgcol.clone();
		let mut bg = self.bgcol.clone();
		if !self.is_active {
			fg.replace_range(6..8, "64");		
			bg.replace_range(6..8, "64");		
		}

		// 描画
		dr_text_ex(&format!("{}{}",check, self.text),
			self.pos.x + self.offs.x, self.pos.y + self.offs.y,
			self.fsize, &fg, &bg, myfont);

		// 当たり判定表示
		if self.hitbox {
			let left = self.pos.x + self.offs.x;
			let top = self.pos.y + self.offs.y;
			draw_rectangle_lines(left, top, self.size.x, self.size.y, 3.0, RED);
		}
	}

}
