use macroquad::prelude::*;

pub struct MyCamera {
    zoom: Vec2,                                         // 画面倍率
    offs: Vec2,                                         // 画面オフセット
}

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl MyCamera {

    //------------------------------
    // 初期化
    //------------------------------
    pub fn new () -> MyCamera {
        MyCamera {
            zoom: Vec2{x:1.0, y:1.0},
            offs: Vec2{x:0.0, y:0.0},
        }
    }

    //------------------------------
    // ウインドウに合わせてカメラを制御する
    //------------------------------
    pub fn update_camera(&mut self) {
        let zoom = Vec2 {
            x: self.zoom.x * 2.0 / screen_width(),
            y: self.zoom.y * 2.0 / screen_height(),
        };
        let offset = Vec2 {
            x: self.offs.x - 1.0,
            y: self.offs.x + 1.0,
        };
        let camera = Camera2D {
            zoom, offset,
            ..Default::default()
        };
        set_camera(&camera);
    }

    //------------------------------
    // ウインドウに合わせてカメラを制御する
    //------------------------------
    pub fn get_mouse_pos(&mut self) -> Vec2 {
        // マウス位置を画面倍率に合わせる
        let (pos_x, pos_y) = mouse_position();
        Vec2 {
            x: pos_x / self.zoom.x,
            y: pos_y / self.zoom.y,
        }
    }

}
