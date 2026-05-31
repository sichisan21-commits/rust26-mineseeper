use crate::panel::Panel;
use macroquad::prelude::*;
use crate::{PANEL_WIDTH,PANEL_HEIGHT,WALL_LEFT,WALL_TOP};
use crate::LAYOUT_COLOR;

// ゲームメインデータ
pub struct GameMain {
    width: i32,                             // 盤面の幅
    height: i32,                            // 盤面の高さ
    bom_num: i32,                           // 爆弾の数
    table: Vec<Panel>,                      // 盤面データ
    cursol_x: i32,                          // カーソル横位置
    cursol_y: i32,                          // カーソル縦位置
    on_table: bool,                         // カーソルが盤面上にあるか
    around: [[i32; 3];3],                   // カーソル周囲９マスのテーブル位置
}

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl GameMain {
    //------------------------------
    // 初期化
    //------------------------------
    pub fn new () -> GameMain {
        GameMain {
            width: 16,
            height: 10,
            bom_num: 0,
            table: Vec::new(),
            on_table: false,
            cursol_x: 0,
            cursol_y: 0,
            around: [[-1;3];3],
        }
    }

    //------------------------------
    // ゲームの情報の設定
    //------------------------------
    pub fn set_gameinfo(&mut self, width: i32, height: i32, bom_num: i32) {
        self.width = width;
        self.height = height;
        self.bom_num = bom_num;
    }

    //------------------------------
    // 盤面の初期化
    //------------------------------
    pub fn initial_game(&mut self) {
        // 盤面を初期化する
        self.table.clear();
        let size = (self.width * self.height) as usize;
        for y in 0..self.height {
            for x in 0..self.width {
                self.table.push(Panel::new(x, y));
            }
        }

        // カーソル位置を初期化
        self.cursol_x = 0;
        self.cursol_y = 0;
    }

    //------------------------------
    // 盤面の描画
    //------------------------------
    pub fn draw_table(&mut self) {
        // 盤面全体を塗りつぶす
        draw_rectangle(0.0,0.0,
            PANEL_WIDTH * self.width as f32 + WALL_LEFT * 2.0,
            PANEL_HEIGHT * self.height as f32 + WALL_TOP * 2.0,
            LAYOUT_COLOR);

        for panel in &self.table {
            panel.draw_panel(self.cursol_x, self.cursol_y);
        }
    }

    //------------------------------
    // カーソル位置の処理
    //------------------------------
    pub fn get_cursolpos(&mut self) {
        // マウス位置の取得
        let (mouse_x, mouse_y) = mouse_position();

        // マウス位置を盤面位置に変換
        let cursol_x = ((mouse_x - WALL_LEFT) / PANEL_WIDTH) as i32;
        let cursol_y = ((mouse_y - WALL_TOP) / PANEL_HEIGHT) as i32;

        // カーソル位置が盤面からはみ出さないよう制御
        self.cursol_x = cursol_x.clamp(0, self.width - 1);
        self.cursol_y = cursol_y.clamp(0, self.height - 1);
        self.on_table = self.cursol_x == cursol_x && self.cursol_y == cursol_y;

        // マウスの周囲９マスのインデックスを求める
        let tblpos = self.cursol_y * self.width + self.cursol_x;
        for y in 0..3 {
            for x in 0..3 {
                // 一旦フラグを落とす
                let index = self.around[y][x];
                if self.around[y][x] != -1 {
                    self.table[index as usize].close();
                }

                // 盤面の横位置、縦位置、テーブルインデックスを求める
                let pos_x = self.cursol_x + x as i32 -1;
                let pos_y = self.cursol_y + y as i32 -1;

                // インデックスを簡易テーブルに保存する
                if pos_x < 0 || pos_x >= self.width ||
                   pos_y < 0 || pos_y >= self.height {
                    self.around[y][x] = -1;
                } else {
                    self.around[y][x] = tblpos + (x as i32 - 1) + self.width * (y as i32 - 1);
                    // デバッグ用にフラグを立てる
                    self.table[self.around[y][x] as usize].open();
                }
            }
        }
    }
}
