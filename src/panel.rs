use macroquad::prelude::*;
use crate::utils::*;
use crate::myconst::*;

//----------------------------------------
// enum
//----------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelSts {                         // パネルの状態
    Close,                                  // 閉じている
    Open,                                   // 開いている
    RedFlg,                                 // 旗（赤）
    BlueFlg,                                // 旗（青）
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoSts {                          // 自動判定フラグ
    None,                                   // なにもなし
    Safety,                                  // 安全マス
    Danger,                                 // 危険マス
}

//----------------------------------------
// 盤面のオブジェクト
//----------------------------------------
#[derive(Clone, PartialEq, Eq)]
pub struct Panel {
    panel_sts: PanelSts,                    // パネルの状態
    pos_x: i32,                             // 自分の横座標
    pos_y: i32,                             // 自分の縦座標
    index: i32,                             // 自分の配列番号
    around_tbl: [[i32; 3];3],               // カーソル周囲９マスのテーブル位置
    around_num: i32,                        // 周りの爆弾数
    isbom: bool,                            // 爆弾有無
    isbold: bool,                           // 太字表示
    auto_flg: AutoSts,                      // 自動判定フラグ
}

// 実装
impl Panel {
    //------------------------------
    // パネルの初期化
    //------------------------------
    pub fn new(pos_x: i32, pos_y: i32, width: i32, height: i32) -> Panel {
        let index = get_index(pos_x, pos_y, width, height);
        let mut panel = Panel {
            panel_sts: PanelSts::Close,
            pos_x,
            pos_y,
            index,
            isbom: false,
            isbold: false,
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
        self.panel_sts = PanelSts::Open;
        self.auto_flg = AutoSts::None;
    }

    //------------------------------
    // パネルが開いているか
    //------------------------------
    pub fn is_open(&self) -> bool {
        self.panel_sts == PanelSts::Open
    }

    //------------------------------
    // 旗のオン／オフ
    //------------------------------
    pub fn set_userflag(&mut self) {
        // Close → RedFlg → BlueFlg の順に巡回する
        // Open の場合なにもしない
        self.panel_sts = match self.panel_sts {
            PanelSts::Close    => PanelSts::RedFlg,
            PanelSts::RedFlg   => PanelSts::BlueFlg,
            PanelSts::BlueFlg  => PanelSts::Close,
            PanelSts::Open     => PanelSts::Open,
        };
    }

    //------------------------------
    // 旗の有無
    //------------------------------
    pub fn is_userflag(&self) -> bool {
        self.panel_sts == PanelSts::RedFlg || self.panel_sts == PanelSts::BlueFlg
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
        self.isbold = true;
    }

    //------------------------------
    // 強調表示オフ
    //------------------------------
    pub fn bold_off(&mut self) {
        self.isbold = false;
    }

    //------------------------------
    // 周囲の爆弾が確定しているか
    //------------------------------
    pub fn is_bold(&self) -> bool {
        self.isbold == true
    }

    //------------------------------
    // 状態を返却する
    //------------------------------
//    pub fn getstat(&self) -> i32 {
//        self.stat
//    }

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
        self.isbom = true;
        self.around_num = 0;
    }

    //------------------------------
    // 爆弾を持っているか
    //------------------------------
    pub fn is_bom(&self) -> bool {
        self.isbom
    }

    //------------------------------
    // 周囲の爆弾カウントを増やす
    //------------------------------
    pub fn num_up(&mut self) {
        // 自分自身が爆弾マスでない場合にカウントを増やす
        if !self.isbom {
            self.around_num += 1;
        }
    }

    //------------------------------
    // 自分自身を描画
    //------------------------------
    pub fn draw_panel(&self, cursol_x: i32, cursol_y: i32) {
        let is_cursol_around =  (cursol_x - self.pos_x).abs() <= 1 &&
           (cursol_y - self.pos_y).abs() <= 1;

        // 描画位置を算出
        let left = self.pos_x as f32 * PANEL_WIDTH + WALL_LEFT;
        let top = self.pos_y as f32 * PANEL_HEIGHT + WALL_TOP;

        // 下地を描く
        draw_rectangle(left,top,
            PANEL_WIDTH, PANEL_HEIGHT,
            BLACK);
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
        if self.panel_sts == PanelSts::Open {
            return
        }

        // デフォルトの描画色を設定
        let mut panelcolor = PANEL_COL_CLOSE;

        // カーソルの周囲であればヘルプ表示色の設定
        if is_cursol_around {
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
        self.draw_text_close(left, top);
    }

    //------------------------------
    // 閉じているパネルの描画
    //------------------------------
    fn draw_panel_open(&self, left:f32, top: f32, is_cursol_around: bool){
        // 閉じているなら何もしない
        if self.panel_sts != PanelSts::Open {
            return
        }

        // 表面の色を設定
        let mut panel_col = PANEL_COL_OPEN;
        if self.isbom {
            panel_col = RED;
        }

        // パネルの表面を描く
        let panel_thick = 1.0;
        draw_rectangle(left + panel_thick,top + panel_thick,
            PANEL_WIDTH - panel_thick * 2.0, PANEL_HEIGHT - panel_thick * 2.0,
           panel_col);

        // 爆弾マスの場合爆弾を表示
        if self.isbom {
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
        if self.panel_sts != PanelSts::Open {
            return
        }

        // 爆弾数の表示
        if self.around_num > 0 {
            let text = format!("{}", self.around_num);

            // 強調表示（一旦力業）
            if self.isbold && is_cursol_around {
                draw_text(&text, left + 3.0, top + 18.0, PALNE_FONT_SIZE, WHITE);
                draw_text(&text, left + 4.0, top + 18.0, PALNE_FONT_SIZE, WHITE);
                draw_text(&text, left + 5.0, top + 18.0, PALNE_FONT_SIZE, WHITE);
                draw_text(&text, left + 5.0, top + 21.0, PALNE_FONT_SIZE, BLACK);
                draw_text(&text, left + 6.0, top + 21.0, PALNE_FONT_SIZE, BLACK);
                draw_text(&text, left + 7.0, top + 21.0, PALNE_FONT_SIZE, BLACK);
            }
            draw_text(&text, left + 5.0, top + 20.0, PALNE_FONT_SIZE, RED);
        }
    }

     //------------------------------
    // パネルの文字を描画（閉じている）
    //------------------------------
    fn draw_text_close(&self, left: f32, top:f32) {
        // 開いていればなにもしない
        if self.panel_sts == PanelSts::Open {
            return
        }

        // 旗が立っているなら描画
        if self.panel_sts == PanelSts::RedFlg || self.panel_sts == PanelSts::BlueFlg {
            let mut flag_col = RED;
            if self.panel_sts == PanelSts::BlueFlg {
                flag_col = BLUE;
            }
            draw_text("F", left + 7.0, top + 20.0, PALNE_FONT_SIZE, BLACK);
            draw_text("F", left + 5.0, top + 19.0, PALNE_FONT_SIZE, flag_col);
        }
    }
}