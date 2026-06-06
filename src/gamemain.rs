use macroquad::prelude::*;
use crate::draw::*;
use crate::chkbox::ChkBox;
use crate::utils::*;
use crate::myconst::*;
use crate::gametable::GameTable;
use crate::gametable::MyCursol;
use crate::panel::Panel;

struct TableInfo {
    width: i32,                             // 盤面の幅
    height: i32,                            // 盤面の高さ
    bom_num: i32,                           // 爆弾の数
    table: GameTable,                       // 盤面テーブル
    offs: Vec2,                             // 画面オフセット
    zoom: Vec2,                             // 画面倍率
    touch_now: Vec<Touch>,                  // タッチ情報（今）
    touch_prev: Vec<Touch>,                 // タッチ情報（直前）
}

// ゲームメインデータ
pub struct GameMain {
    stat: i32,                              // 0:初期画面/1:開始待ち/2:プレイ中
    scr_width: f32,                         // ウインドウ幅
    scr_height: f32,                        // ウインドウ幅
    helplv: i32,                            // アシストレベル
    mouse_pos: PosTable,                    // マウスカーソル位置
    cursol: MyCursol,                       // カーソル位置
    tbldt: TableInfo,                       // 盤面情報
    //--- アシストモード ---//
    is_frame: bool,                         // カーソル周囲枠の表示
    is_useundo: bool,                       // UNDO 許可
    is_panel_bold: bool,                    // 確実に旗が立てられるマスの強調
    is_auto_mode: bool,                     // 自動で危険マスと安全マスを表示
    is_dang_on: bool,                       // 危険マスの表示
    is_safe_on: bool,                       // 安全マスの表示
    is_allhint: bool,                       // 前面表示
    chkbox:Vec<ChkBox>,                     // チェックボックス
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
            scr_width: screen_width(),
            scr_height: screen_height(),
            helplv: 0,
            mouse_pos: PosTable{ x:0.0, y:0.0},
            cursol: MyCursol {x: -1, y: -1, index: -1},
            tbldt: TableInfo {
                width: 0,
                height: 0,
                bom_num: 0,
                table: GameTable::new(0,0,0),
                offs: Vec2 { x: WALL_LEFT, y: WALL_TOP },
                zoom: Vec2 { x: 0.5, y: 0.5},
                touch_now: Vec::new(),
                touch_prev: Vec::new(),
            },
            chkbox: Vec::new(),
            is_frame: false,
            is_useundo: false,
            is_panel_bold: false,
            is_auto_mode: false,
            is_dang_on: false,
            is_safe_on: false,
            is_allhint: false,
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
        self.tbldt.width = width;
        self.tbldt.height = height;
        self.tbldt.bom_num = bom_num;
    }

    //------------------------------
    // 盤面の初期化
    //------------------------------
    pub fn initial_game(&mut self) {
        // 盤面を初期化する
        self.tbldt.table = GameTable::new(self.tbldt.width, self.tbldt.height, self.tbldt.bom_num);
        self.tbldt.table.initial(self.tbldt.width, self.tbldt.height);
        self.set_tablepos();

        // クリック待ち
        self.stat = 1;
    }

    //------------------------------
    // ゲーム全体の描画
    //------------------------------
    pub fn draw(&mut self) {
        // 盤面全体を塗りつぶす
        clear_window(LAYOUT_COLOR);

        self.draw_table();

        let close_num = self.tbldt.table.get_num_close();
        let flag_num = self.tbldt.table.get_num_redflag();
        let mut pos_y = 1;
        pos_y += 1; drawtextln(&format!("SIZE: {} x {}",self.tbldt.width, self.tbldt.height), 1, pos_y, FONT_SIZE, BLACK);
        pos_y += 1; drawtextln(&format!("BOMB: {}",self.tbldt.bom_num), 1, pos_y, FONT_SIZE, BLACK);
        pos_y += 1; drawtextln(&format!("CLOSE PANEL: {}",close_num), 1, pos_y, FONT_SIZE, BLACK);
        pos_y += 1; drawtextln(&format!("REDFLAG: {}",flag_num), 1, pos_y, FONT_SIZE, BLACK);

        // アシスト関連情報の表示
        drawtextln(&format!("ASSIST LEVEL={}", self.helplv), 1, 9, FONT_SIZE, BLACK);
        drawtextln(&format!("(PUSH LEFT or RIGHT)"), 1, 10, FONT_SIZE, BLACK);
        // チェックボックスを表示する
        for chkbox in &self.chkbox {
            chkbox.draw();
        }
        // UNDO がオン
        if self.is_useundo {
            drawtextln(&format!("'U'=UNDO/'R'=REDO"), 1, 17, FONT_SIZE, BLACK);
        }

        // マウス位置表示
        draw_circle(self.mouse_pos.x, self.mouse_pos.y, 5.0, BLACK);

    }

    //------------------------------
    // 盤面の描画
    //------------------------------
    fn draw_table(&self) {
        // カメラをセット
        let zoom = Vec2 {
            x: self.tbldt.zoom.x * 2.0 / screen_width(),
            y: self.tbldt.zoom.x * 2.0 / screen_height(),
        };
        let offset = Vec2 {
            x: self.tbldt.offs.x * 2.0 / screen_width() - 1.0,
            y: - (self.tbldt.offs.y * 2.0 / screen_height()) + 1.0,
        };
        let camera = Camera2D {
            zoom, offset,
            ..Default::default()
        };
        set_camera(&camera);

        let offs = 5.0;
        draw_rectangle( -offs, -offs, self.tbldt.width as f32 * PANEL_WIDTH + offs * 2.0, self.tbldt.height as f32 * PANEL_HEIGHT + offs * 2.0, BLACK);
        // 盤面の描画
        self.tbldt.table.draw_panel(self.is_allhint);
        // カーソル周りに枠を表示
        if self.is_frame {
            self.tbldt.table.draw_curasol();
        }

        // カメラをリセット
        set_default_camera();
    }

    //------------------------------
    // 入力制御
    //------------------------------
    pub fn playcontrol(&mut self) {
        let mut is_update = false;

        let is_keyupdate = self.keycontrol();

        let is_corsol_move = self.mouse_move();

        // マウスクリック判定
        is_update |= self.click_tbl_left();
        is_update |= self.click_tbl_right();

        // ヒントを作成する
        if is_update || is_keyupdate {
            self.tbldt.table.clear_help(self.helplv);
            if self.is_panel_bold {
                self.tbldt.table.set_bold(self.is_panel_bold, self.is_dang_on, self.is_safe_on);
            }
            if self.is_auto_mode {
                self.tbldt.table.auto_flag(self.is_dang_on, self.is_safe_on);
            }
        }

        // 更新が発生した場合
        if is_update {
            // 今の盤面を保存する
            self.tbldt.table.undo_push();
            // 爆弾が開かれた場合はステータスを変える
            if self.tbldt.table.open_bomnum() > 0 {
                self.stat = 3;
            }
        }
    }

    //------------------------------
    // マウス位置をテーブルへ反映
    //------------------------------
    fn mouse_move(&mut self) -> bool {
        let mut is_update = false;

        // 画面サイズの取得
        self.scr_width = screen_width();
        self.scr_height = screen_height();
        
        // マウス位置の取得
        let (x,y) = mouse_position();
        self.mouse_pos.x = x;
        self.mouse_pos.y = y;

        // カーソル座標へ変換
        let cursol_x = ((self.mouse_pos.x - WALL_LEFT) / PANEL_WIDTH) as i32;
        let cursol_y = ((self.mouse_pos.y - WALL_TOP) / PANEL_HEIGHT) as i32;
        let cursol = MyCursol {
            x: cursol_x, y: cursol_y,
            index: get_index(cursol_x, cursol_y, self.tbldt.width, self.tbldt.height),
        };

        // 盤面にマウス位置を反映
        let tablepos = Vec2 {
            x: (self.mouse_pos.x - self.tbldt.offs.x) * (1.0 / self.tbldt.zoom.x),
            y: (self.mouse_pos.y - self.tbldt.offs.y) * (1.0 / self.tbldt.zoom.y),
        };
        self.tbldt.table.set_mousepos(tablepos);

        // 上または左の壁を越えたら移動を開始するようにする
        let mousepos_x = (self.mouse_pos.x - WALL_LEFT).max(0.0);
        let mousepos_y = (self.mouse_pos.y - WALL_TOP).max(0.0);

        // 盤面が大きい／ウインドウサイズが狭いとカーソルが早くなる
//        let move_x = self.tbldt.width as f32 / 15.0 * (1000.0 / self.scr_width);
//        let move_y = self.tbldt.width as f32 / 15.0 * (1000.0 / self.scr_height);
//        let move_x = self.tbldt.width as f32 / 15.0;
//        let move_y = self.tbldt.width as f32 / 15.0;

        // 盤面のリアルサイズを求める
        let real_width = self.tbldt.width as f32 * PANEL_WIDTH * self.tbldt.zoom.x;
        let real_height = self.tbldt.height as f32 * PANEL_HEIGHT * self.tbldt.zoom.y;

        let move_x = (real_width + WALL_LEFT - self.scr_width) / (self.scr_width - WALL_LEFT - WALL_RIGHT);
        let move_y = (real_height + WALL_TOP - self.scr_height) / (self.scr_height - WALL_TOP - WALL_BOTTOM);

        // 原点側にスクロールしすぎないよう、盤面の幅から最小座標を求める
        let min_left= self.scr_width - real_width - WALL_RIGHT;
        let min_top = self.scr_height - real_height - WALL_BOTTOM;

        // オフセットに反映する
        // このとき盤面が壁のサイズ（WALL_XXXX）を超えてスクロールしないよう制御する
        self.tbldt.offs.x = ((WALL_LEFT - mousepos_x * move_x).max(min_left)).min(WALL_LEFT);
        self.tbldt.offs.y = ((WALL_TOP - mousepos_y * move_y).max(min_top)).min(WALL_TOP);

        // カーソルは動いたか
        is_update |= self.cursol.index != cursol.index;
        self.cursol = cursol;

        is_update
        
    }

   fn set_tablepos(&mut self) {
        let tablepos = Vec2 {
            x: (self.mouse_pos.x - self.tbldt.offs.x) * (1.0 / self.tbldt.zoom.x),
            y: (self.mouse_pos.y - self.tbldt.offs.y) * (1.0 / self.tbldt.zoom.y),
        };
        self.tbldt.table.set_mousepos(tablepos);
    }

     //------------------------------
    // キーボード入力処理
    //------------------------------
    fn keycontrol(&mut self) -> bool {
        let mut is_update = false;

        //--- UNDO 処理 ---//
        if self.is_useundo {
            if is_key_pressed(KeyCode::Up) {
                // UNDO情報の最新＝現在なので、UNDO中でなければ
                // １回余計に UNDO する
                if !self.tbldt.table.is_useundo() {
                    self.tbldt.table.table_undo();
                }
                self.tbldt.table.table_undo();
                self.stat = 2;
                return false;
            }

            // REDO 処理
            if is_key_pressed(KeyCode::Down) {
                self.tbldt.table.table_redo();
                return false;
            }
        }

        // アシストレベルの制御
        is_update |= self.help_control();
        is_update
    }

    //------------------------------
    // アシストレベルの変更
    //------------------------------
    fn help_control(&mut self) -> bool {
        let mut is_update = false;

        // アシストレベル変更
        if is_key_pressed(KeyCode::Left) {
            self.helplv = (self.helplv - 1).max(0);
            is_update = true;
        }
        if is_key_pressed(KeyCode::Right) {
            self.helplv = (self.helplv + 1).min(6);
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
        self.is_allhint = false;

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

        if self.helplv >= 6 {
            self.is_allhint = true;
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
            self.cursol.index == -1 {
            return false
        }

        // クリックしたことを盤面に伝える
        let result = self.tbldt.table.click_right();
        result
    }

    //------------------------------
    // 盤面左クリック処理
    // 変更があった場合 true、ない場合は false を返す
    //------------------------------
    fn click_tbl_left (&mut self) -> bool {
        // マウス左クリックされていない、マウスが盤面上ではない、なら何もしない
        if !is_mouse_button_pressed(MouseButton::Left) ||
           self.cursol.index == -1 {
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
            let mut table_backup: Vec<Panel> = Vec::new();

            // 盤面の１割は必ず開かせる
            let target = self.tbldt.width * self.tbldt.height / 10;
            let mut max = 0;
            for x in 0..100 {
                self.tbldt.table.setting_bom(self.tbldt.bom_num);
                self.tbldt.table.click_left();
                let opennum = self.tbldt.table.get_opennum();
                // 最も開いたパターンを保持しておく
                if max < opennum {
                    max = opennum;
                    table_backup = self.tbldt.table.tbl_backup();
                }
                if target <= opennum as i32 {
                    break;
                }
                self.initial_game();
            }
            // 最も開いた盤面を復旧
            self.tbldt.table.tbl_restore(table_backup);

            // 今の盤面を保存する
            self.tbldt.table.undo_push();
            self.stat = 2;
        }
        
        // クリックしたことを盤面に伝える
        let result = self.tbldt.table.click_left();
        result
    }
}