use macroquad::prelude::*;
use crate::panel::Panel;

pub fn draw_panel(&panel, cursol_x: i32, cursol_y: i32, is_alldraw: bool) {
    let mut is_cursol_around = true;

    // カーソル周囲９マスか判定
    if !is_alldraw {
        is_cursol_around =  (cursol_x - self.pos_x).abs() <= 1 &&
               (cursol_y - self.pos_y).abs() <= 1;
    }

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
    self.draw_text_close(left, top, is_cursol_around);
}

//------------------------------
// 閉じているパネルの描画
//------------------------------
fn draw_panel_open(&self, left:f32, top: f32, is_cursol_around: bool){
    // 閉じているなら何もしない
    if self.panel_sts == PanelSts::Close ||
       self.panel_sts == PanelSts::BlueFlg ||
       self.panel_sts == PanelSts::RedFlg {
        return
    }

    // 踏まれた爆弾か
    let is_bom = self.isbom && self.panel_sts == PanelSts::BomOpen;

    // 表面の色を設定
    let mut panel_col = PANEL_COL_OPEN;
    if is_bom {
        panel_col = RED;
    }

    // パネルの表面を描く
    let panel_thick = 1.0;
    draw_rectangle(left + panel_thick,top + panel_thick,
        PANEL_WIDTH - panel_thick * 2.0, PANEL_HEIGHT - panel_thick * 2.0,
        panel_col);

    // 爆弾マスの場合爆弾を表示
    if is_bom {
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
        let text_col = from_number(self.around_num);

        // 強調表示（一旦力業）
        if self.isbold && is_cursol_around {
            draw_text(&text, left + 3.0, top + 18.0, PANEL_FONT_SIZE, WHITE);
            draw_text(&text, left + 4.0, top + 18.0, PANEL_FONT_SIZE, WHITE);
            draw_text(&text, left + 5.0, top + 18.0, PANEL_FONT_SIZE, WHITE);
            draw_text(&text, left + 5.0, top + 22.0, PANEL_FONT_SIZE, BLACK);
            draw_text(&text, left + 6.0, top + 22.0, PANEL_FONT_SIZE, BLACK);
            draw_text(&text, left + 7.0, top + 22.0, PANEL_FONT_SIZE, BLACK);
        }
        draw_text(&text, left + 5.0, top + 20.0, PANEL_FONT_SIZE, text_col);
        draw_text(&text, left + 6.0, top + 20.0, PANEL_FONT_SIZE, text_col);
    }
}

//------------------------------
// パネルの文字を描画（閉じている）
//------------------------------
fn draw_text_close(&self, left: f32, top:f32, is_cursol_around: bool) {
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
        draw_text("F", left + 7.0, top + 20.0, PANEL_FONT_SIZE, BLACK);
        draw_text("F", left + 5.0, top + 19.0, PANEL_FONT_SIZE, flag_col);
    } else {
        // 判明していないパネルなら？を表示
        if self.auto_flg == AutoSts::Unknown && is_cursol_around {
            draw_text("?", left + 6.0, top + 20.0, PANEL_FONT_SIZE, GRAY);
            draw_text("?", left + 5.0, top + 20.0, PANEL_FONT_SIZE, GRAY);
        }
    }
}
