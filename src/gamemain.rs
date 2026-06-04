use macroquad::prelude::*;
use crate::utils::*;
use crate::myconst::*;
use crate::gametable::GameTable;

// ゲームメインデータ
pub struct GameMain {
    stat: i32,                              // 0:初期画面/1:開始待ち/2:プレイ中
    width: i32,                             // 盤面の幅
    height: i32,                            // 盤面の高さ
    bom_num: i32,                           // 爆弾の数
    cursol_x: i32,                          // カーソル横位置
    cursol_y: i32,                          // カーソル縦位置
    cursol_index: i32,                      // カーソル位置のパネル番号
    table: GameTable,                       // 盤面情報
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
            stat: 0,
            width: 10,
            height: 10,
            bom_num: 0,
            cursol_x: 0,
            cursol_y: 0,
            cursol_index: -1,
            table: GameTable::new(),
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
        // カーソル位置を初期化
        self.cursol_x = -1;
        self.cursol_y = -1;

        // 盤面を初期化する
        self.table.initial(self.width, self.height);

        // クリック待ち
        self.stat = 1;
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

        // パネルを描画する
        self.table.draw_panel();

        // カーソル周りに枠を表示
        let border = 4.0;
        let curx = (self.cursol_x - 1) as f32 * PANEL_WIDTH + WALL_LEFT;
        let cury = (self.cursol_y - 1) as f32 * PANEL_WIDTH + WALL_TOP;
        draw_rectangle_lines(curx, cury,
            PANEL_WIDTH * 3.0, PANEL_HEIGHT * 3.0,
            border, YELLOW);

/*
        // カーソル位置のパネルが開いていて、周囲の爆弾数が１以上の場合
        // カーソル周りに枠を表示
        if let Some(tgt_panel) = self.table.getpanel_idx(self.cursol_index) {
            if tgt_panel.getstat() == 1 && tgt_panel.get_around_num() > 0 {
                let border = 4.0;
                let curx = (self.cursol_x - 1) as f32 * PANEL_WIDTH + WALL_LEFT;
                let cury = (self.cursol_y - 1) as f32 * PANEL_WIDTH + WALL_TOP;
                draw_rectangle_lines(curx, cury,
                    PANEL_WIDTH * 3.0, PANEL_HEIGHT * 3.0,
                    border, YELLOW);
            }
        }
 */
    }

    //------------------------------
    // カーソル位置の処理
    //------------------------------
    pub fn playcontrol(&mut self) {
        // UNDO 処理
        if is_key_pressed(KeyCode::Up) {
            // UNDO情報の最新＝現在なので、UNDO中でなければ
            // １回余計に UNDO する
            if !self.table.is_useundo() {
                self.table.table_undo();
            }
            self.table.table_undo();
            self.stat = 2;
            return;
        }

        // REDO 処理
        if is_key_pressed(KeyCode::Down) {
            self.table.table_redo();
            return;
        }

        // マウス位置の取得
        let (mouse_x, mouse_y) = mouse_position();
        let cursol_x = ((mouse_x - WALL_LEFT) / PANEL_WIDTH) as i32;
        let cursol_y = ((mouse_y - WALL_TOP) / PANEL_HEIGHT) as i32;
        let cursol_index = get_index(cursol_x, cursol_y, self.width, self.height);

        // カーソルは動いたか
        let is_corsol_move = self.cursol_index != cursol_index;

        // カーソル位置をゲーム内に反映
        self.cursol_x = cursol_x.clamp(0, self.width - 1);
        self.cursol_y = cursol_y.clamp(0, self.height - 1);
        self.cursol_index = cursol_index;
        self.table.set_cursol(self.cursol_x, self.cursol_y, self.cursol_index);

        // マウスクリック判定
        let mut is_update = self.click_tbl_left();
        is_update |= self.click_tbl_right();

        // 更新が発生した場合
        if is_update {
            // 今の盤面を保存する
            self.table.undo_push();
            // 爆弾が開かれた場合はステータスを変える
            if self.table.open_bomnum() > 0 {
                self.stat = 3;
            }
        }

        // カーソル周囲９マスのヒントを設定する
        if is_corsol_move || is_update {
            self.table.clear_help();
            self.table.set_bold();
//            self.table.auto_flag();
        }
    }

    //------------------------------
    // 盤面右クリック処理
    // 変更があった場合 true、ない場合は false を返す
    //------------------------------
    fn click_tbl_right (&mut self) -> bool {

        // マウス右クリックされていない、マウスが盤面上ではない、なら何もしない
        if !is_mouse_button_pressed(MouseButton::Right) ||
            self.cursol_index == -1 {
            return false
        }

        // クリックしたことを盤面に伝える
        let result = self.table.click_right(self.cursol_x, self.cursol_y);
        result
    }

    //------------------------------
    // 盤面左クリック処理
    // 変更があった場合 true、ない場合は false を返す
    //------------------------------
    fn click_tbl_left (&mut self) -> bool {
        // マウス左クリックされていない、マウスが盤面上ではない、なら何もしない
        if !is_mouse_button_pressed(MouseButton::Left) ||
           self.cursol_index == -1 {
            return false
        }

        // ゲームが待機中なら初期化しなおす
        if self.stat == 3 {
            self.initial_game();
            self.stat = 1;
            return true
        }

        // ゲームは開始し、クリック待ちなら爆弾を生成する
        if self.stat == 1 {
            self.table.setting_bom(self.bom_num, self.cursol_x, self.cursol_y);
            // 今の盤面を保存する
            self.table.undo_push();
            self.stat = 2;
        }
        
        // クリックしたことを盤面に伝える
        let result = self.table.click_left(self.cursol_x, self.cursol_y);
        result
    }
}