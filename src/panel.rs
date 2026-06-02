use macroquad::prelude::*;
use crate::{PANEL_WIDTH,PANEL_HEIGHT,WALL_LEFT,WALL_TOP};

// パネル情報
use crate::{PANEL_COL_CLOSE,PANEL_COL_OPEN,PANEL_COL_DANGER,PANEL_COL_SAFETY,PALNE_FONT_SIZE};

#[derive(Clone)]
// 盤面のオブジェクト
pub struct Panel {
    dbgflg: i32,                            // デバッグ用
    stat: i32,                              // 0=閉／1=開
    around_num: i32,                        // 周りの爆弾数
    isbom: bool,                            // 爆弾有無
    user_flg: i32,                          // 0=なし/1=確定フラグ/2=暫定フラグ
    auto_flg: i32,                          // 0=なし/1=危険フラグ/2=安全フラグ
    pos_x: i32,                             // 自分の横座標
    pos_y: i32,                             // 自分の縦座標
    around: [[i32; 3];3],                   // カーソル周囲９マスのテーブル位置
}

// 実装
impl Panel {
    //------------------------------
    // パネルの初期化
    //------------------------------
    pub fn new(pos_x: i32, pos_y: i32, width: i32, height: i32) -> Panel {
        let mut panel = Panel {
            dbgflg: 0,
            stat: 0,
            around_num: 0,
            isbom: false,
            user_flg: 0,
            auto_flg: 0,
            pos_x,
            pos_y,
            around: [[-1;3];3],
        };
        // 周囲のインデックスを求める
        for y in 0..3 {
            for x in 0..3 {
                panel.around[y][x] = get_index(
                    pos_x + x as i32 - 1, pos_y + y as i32 - 1,
                    width, height);
            }
        }
        panel
    }

    //------------------------------
    // 周囲のインデックステーブルを返却（コピー）
    //------------------------------
    pub fn get_around(&self) -> [[i32; 3]; 3] {
        self.around
    }

    //------------------------------
    // パネルのオープン処理
    //------------------------------
    pub fn open(&mut self) {
        self.stat = 1;
        self.auto_flg = 0;
    }

    //------------------------------
    // ゲームの自動判定フラグ
    //------------------------------
    pub fn set_autoflag(&mut self, flg: i32) {
        self.auto_flg = flg;
    }

    //------------------------------
    // ゲームの自動判定フラグ
    //------------------------------
    pub fn get_autoflag(&mut self) -> i32{
        self.auto_flg
    }

    //------------------------------
    // ユーザのフラグを立てる
    //------------------------------
    pub fn set_userflag(&mut self) {
        self.user_flg = (self.user_flg + 1) % 3;
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
        if !self.isbom {
            self.around_num += 1;
        }
    }

    //------------------------------
    // 自分自身を描画
    //------------------------------
    pub fn draw_panel(&self) {
        let left = self.pos_x as f32 * PANEL_WIDTH + WALL_LEFT;
        let top = self.pos_y as f32 * PANEL_HEIGHT + WALL_TOP;
        let mut panelcolor = PANEL_COL_CLOSE;
        let font_size= PALNE_FONT_SIZE as f32;

        let mut height = 2.0;
        if self.stat == 1 {
            height = 1.0;
            panelcolor = PANEL_COL_OPEN;
            }

        // 色を決める
        if self.auto_flg == 1 {
            panelcolor = PANEL_COL_DANGER;
        } else if self.auto_flg == 2 {
            panelcolor = PANEL_COL_SAFETY;
        } else if self.dbgflg == 1 {
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
        if self.stat == 0 && self.user_flg != 0 {
            let mut flag_col = RED;
            if self.user_flg == 2 {
                flag_col = BLUE;
            }
            draw_text("P", left + 5.0, top + 20.0, font_size, flag_col);
        }

        // パネルが閉じている場合はここまで
        if self.stat == 0 {
//            let text = format!("{}", self.around_num);
//            draw_text(text, left + 5.0, top + 20.0, font_size, RED);
            return;
        }

        // パネルが開いているなら
        // 爆弾マスは爆弾を描く
        if self.isbom {
            draw_circle(left + PANEL_WIDTH as f32 / 2.0,top + PANEL_WIDTH as f32 / 2.0,
            PANEL_WIDTH as f32 / 2.0 - 3.0,BLACK);
            return;
        }

        // 周囲の爆弾数の表示
        if self.around_num > 0 {
            let text = format!("{}", self.around_num);
            draw_text(text, left + 5.0, top + 20.0, font_size, RED);
        }
    }
}

//------------------------------
// 座標をインデックスへ変換
//------------------------------
fn get_index(cursol_x:i32, cursol_y:i32, width:i32, height:i32) -> i32 {
    if cursol_x < 0 || cursol_x >= width ||
       cursol_y < 0 || cursol_y >= height {
        return -1;
       }
    cursol_y * width + cursol_x
}