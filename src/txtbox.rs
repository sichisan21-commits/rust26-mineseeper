use macroquad::prelude::*;
use crate::draw::*;

pub struct TextBox {
	text: String,
	x: f32,
	y: f32,
	w: f32,
	h: f32,
	min: i32,
	max: i32,
	focused: bool,
	shift_on: bool,
}

impl TextBox {
	pub fn new(text: &str, x: f32, y: f32, w: f32, h: f32, min:i32, max:i32) -> Self {
		Self {
			text: text.to_string(),
			x, y, w, h, min, max,
			focused: false,
			shift_on: false,
		}
	}

	//--------------------------------------------------
	// 入力上限値を設定する
	//--------------------------------------------------
	pub fn set_max(&mut self, max: i32) {
		self.max = max;
	}

	//--------------------------------------------------
	// 対象にフォーカスする
	//--------------------------------------------------
	pub fn focus(&mut self) {
		self.focused = true;
	}

	//--------------------------------------------------
	// フォーカスを外す
	//--------------------------------------------------
	pub fn focus_off(&mut self) {
		self.focused = false;
	}

	//--------------------------------------------------
	// 対象にフォーカスする
	// 戻り値：（入力されたコード,シフトオンオフ）
	//--------------------------------------------------
	pub fn update(&mut self) -> (KeyCode, bool) {
		// シフトキーダウン・アップを記憶する
		if is_key_pressed(KeyCode::LeftShift) | is_key_pressed(KeyCode::RightShift){
			self.shift_on = true;
		}
		if is_key_released(KeyCode::LeftShift) | is_key_released(KeyCode::RightShift){
			self.shift_on = false;
		}

		// 最小最大チェック
		if !self.focused {
		   if self.text == "" || self.text.parse::<i32>().unwrap_or(0) < self.min {
				self.text = self.min.to_string();
		   }
			if self.text.parse::<i32>().unwrap_or(0) > self.max {
				self.text = self.max.to_string();
			}
		}

		// クリックでフォーカス
		if is_mouse_button_pressed(MouseButton::Left) {
			let (mx, my) = mouse_position();
			self.focused = mx >= self.x && mx <= self.x + self.w &&
						   my >= self.y && my <= self.y + self.h;
		}

		// フォーカスされていない場合キー受け付けせずに終了
		if !self.focused {
			return (KeyCode::Space, self.shift_on);
		}

		// フォーカス中だけ文字入力
		if self.focused {
			if let Some(c) = get_char_pressed() {
				if !c.is_control() {
					self.text.push(c);
				}
			}

			// 上下左右キー
			if is_key_pressed(KeyCode::Up) {
				// 上キーは１０増やす
				self.text = (self.text.parse::<i32>().unwrap_or(self.min) + 10).min(self.max).to_string();
			} else if is_key_pressed(KeyCode::Right) {
				// 右キーは１増やす
				self.text = (self.text.parse::<i32>().unwrap_or(self.min) + 1).min(self.max).to_string();
			} else if is_key_pressed(KeyCode::Down) {
				// 下キーは１０減らす
				self.text = (self.text.parse::<i32>().unwrap_or(self.min) - 10).max(self.min).to_string();
			} else if is_key_pressed(KeyCode::Left){
				// 左キーは１減らす
				self.text = (self.text.parse::<i32>().unwrap_or(self.min) - 1).max(self.min).to_string();
			}

			// バックスペース
			if is_key_pressed(KeyCode::Backspace) {
				self.text.pop();
			}

			// 入力から数字以外を削除
			self.text = self.text.chars()
				.filter(|c| c.is_ascii_digit())
				.collect();
		}

		let key = get_last_key_pressed().unwrap_or(KeyCode::Space);
		(key, self.shift_on)
	}

	//--------------------------------------------------
	// 入力値を返却
	//--------------------------------------------------
	pub fn get_value(&self) -> i32 {
		// テキストを数値に変換し、最小・最大値の範囲で返却する
		self.text.parse::<i32>().unwrap_or(self.min).max(self.min).min(self.max)
	}

	//--------------------------------------------------
	// 描画
	//--------------------------------------------------
	pub fn draw(&self) {
		// 枠
		dr_rect(self.x, self.y, self.w, self.h, 5.0,
			"000000FF", if self.focused { "FF0000FF" } else { "777777FF" });

		// テキスト
		let mut text = self.text.clone();
		if self.focused {
			text.push_str("|");
		}
		dr_text(&text, self.x + 10.0, self.y + 5.0, 28.0,
			"FFFFFFFF", if self.focused {"FF000077"} else {"00000000"});
	}
}
