// 自作チェックボックス
use macroquad::prelude::*;
use crate::utils::*;
use crate::draw::*;

pub struct ChkBox<T>{
	is_active: bool,							// 有効／無効
	is_lock: bool,								// 変更不可（値は有効）
	mytype: T,									// チェックボックスのタイプ（上位が識別するためのenumm）
	parent: Option<T>,							// 親となるチェックボックスのタイプ（＝mytype）
	text: String,								// 表示文字列
	flg: bool,									// チェックの状態
	fsize: f32,									// フォントサイズ
	fgcol: String,								// 前面色
	bgcol: String,								// 輪郭色
	is_absolute: bool,							// 絶対座標か相対座標か
	pos: PosTable,								// 表示座標
	size: PosTable,								// サイズ
	offs: PosTable,								// オフセット
	viewbox: bool,								// [*] の表示有無
	hitbox: bool,								// 当たり判定表示
	help_txt: String,							// 説明文
}

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl<T> ChkBox<T>
	where
		T: Copy + PartialEq,
	{

	//--------------------------------------------------
	// 初期化
	// mytype＝チェックボックスを識別するタイプ（上位が指定するenum）
	// parent＝親のタイプ（＝mytype）
	// is_active＝有効／無効
	// text＝表示テキスト
	// fsize＝フォントサイズ
	// flg＝チェックのオンオフ
	// pos＝チェックボックスの位置
	// is_absolute＝絶対座標か否か
	// offs＝オフセット（相対座標の場合）
	// size＝大きさ（当たり判定）
	// fgcol＝文字の色（RRGGBBAA形式のテキスト）
	// bgcol＝文字の縁の色（RRGGBBAA形式のテキスト）
	//--------------------------------------------------
	pub fn new(mytype:T, parent:Option<T>, is_active: bool, text:String, fsize:f32, flg: bool, pos:PosTable, is_absolute:bool, offs:PosTable,size:PosTable, fgcol:String, bgcol:String) -> ChkBox<T> {
		ChkBox {
			is_active,
			is_lock: false,
			mytype,
			text,
			pos,
			is_absolute,
			fsize,
			fgcol,
			bgcol,
			flg,
			parent,
			viewbox: true,
			offs,
			size,
			hitbox: false,
			help_txt: String::new(),
			}
	}
	
	//--------------------------------------------------
	// チェックボックスをクリック
	//--------------------------------------------------
	pub fn click(&mut self,  mouse: PosTable) -> bool {
		// チェックボックスが無効・またはロック中の場合はなにもしない
		if !self.is_active || self.is_lock {
			return false
		}

		// 当たり判定に合致していればクリック処理
		if self.is_mouse_over(mouse) {
			self.flg ^= true;
			true
		} else {
			false
		}
	}

	//--------------------------------------------------
	// マウスオーバー判定
	//--------------------------------------------------
	pub fn is_mouse_over(&self, mouse: PosTable) -> bool {
		// 無効化されている場合判定しない
		if !self.is_active {
			return false
		}

		// 当たり判定の算出
		let (min, max) = get_hitbox(self.pos, self.offs, self.size);
		if mouse.x >= min.x && mouse.x <= max.x &&
		   mouse.y >= min.y && mouse.y <= max.y {
			return true
		}
		false
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
	// 色の設定
	//--------------------------------------------------
	pub fn set_col(&mut self, fgcol: String, bgcol: String) {
		if fgcol != "" {
			self.fgcol = fgcol;
		}
		if bgcol != "" {
			self.bgcol = bgcol;
		}
	}

	//--------------------------------------------------
	// ヘルプテキストの設定
	// help_txt＝テキスト型の配列
	//--------------------------------------------------
	pub fn set_help(&mut self, help_txt: &str) {
		self.help_txt = help_txt.to_string();
	}

	//------------------------------
	// ヘルプテキストの返却
	//------------------------------
	pub fn get_help(&self) -> &str {
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
	// 絶対座標指定かどうか返却
	//--------------------------------------------------
	pub fn is_absolute(&self) -> bool {
		self.is_absolute
	}

	//--------------------------------------------------
	// オフセット（原点側の余白）を取得する
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
	pub fn set_active_flg(&mut self, flg: bool)  {
		self.is_active = flg;
	}

	//--------------------------------------------------
	// 有効無効を返却する
	//--------------------------------------------------
	pub fn is_active(&self) -> bool {
		self.is_active
	}

	//--------------------------------------------------
	// 変更の許可を設定する
	//--------------------------------------------------
	pub fn set_lock_flg(&mut self, flg: bool)  {
		self.is_lock = flg;
	}

	//--------------------------------------------------
	// 当たり判定表示
	//--------------------------------------------------
	pub fn view_hitbox(&mut self, flg: bool)  {
		self.hitbox = flg;
	}

	//--------------------------------------------------
	// チェック部分（[*]）の表示・非表示設定
	//--------------------------------------------------
	pub fn view_box(&mut self, viewbox: bool)  {
		self.viewbox = viewbox;
	}

	//--------------------------------------------------
	// チェックボックスのフラグを返却する
	//--------------------------------------------------
	pub fn get_flg(&self) -> bool {
		// 無効の場合「偽」を返す
		if !self.is_active {
			return false
		}
		self.flg
	}

	//--------------------------------------------------
	// チェックボックスのフラグを設定する
	//--------------------------------------------------
	pub fn set_flg(&mut self, flg: bool) {
		if self.is_active && !self.is_lock {
			self.flg = flg;
		}
	}

	//--------------------------------------------------
	// 描画
	//--------------------------------------------------
	pub fn draw(&self, myfont: &Font) {
		// 無効ならなにもしない
		if !self.is_active {
			return
		}

		// チェック表示ありなら文字列を作る
		let check = {
			if !self.viewbox {
				""
			} else if self.flg {
				"[＊]"
			} else {
				"[－]"
			}
		};

		// ロック中の場合薄く表示する
		let mut fgcol = self.fgcol.clone();
		let mut bgcol = self.bgcol.clone();
		if self.is_lock {
			fgcol.replace_range(6..8, "AA");
			bgcol.replace_range(6..8, "55");
		}

		// 描画
		dr_text_ex(&format!("{}{}",check, self.text),
			self.pos.x + self.offs.x, self.pos.y + self.offs.y,
			self.fsize, &fgcol, &bgcol, myfont);

		// 当たり判定表示
		if self.hitbox {
			let (min, max) = get_hitbox(self.pos, self.offs, self.size);
			dr_rect(min.x, min.y, max.x - min.x, max.y - min.y, 3.0, "", "FF0000FF");
		}
	}

}

fn get_hitbox(pos: PosTable, offs: PosTable, size: PosTable) -> (PosTable, PosTable) {
	let min = PosTable {
		x: pos.x + offs.x,
		y: pos.y + offs.y,
		};
	let max = PosTable {
		x: pos.x + size.x + offs.x,
		y: pos.y + size.y * 0.8 + offs.y,
		};
	(min,max)
}