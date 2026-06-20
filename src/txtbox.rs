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
}

impl TextBox {
	pub fn new(text: &str, x: f32, y: f32, w: f32, h: f32, min:i32, max:i32) -> Self {
		Self {
			text: text.to_string(),
			x, y, w, h, min, max,
			focused: false,
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
	// 対象にフォーカスする
	// タブでフォーカスアウトされた場合 true を返す
	//--------------------------------------------------
	pub fn update(&mut self) -> bool {
		let mut is_tab = false;

		// クリックでフォーカス
		if is_mouse_button_pressed(MouseButton::Left) {
			let (mx, my) = mouse_position();
			self.focused = mx >= self.x && mx <= self.x + self.w &&
						   my >= self.y && my <= self.y + self.h;
		}

		// フォーカス中だけ文字入力
		if self.focused {
			if let Some(c) = get_char_pressed() {
				if !c.is_control() {
					self.text.push(c);
				}
			}

			// タブ
			if is_key_pressed(KeyCode::Tab) || is_key_pressed(KeyCode::Enter){
				self.focused = false;
				is_tab = true;
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

		// 最小値チェック
		if !self.focused &&
		   (self.text == "" || self.text.parse::<i32>().unwrap_or(0) < self.min) {
			self.text = self.min.to_string();
		}

		// 最大値チェック
		if !self.focused &&
		   self.text.parse::<i32>().unwrap_or(0) > self.max {
			self.text = self.max.to_string();
		}

		is_tab
	}

	//--------------------------------------------------
	// 入力値を返却
	//--------------------------------------------------
	pub fn get_value(&self) -> i32 {
		self.text.parse::<i32>().unwrap_or(self.min)
	}

	pub fn draw(&self) {
		// 枠
		dr_rect(self.x, self.y, self.w, self.h, 5.0,
			"000000FF", if self.focused { "FF0000FF" } else { "777777FF" });

		// テキスト
		let mut text = self.text.clone();
		if self.focused {
			text.push_str("|");
		}
		draw_text(&text, self.x + 5.0, self.y + self.h - 8.0, 24.0, WHITE);
	}
}
