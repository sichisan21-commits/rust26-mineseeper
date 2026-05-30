use macroquad::prelude::*;

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

    pub fn draw_panel(&self, cursol_x: i32, cursol_y: i32) {
        let left = self.pos_x as f32 * 20.0;
        let top = self.pos_y as f32 * 20.0;
        draw_rectangle(left,top,10.0,10.0, LIGHTGRAY);
    }
    //------------------------------
    // 描画情報の取得
    //------------------------------
    pub fn get_view_txt(&self, cursol_x: i32, cursol_y: i32) -> String {
        // パネルが未開封の場合
        if self.stat == 0 {
            // かつカーソル位置と一致している場合
            if self.pos_x == cursol_x && self.pos_y == cursol_y {
                return String::from("\x1b[7m\x1b[31m　\x1b[0m")
            } else {
                return String::from("\x1b[7m　\x1b[0m")
            }
        }
 
        // パネルが開封済の場合
        // かつカーソル位置と一致している場合
        if self.pos_x == cursol_x && self.pos_y == cursol_y {
            return String::from("\x1b[31m■\x1b[0m")
        } else {
            return String::from("　")
        }
    }
}
