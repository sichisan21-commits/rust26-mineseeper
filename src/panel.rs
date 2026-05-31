use macroquad::prelude::*;
use crate::{PANEL_WIDTH,PANEL_HEIGHT,WALL_LEFT,WALL_TOP};

//--- 色 ---
// パネル
use crate::PANEL_COL_CLOSE;

// 盤面のオブジェクト
pub struct Panel {
    stat: i32,                              // 0=閉／1=開
    around_num: i32,                        // 周りの爆弾数
    isbom: bool,                            // 爆弾有無
    user_flag: i32,                         // 0=なし/1=確定フラグ/2=暫定フラグ
    pos_x: i32,                             // 自分の横座標
    pos_y: i32,                             // 自分の縦座標
}

// 実装
impl Panel {
    //------------------------------
    // パネルの初期化
    //------------------------------
    pub fn new(pos_x: i32, pos_y: i32) -> Panel {
        Panel {
            stat: 0,
            around_num: 0,
            isbom: false,
            user_flag: 0,
            pos_x,
            pos_y, 
        }        
    }

    pub fn open(&mut self) {
        self.stat = 1;
    }
    pub fn close(&mut self) {
        self.stat = 0;
    }

    //------------------------------
    // 自分自身を描画
    //------------------------------
    pub fn draw_panel(&self, cursol_x: i32, cursol_y: i32) {
        let left = self.pos_x as f32 * PANEL_WIDTH + WALL_LEFT;
        let top = self.pos_y as f32 * PANEL_HEIGHT + WALL_TOP;
        draw_rectangle(left,top,
            PANEL_WIDTH, PANEL_HEIGHT,
            BLACK);

        let mut panelcolor = PANEL_COL_CLOSE;
        if self.stat == 1 {
            panelcolor = Color::from_rgba(255, 255, 128, 255);
        }
        if self.pos_x == cursol_x && self.pos_y == cursol_y {
            panelcolor = Color::from_rgba(255, 128, 128, 255);
        }
        draw_rectangle(left,top,
            PANEL_WIDTH - 2.0, PANEL_HEIGHT - 2.0,
            panelcolor);
        let text = format!("{}", self.stat);
        draw_text(text, left + 20.0, top + 20.0, 20.0, RED);
        }
}
