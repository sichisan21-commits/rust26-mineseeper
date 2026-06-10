use macroquad::prelude::*;
use crate::utils::*;
use crate::myconst::*;

//----------------------------------------
// 盤面のオブジェクト
//----------------------------------------
#[derive(Clone, PartialEq, Eq)]
pub struct Panel {
    pos_x: i32,                             // 自分の横座標
    pos_y: i32,                             // 自分の縦座標
    index: i32,                             // 自分の配列番号
    is_open: bool,                          // 開いているか
    is_bom: bool,                           // 爆弾有無
    is_bold: bool,                          // 太字表示
    auto_flg: AutoSts,                      // 自動判定フラグ
    userflg: UserFlg,                       // 旗
    around_tbl: [[i32; 3];3],               // カーソル周囲９マスのテーブル位置
    around_num: i32,                        // 周りの爆弾数
}

// 実装
impl Panel {
    //------------------------------
    // パネルの初期化
    //------------------------------
    pub fn new(pos_x: i32, pos_y: i32, width: i32, height: i32) -> Panel {
        let index = get_index(pos_x, pos_y, width, height);
        let mut panel = Panel {
            pos_x,
            pos_y,
            index,
            is_open: false,
            is_bom: false,
            is_bold: false,
            userflg: UserFlg::None,
            around_num: 0,
            auto_flg: AutoSts::None,
            around_tbl: [[-1;3];3],
        };

        // 周囲のインデックスを求める
        for y in 0..3 {
            for x in 0..3 {
                panel.around_tbl[y][x] = get_index(
                    pos_x + x as i32 - 1, pos_y + y as i32 - 1,
                    width, height);
            }
        }
        panel
    }

    //------------------------------
    // パネルを開く
    //------------------------------
    pub fn open(&mut self) {
        self.is_open = true;
    }

    //------------------------------
    // パネルが開いているか
    //------------------------------
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    //------------------------------
    // 爆弾が踏まれたか
    //------------------------------
    pub fn is_bomopen(&self) -> bool {
        self.is_open && self.is_bom
    }

    //------------------------------
    // 旗のオン／オフ
    //------------------------------
    pub fn userflg(&mut self) {
        // Close → RedFlg → BlueFlg の順に巡回する
        self.userflg = match self.userflg {
            UserFlg::None    => UserFlg::RedFlg,
            UserFlg::RedFlg  => UserFlg::BlueFlg,
            UserFlg::BlueFlg => UserFlg::None,
        };
    }

    //------------------------------
    // 旗のオン／オフ
    //------------------------------
    pub fn set_userflg(&mut self, flg:UserFlg) {
        self.userflg = flg;
    }

    //------------------------------
    // 旗の有無
    //------------------------------
    pub fn get_userflg(&self) -> UserFlg {
        self.userflg
    }

    //------------------------------
    // 周囲のインデックステーブルを返却（コピー）
    //------------------------------
    pub fn get_around_tbl(&self) -> [[i32; 3]; 3] {
        self.around_tbl
    }

    //------------------------------
    // ゲームの自動判定フラグセット
    //------------------------------
    pub fn set_autoflag(&mut self, flg: AutoSts) {
        self.auto_flg = flg;
    }

    //------------------------------
    // ゲームの自動判定フラグ取得
    //------------------------------
    pub fn get_autoflag(&self) -> AutoSts{
        self.auto_flg
    }

    //------------------------------
    // 強調表示オン
    //------------------------------
    pub fn bold_on(&mut self) {
        self.is_bold = true;
    }

    //------------------------------
    // 強調表示オフ
    //------------------------------
    pub fn bold_off(&mut self) {
        self.is_bold = false;
    }

    //------------------------------
    // 周囲の爆弾数を返却する
    //------------------------------
    pub fn get_around_num(&self) -> i32 {
        self.around_num
    }

    //------------------------------
    // 爆弾を置く
    //------------------------------
    pub fn bomon(&mut self) {
        self.is_bom = true;
        self.around_num = 0;
    }

    //------------------------------
    // 爆弾を持っているか
    //------------------------------
    pub fn is_bom(&self) -> bool {
        self.is_bom
    }

    //------------------------------
    // 周囲の爆弾カウントを増やす
    //------------------------------
    pub fn num_up(&mut self) {
        // 自分自身が爆弾マスでない場合にカウントを増やす
        if !self.is_bom {
            self.around_num += 1;
        }
    }

    //------------------------------
    // 自分自身を描画
    //------------------------------
    pub fn draw_panel(&self, cursol_x: i32, cursol_y: i32, is_alldraw: bool) {
        let mut is_cursol_around = true;

        // カーソル周囲９マスか判定
        if !is_alldraw {
            is_cursol_around =  (cursol_x - self.pos_x).abs() <= 1 &&
                (cursol_y - self.pos_y).abs() <= 1;
        }

        // 描画位置を算出
        let left = self.pos_x as f32 * PANEL_WIDTH;
        let top = self.pos_y as f32 * PANEL_HEIGHT;

        // 下地を描く
        draw_rectangle(left,top,
            PANEL_WIDTH, PANEL_HEIGHT,
            PANEL_COL_SHADOW);
        draw_rectangle(left,top,
            PANEL_WIDTH - PANEL_THICK, PANEL_HEIGHT - PANEL_THICK,
            WHITE);

        // パネルを描く
        self.draw_panel_close(left, top, is_cursol_around);
        self.draw_panel_open(left, top, is_cursol_around);
    }

    //------------------------------
    // 閉じているパネルの描画
    //------------------------------
    fn draw_panel_close(&self, left:f32, top: f32, is_cursol_around: bool){
        // 開いているなら何もしない
        if self.is_open {
            return
        }

        // デフォルトの描画色を設定
        let mut panelcolor = PANEL_COL_CLOSE;

        // カーソルの周囲で旗が立てられていなければヘルプ表示色の設定
        if is_cursol_around && self.userflg == UserFlg::None {
            if self.auto_flg == AutoSts::Danger {
                panelcolor = PANEL_COL_DANGER;
            } else if self.auto_flg == AutoSts::Safety {
                panelcolor = PANEL_COL_SAFETY;
            }
        }

        // パネルの表面を描く
        draw_rectangle(left + PANEL_THICK,top + PANEL_THICK,
            PANEL_WIDTH - PANEL_THICK * 2.0, PANEL_HEIGHT - PANEL_THICK * 2.0,
           panelcolor);

        // パネルの文字を描く
        self.draw_text_close(left, top, is_cursol_around);
    }

    //------------------------------
    // 閉じているパネルの描画
    //------------------------------
    fn draw_panel_open(&self, left:f32, top: f32, is_cursol_around: bool){
        // 閉じているなら何もしない
        if !self.is_open {
            return
        }

        // 表面の色を設定
        let mut panel_col = PANEL_COL_OPEN;
        if self.is_bom {
            panel_col = RED;
        }

        // パネルの表面を描く
        let panel_thick = 1.0;
        draw_rectangle(left + panel_thick,top + panel_thick,
            PANEL_WIDTH - panel_thick * 2.0, PANEL_HEIGHT - panel_thick * 2.0,
           panel_col);

        // 爆弾マスの場合爆弾を表示
        if self.is_bom {
            draw_circle(left + PANEL_WIDTH as f32 / 2.0,top + PANEL_WIDTH as f32 / 2.0,
            PANEL_WIDTH as f32 / 2.0 - 5.0,BLACK);
            return;
        }
        
        // テキストを描画
        self.draw_text_open(left, top, is_cursol_around);
    }

    //------------------------------
    // パネルの文字を描画（開いている）
    //------------------------------
    fn draw_text_open(&self, left: f32, top:f32, is_cursol_around: bool) {
        // 開いていなければなにもしない
        if !self.is_open {
            return
        }

        // 爆弾数の表示
        if self.around_num > 0 {
            let text = format!("{}", self.around_num);
            let text_col = from_number(self.around_num);

            // 強調表示（一旦力業）
            if self.is_bold && is_cursol_around {
                draw_text(&text, left + PANEL_FONT_OFFSX - 4.0, top + PANEL_FONT_OFFSY - 4.0, PANEL_FONT_SIZE, WHITE);
                draw_text(&text, left + PANEL_FONT_OFFSX - 2.0, top + PANEL_FONT_OFFSY - 4.0, PANEL_FONT_SIZE, WHITE);
                draw_text(&text, left + PANEL_FONT_OFFSX,       top + PANEL_FONT_OFFSY - 4.0, PANEL_FONT_SIZE, WHITE);
                draw_text(&text, left + PANEL_FONT_OFFSX,       top + PANEL_FONT_OFFSY + 4.0, PANEL_FONT_SIZE, BLACK);
                draw_text(&text, left + PANEL_FONT_OFFSX + 2.0, top + PANEL_FONT_OFFSY + 4.0, PANEL_FONT_SIZE, BLACK);
                draw_text(&text, left + PANEL_FONT_OFFSX + 4.0, top + PANEL_FONT_OFFSY + 4.0, PANEL_FONT_SIZE, BLACK);
            }
            draw_text(&text, left + PANEL_FONT_OFFSX,       top + PANEL_FONT_OFFSY, PANEL_FONT_SIZE, text_col);
            draw_text(&text, left + PANEL_FONT_OFFSX + 3.0, top + PANEL_FONT_OFFSY, PANEL_FONT_SIZE, text_col);
        }
    }

     //------------------------------
    // パネルの文字を描画（閉じている）
    //------------------------------
    fn draw_text_close(&self, left: f32, top:f32, is_cursol_around: bool) {
        // 開いていればなにもしない
        if self.is_open {
            return
        }

        // 旗が立っているなら描画
        if self.userflg != UserFlg::None {
            let flag_col =
                if self.userflg == UserFlg::RedFlg {
                    RED
                } else {
                    BLUE
                };
            draw_text("F", left + PANEL_FONT_OFFSX + 3.0, top + PANEL_FONT_OFFSY, PANEL_FONT_SIZE, BLACK);
            draw_text("F", left + PANEL_FONT_OFFSX,       top + PANEL_FONT_OFFSY, PANEL_FONT_SIZE, flag_col);
            draw_text("F", left + PANEL_FONT_OFFSX - 3.0, top + PANEL_FONT_OFFSY, PANEL_FONT_SIZE, flag_col);
        } else {
            // 判明していないパネルなら？を表示
            if self.auto_flg == AutoSts::Unknown && is_cursol_around {
                draw_text("?", left + PANEL_FONT_OFFSX + 3.0, top + PANEL_FONT_OFFSY, PANEL_FONT_SIZE, GRAY);
                draw_text("?", left + PANEL_FONT_OFFSX, top + PANEL_FONT_OFFSY, PANEL_FONT_SIZE, GRAY);
            }
        }
    }
}

//---------------------------------------------
// 爆弾数に応じた色を表示する
//---------------------------------------------
pub fn from_number(n: i32) -> Color {
    match n {
        1 => BLUE,
        2 => Color::from_rgba(0,150,0,255),
        3 => RED,
        4 => DARKBLUE,
        5 => BROWN,
        6 => Color::from_rgba(0,128,128,255),
        7 => BLACK,
        8 => GRAY,
        _ => BLACK, // 0 や範囲外は黒など
    }
}