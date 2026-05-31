use macroquad::prelude::*;
use crate::{PANEL_WIDTH,PANEL_HEIGHT,WALL_LEFT,WALL_TOP};

//--- 色 ---
// パネル
use crate::{PANEL_COL_CLOSE,PANEL_COL_OPEN};

// 盤面のオブジェクト
pub struct Panel {
    dbgflg: i32,                            // デバッグ用
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
            dbgflg: 0,
            stat: 0,
            around_num: 0,
            isbom: false,
            user_flag: 0,
            pos_x,
            pos_y, 
        }        
    }

    //------------------------------
    // パネルのオープン処理
    //------------------------------
    pub fn open(&mut self) {
        self.stat = 1;
    }

    //------------------------------
    // ユーザのフラグを立てる
    //------------------------------
    pub fn set_userflag(&mut self, flg: i32) {
        self.user_flag = flg;
//        self.user_flag = (self.user_flag + 1) % 3;
    }

    pub fn setflg(&mut self, flg: i32) {
        self.dbgflg = flg;
    }

    //------------------------------
    // 状態を返却する
    //------------------------------
    pub fn getstat(&mut self) -> i32 {
        self.stat
    }

    //------------------------------
    // 周囲の爆弾数を返却する
    //------------------------------
    pub fn get_around_num(&mut self) -> i32 {
        self.around_num
    }

    //------------------------------
    // 爆弾を置く
    //------------------------------
    pub fn bomon(&mut self) {
        self.isbom = true;
    }

    //------------------------------
    // 爆弾を持っているか
    //------------------------------
    pub fn is_bom(&mut self) -> bool {
        self.isbom
    }

    //------------------------------
    // 周囲の爆弾カウントを増やす
    //------------------------------
    pub fn numup(&mut self) {
        self.around_num += 1;
    }

    //------------------------------
    // 自分自身を描画
    //------------------------------
    pub fn draw_panel(&self) {
        let left = self.pos_x as f32 * PANEL_WIDTH + WALL_LEFT;
        let top = self.pos_y as f32 * PANEL_HEIGHT + WALL_TOP;
        let mut panelcolor = PANEL_COL_CLOSE;
        let font_size= 35.0;

        let mut height = 2.0;
        if self.stat == 1 {
            height = 1.0;
            panelcolor = PANEL_COL_OPEN;
            }

        // 色を決める
        if self.dbgflg == 1 {
            if self.stat == 0 {
                panelcolor = Color::from_rgba(255, 255, 128, 255);
            } else {
                panelcolor = Color::from_rgba(240, 240, 100, 255);
            }
        }

        // 下地を描く
        draw_rectangle(left,top,
            PANEL_WIDTH, PANEL_HEIGHT,
            BLACK);
        draw_rectangle(left,top,
            PANEL_WIDTH - height, PANEL_HEIGHT - height,
            WHITE);
        // パネルを描く
        draw_rectangle(left + height,top + height,
            PANEL_WIDTH - height * 2.0, PANEL_HEIGHT - height * 2.0,
            panelcolor);

        // フラグが立っているなら表示
        if self.stat == 0 && self.user_flag != 0 {
            let mut flag_col = RED;
            if self.user_flag == 2 {
                flag_col = BLUE;
            }
            draw_text("P", left + 5.0, top + 20.0 + 5.0, font_size, flag_col);
        }

        // パネルが閉じている場合はここまで
        if self.stat == 0 {
            return;
        }

        // パネルが開いているなら
        // 爆弾マスは爆弾を描く
        if self.isbom {
            draw_circle(left + PANEL_WIDTH as f32 / 2.0,top + PANEL_WIDTH as f32 / 2.0,
            PANEL_WIDTH as f32 / 2.0 - height,BLACK);
            return;
        }

        // 周囲の爆弾数の表示
        if self.around_num > 0 {
            let text = format!("{}", self.around_num);
            draw_text(text, left + 5.0, top + 20.0 + 5.0, font_size, RED);
        }
    }
}
