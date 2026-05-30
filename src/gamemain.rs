use crate::panel::Panel;
use crate::keymanager::KeyManager;
use macroquad::prelude::*;

// ゲームメインデータ
pub struct GameMain {
    width: i32,                             // 盤面の幅
    height: i32,                            // 盤面の高さ
    bom_num: i32,                           // 爆弾の数
    table: Vec<Panel>,                      // 盤面データ
    cursol_x: i32,                          // カーソル横位置
    cursol_y: i32,                          // カーソル縦位置
    key_manager: KeyManager,                // キー管理オブジェクト
    lastkey: char,                          // 最後に入力されたキー 
}

// 実装
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
            cursol_x: 0,
            cursol_y: 0,
            key_manager: KeyManager::new(),
            lastkey: ' ',
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
        let size = (self.width * self.height) as usize;
        for y in 0..self.height {
            for x in 0..self.width {
                self.table.push(Panel::new(x, y));
            }
        }

        // カーソル位置を初期化
        self.cursol_x = 0;
        self.cursol_y = 0;
        println!("\x1B[2J\x1b[1;1H"); // 画面をクリア
    }

    //------------------------------
    // 盤面の描画
    //------------------------------
    pub fn draw_table(&mut self) {
        for panel in &self.table {
            panel.draw_panel(self.cursol_x, self.cursol_y);
        }
    }

    pub fn key_control(&mut self) {
        // キー入力チェック
        self.lastkey =  self.key_manager.get_key();
    }
}