use macroquad::prelude::*;
use crate::myconst::*;
use crate::draw::*;


struct ChkBox{
	mytype: ChkBoxType,							// チェックボックスのタイプ
	viewbox: bool,								// [*] の表示有無
	text: String,								// 表示文字列
	left: f32,									// 表示左位置
	top: f32,									// 表示上位置
	size: f32,									// フォントサイズ
	fgcol: (u8,u8,u8,u8),						// 前面色
	bgcol: (u8,u8,u8,u8),						// 輪郭色
	flg: bool,									// チェックの状態
}

pub struct ChkBoxMng {						// 管理テーブル
	chkboxs: Vec<ChkBox>,					// チェックボックス配列
}

//--------------------------------------------------
// 管理テーブル
//--------------------------------------------------
impl ChkBoxMng{
	//--------------------------------------------------
	// 初期化
	//--------------------------------------------------
	pub fn new() -> ChkBoxMng {
		ChkBoxMng {
			chkboxs: Vec::new(),
		}
	}

	//--------------------------------------------------
	// 追加
	//--------------------------------------------------
	pub fn add(&mut self,
		mytype: ChkBoxType, text: String, left: f32, top: f32, size: f32,
		fgcol: (u8,u8,u8,u8),bgcol: (u8,u8,u8,u8), flg: bool) {
			let boxon = true;
			self.chkboxs.push(ChkBox::new(
				mytype,
				boxon,
				text,
				left,
				top,
				size,
				fgcol,
				bgcol,
				flg));
	}

	//------------------------------
	// チェックボックスのチェックマークオン／オフ
	//------------------------------
	pub fn view_box(&mut self, mytype: ChkBoxType, boxon: bool) {
		for chkbox in &mut self.chkboxs {
			if chkbox.get_type() == mytype {
				chkbox.view_box(boxon);
			}
		}
	}

	//------------------------------
	// チェックボックスからフラグを取得
	//------------------------------
	pub fn get_flg(&self, mytype: ChkBoxType) -> bool {
		for chkbox in &self.chkboxs {
			if chkbox.get_type() == mytype {
				return chkbox.get_flg()
			}
		}
		false
	}

	//------------------------------
	// チェックボックスからフラグを取得
	//------------------------------
	pub fn set_flg(&mut self, mytype: ChkBoxType, flg: bool) {
		for chkbox in &mut self.chkboxs {
			if chkbox.get_type() == mytype {
				chkbox.set_flg(flg);
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
	pub fn click(&mut self, mouse_x: f32, mouse_y: f32) -> Option<(ChkBoxType, bool)> {
		// 全てのチェックボックスのクリック判定
		for chkbox in &mut self.chkboxs {
			if chkbox.click(mouse_x, mouse_y) {
				// クリック判定された場合タイプとフラグを返却
				return Some((chkbox.get_type(), chkbox.get_flg()));
			}
		}
		None
	}

	//------------------------------
	// 全チェックボックス描画
	//------------------------------
	pub fn draw(&self) {
		for chkbox in &self.chkboxs {
			chkbox.draw();
		}
	}
}

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl ChkBox {
	//--------------------------------------------------
	// 初期化
	//--------------------------------------------------
	pub fn new(mytype: ChkBoxType, viewbox: bool, text: String, left: f32, top: f32, size: f32,
		fgcol: (u8,u8,u8,u8),bgcol: (u8,u8,u8,u8), flg: bool) -> ChkBox {
		ChkBox {
			mytype,
			viewbox,
			text,
			left,
			top,
			size,
			fgcol,
			bgcol,
			flg,
		}
	}

	//--------------------------------------------------
	// チェックボックスをクリック（座標が一致していれば）
	//--------------------------------------------------
	fn click(&mut self, mouse_x:f32, mouse_y: f32) -> bool {
		let right = self.left + self.size as f32 * self.text.len() as f32 * 0.7;
		let bottom = self.top + self.size as f32;
		if mouse_x >= self.left && mouse_x <= right &&
		   mouse_y >= self.top && mouse_y <= bottom {
			self.flg ^= true;
			return true
		}
		false
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
	pub fn get_type(&self) -> ChkBoxType {
		self.mytype
	}

	//--------------------------------------------------
	// フラグを返却する
	//--------------------------------------------------
	pub fn get_flg(&self) -> bool {
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
	pub fn draw(&self) {
		let check = {
			if !self.viewbox {
				""
			} else if self.flg {
				"[*]"
			} else {
				"[-]"
			}
		};
		dr_text(&format!("{}{}",check, self.text),
			self.left, self.top, self.size, self.fgcol, self.bgcol);
	}

}
