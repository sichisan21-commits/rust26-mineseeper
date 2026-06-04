use macroquad::prelude::*;
use crate::chkbox::ChkBox;
use crate::utils::*;
use crate::myconst::*;
use crate::gametable::GameTable;

// ゲームメインデータ
pub struct GameMain {
    stat: i32,                              // 0:初期画面/1:開始待ち/2:プレイ中
    helplv: i32,                            // アシストレベル
    width: i32,                             // 盤面の幅
    height: i32,                            // 盤面の高さ
    bom_num: i32,                           // 爆弾の数
    cursol_x: i32,                          // カーソル横位置
    cursol_y: i32,                          // カーソル縦位置
    cursol_index: i32,                      // カーソル位置のパネル番号
    table: GameTable,                       // 盤面情報
    chkbox:Vec<ChkBox>,                     // チェックボックス
    //--- アシストモード ---//
    is_frame: bool,                         // カーソル周囲枠の表示
    is_useundo: bool,                       // UNDO 許可
    is_panel_bold: bool,                    // 確実に旗が立てられるマスの強調
    is_auto_mode: bool,                     // 自動で危険マスと安全マスを表示
    is_dang_on: bool,                       // 危険マスの表示
    is_safe_on: bool,                       // 安全マスの表示
}

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl GameMain {
    //------------------------------
    // 初期化
    //------------------------------
    pub fn new () -> GameMain {
        let mut gm = GameMain {
            stat: 0,
            helplv: 0,
            width: 10,
            height: 10,
            bom_num: 0,
            cursol_x: 0,
            cursol_y: 0,
            cursol_index: -1,
            table: GameTable::new(),
            chkbox: Vec::new(),
            is_frame: false,
            is_useundo: false,
            is_panel_bold: false,
            is_auto_mode: false,
            is_dang_on: false,
            is_safe_on: false,
        };

        // チェックボックスを作成
        let mut pos_y = 10;
        pos_y += 1; gm.chkbox.push(ChkBox::new(String::from("CURSOL FLAME"), 1, pos_y, BLACK, gm.is_frame));
        pos_y += 1; gm.chkbox.push(ChkBox::new(String::from("UNDO USE"), 1, pos_y, BLACK, gm.is_useundo));
        pos_y += 1; gm.chkbox.push(ChkBox::new(String::from("BOLD"), 1, pos_y, BLACK, gm.is_panel_bold));
        pos_y += 1; gm.chkbox.push(ChkBox::new(String::from("SAFETY PANEL"), 1, pos_y, BLACK, gm.is_safe_on));
        pos_y += 1; gm.chkbox.push(ChkBox::new(String::from("DANGER PANEL"), 1, pos_y, BLACK, gm.is_dang_on));

        gm
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
        if self.is_frame {
            let border = 4.0;
            let curx = (self.cursol_x - 1) as f32 * PANEL_WIDTH + WALL_LEFT;
            let cury = (self.cursol_y - 1) as f32 * PANEL_WIDTH + WALL_TOP;

            draw_rectangle_lines(curx, cury,
                PANEL_WIDTH * 3.0, PANEL_HEIGHT * 3.0,
                border + 1.0, RED);
            draw_rectangle_lines(curx, cury,
                PANEL_WIDTH * 3.0, PANEL_HEIGHT * 3.0,
                border, YELLOW);
        }

        // アシスト関連情報の表示
        drawtextln(&format!("ASSIST LEVEL={}", self.helplv), 1, 9, BLACK);
        drawtextln(&format!("(PUSH LEFT or RIGHT)"), 1, 10, BLACK);
        // チェックボックスを表示する
        for chkbox in &self.chkbox {
            chkbox.draw();
        }
        // UNDO がオン
        if self.is_useundo {
            drawtextln(&format!("UP=UNDO/DOWN=REDO"), 1, 17, BLACK);
        }
    }

    //------------------------------
    // カーソル位置の処理
    //------------------------------
    pub fn playcontrol(&mut self) {
        let mut is_update = false;

        // アシストレベルの制御
        let is_helpchg = self.help_control();

        //--- UNDO 処理 ---//
        if self.is_useundo {
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
        is_update |= self.click_tbl_left();
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

        // ヒントを作成する
        if is_corsol_move || is_update || is_helpchg {
            self.table.clear_help(self.helplv);
            if self.is_panel_bold {
                self.table.set_bold(self.is_panel_bold, self.is_dang_on, self.is_safe_on);
            }
            if self.is_auto_mode {
                self.table.auto_flag(self.is_dang_on, self.is_safe_on);
            }
        }
    }

    //------------------------------
    // カーソル位置の処理
    //------------------------------
    fn help_control(&mut self) -> bool {
        let mut is_update = false;

        // アシストレベル変更
        if is_key_pressed(KeyCode::Left) {
            self.helplv = (self.helplv - 1).max(0);
            is_update = true;
        }
        if is_key_pressed(KeyCode::Right) {
            self.helplv = (self.helplv + 1).min(5);
            is_update = true;
        }

        // 更新がなければなにもしない
        if !is_update {
            return is_update
        }

        // 各種フラグを初期化
        self.is_frame = false;
        self.is_useundo = false;
        self.is_panel_bold = false;
        self.is_auto_mode = false;
        self.is_dang_on = false;
        self.is_safe_on = false;

        // アシストレベル１なら
        if self.helplv >= 1 {
            self.is_frame = true;
            self.is_useundo = true;
        }
        // アシストレベル１なら
        if self.helplv >= 2 {
            self.is_panel_bold = true;
        }
        // アシストレベル3なら
        if self.helplv >= 3 {
            self.is_safe_on = true;
        }
        // アシストレベル4なら
        if self.helplv >= 4 {
            self.is_auto_mode = true;
            self.is_panel_bold = false;
            self.is_dang_on = true;
            self.is_safe_on = false;
        }

        // アシストレベル5なら
        if self.helplv >= 5 {
            self.is_auto_mode = true;
            self.is_dang_on = true;
            self.is_safe_on = true;
        }

        self.chkbox[0].set_flg(self.is_frame);
        self.chkbox[1].set_flg(self.is_useundo);
        self.chkbox[2].set_flg(self.is_panel_bold);
        self.chkbox[3].set_flg(self.is_safe_on);
        self.chkbox[4].set_flg(self.is_dang_on);
        true
        
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
        let result = self.table.click_right();
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
            // 盤面の１割は必ず開かせる
            let target = self.width * self.height / 10;
            let mut max = 0;
            for _ in 0..100 {
                self.table.setting_bom(self.bom_num);
                self.table.click_left();
                let opennum = self.table.get_opennum();
                // 最も開いたパターンを保持しておく
                if max < opennum {
                    max = opennum;
                    self.table.tbl_backup();
                }
                if target <= opennum as i32 {
                    break;
                }
                self.initial_game();
            }
            // 最も開いた盤面を復旧
            self.table.tbl_restore();

            // 今の盤面を保存する
            self.table.undo_push();
            self.stat = 2;
        }
        
        // クリックしたことを盤面に伝える
        let result = self.table.click_left();
        result
    }
}