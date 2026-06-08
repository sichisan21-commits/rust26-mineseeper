use macroquad::prelude::*;

pub struct TextBox {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub focused: bool,
}

impl TextBox {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            text: String::new(),
            x, y, w, h,
            focused: false,
        }
    }

    pub fn update(&mut self) {
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

            // バックスペース
            if is_key_pressed(KeyCode::Backspace) {
                self.text.pop();
            }
        }
    }

    pub fn draw(&self) {
        // 枠
        draw_rectangle_lines(self.x, self.y, self.w, self.h, 2.0,
            if self.focused { YELLOW } else { WHITE });

        // テキスト
        draw_text(&self.text, self.x + 5.0, self.y + self.h - 8.0, 24.0, WHITE);
    }
}
