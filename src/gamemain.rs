use macroquad::prelude::*;
use crate::panel::Panel;
use crate::{PANEL_WIDTH,PANEL_HEIGHT,WALL_LEFT,WALL_TOP};
use crate::LAYOUT_COLOR;

// ゲームメインデータ
pub struct GameMain {
    stat: i32,                              // 0:初期画面/1:開始待ち/2:プレイ中
    width: i32,                             // 盤面の幅
    height: i32,                            // 盤面の高さ
    bom_num: i32,                           // 爆弾の数
    table: Vec<Panel>,                      // 盤面データ
    table_undo: Vec<Vec<Panel>>,            // 盤面データ（やり直し）
    cursol_x: i32,                          // カーソル横位置
    cursol_y: i32,                          // カーソル縦位置
    cursol_index: i32,                      // カーソル位置のパネル番号
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
            table: Vec::new(),
            table_undo: Vec::new(),
            cursol_x: 0,
            cursol_y: 0,
            cursol_index: -1,
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
        self.table_undo.clear();
        for y in 0..self.height {
            for x in 0..self.width {
                self.table.push(Panel::new(x, y, self.width, self.height));
            }
        }

        // カーソル位置を初期化
        self.cursol_x = -1;
        self.cursol_y = -1;

        // クリック待ち
        self.stat = 1;
    }

    //------------------------------
    // 爆弾を配置する
    //------------------------------
    pub fn setting_bom(&mut self) {
        for _ in 0..self.bom_num{
            loop {
                // 爆弾位置をランダム生成する
                let land_x = rand::gen_range(0, self.width);
                let land_y = rand::gen_range(0, self.height);

                // カーソル位置と一致する位置には配置しない
                if land_x == self.cursol_x && land_y == self.cursol_y {
                    continue;
                }

                // 爆弾を配置
                if self.bomon(land_x, land_y) {
                    break;
                }
            }
        }
    }

    //------------------------------
    // 指定のマスに爆弾を置く
    //------------------------------
    fn bomon(&mut self, cursol_x:i32, cursol_y:i32) -> bool {
        // 座標不正または配置済みの場合 false
        let tblpos = get_index(cursol_x, cursol_y, self.width, self.height);
        if tblpos == -1 || self.table[tblpos as usize].is_bom() {
            return false;
        }

        // 該当パネルに爆弾を置き
        // 周囲のパネルのカウントを増やす
        self.table[tblpos as usize].bomon();
        let around = self.table[tblpos as usize].get_around();      
        for index in around.into_iter().flatten() {
            if index != -1 {
                self.table[index as usize].numup();
            }
        }
        true
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
        for panel in &self.table {
            panel.draw_panel();
        }

        // カーソル周りに枠を表示
        // カーソルのあっているパネルが開いていて、周囲の爆弾数が１以上の場合
        if self.cursol_index != -1 &&
           self.table[self.cursol_index as usize].getstat() == 1 &&
           self.table[self.cursol_index as usize].get_around_num() > 0 {
            let border = 4.0;
            let curx = (self.cursol_x - 1) as f32 * PANEL_WIDTH + WALL_LEFT;
            let cury = (self.cursol_y - 1) as f32 * PANEL_WIDTH + WALL_TOP;
            draw_rectangle_lines(curx, cury,
                 PANEL_WIDTH * 3.0, PANEL_HEIGHT * 3.0,
                 border, YELLOW);
        }
    }

    //------------------------------
    // カーソル位置の処理
    //------------------------------
    pub fn playcontrol(&mut self) {
        // やり直し処理
        if is_key_pressed(KeyCode::Up) && self.table_undo.len() > 0 {
            // 一番最後の履歴へ戻す
            self.table = self.table_undo[self.table_undo.len()-1].clone();
            self.table_undo.remove(self.table_undo.len()-1);
            self.stat = 2;
            return;
        }

        // マウス位置の取得
        let (mouse_x, mouse_y) = mouse_position();
        let cursol_x = ((mouse_x - WALL_LEFT) / PANEL_WIDTH) as i32;
        let cursol_y = ((mouse_y - WALL_TOP) / PANEL_HEIGHT) as i32;
        self.cursol_index = get_index(cursol_x, cursol_y, self.width, self.height);
//        println!("{},{},{}",cursol_x, cursol_y, self.cursol_index);

        // カーソル位置が盤面からはみ出さないよう制御
        self.cursol_x = cursol_x.clamp(0, self.width - 1);
        self.cursol_y = cursol_y.clamp(0, self.height - 1);

        // マウスクリック判定（左）
        self.click_tbl_left();

        // マウスクリック判定（右）
        self.click_tbl_right();

        // 危険マス・安全マスの自動判定
        self.auto_flag();
    }

    //------------------------------
    // 盤面右クリック処理
    //------------------------------
    fn click_tbl_right (&mut self) -> bool {
        let mut is_update = false;

        // マウス右クリックされていない、マウスが盤面上ではない、なら何もしない
        if !is_mouse_button_pressed(MouseButton::Right) ||
            self.cursol_index == -1 {
            return is_update
        }

        // フラグ処理を行う
        let tblpos = get_index(self.cursol_x, self.cursol_y, self.width, self.height);
        if tblpos != -1 {
            self.table[tblpos as usize].set_userflag();
            is_update = true;
        }
        is_update
    }

    //------------------------------
    // 盤面左クリック処理
    //------------------------------
    fn click_tbl_left (&mut self) {
        // マウス左クリックされていない、マウスが盤面上ではない、なら何もしない
        if !is_mouse_button_pressed(MouseButton::Left) ||
           self.cursol_index == -1 {
            return
        }

        // ゲームが待機中なら初期化しなおす
        if self.stat == 3 {
            self.initial_game();
            self.stat = 1;
            return;
        }
        // ゲームは開始し、クリック待ちなら爆弾を生成する
        if self.stat == 1 {
            self.setting_bom();
            self.stat = 2;
        }

        // 変更前の盤面を保存
        self.table_undo.push(self.table.clone());

        // カーソル位置及び隣接位置を開く
        self.openchain(true, self.cursol_x, self.cursol_y);
    }

    //------------------------------
    // 連鎖的に開く
    //------------------------------
    fn openchain(&mut self, onclick: bool, cursol_x: i32, cursol_y:i32) {
        // すでに開いているなら何もしない
        let tblpos = (cursol_y * self.width + cursol_x) as usize;
        if self.table[tblpos].getstat() == 1 {
            return;
        }

        // 参照位置に爆弾がある
        if self.table[tblpos].is_bom() {
            // 直接クリックした場合だけ開く
            if onclick {
                self.table[tblpos].open();
                // 暫定で状態を変える
                self.stat = 3;
            }
            return;
        } 

        // クリック位置を開く
        self.table[tblpos].open();

        // 周囲をチェックし開いていく
        for y in -1..2 {
            for x in -1..2 {
                // 斜めは参照しない（周りに爆弾が一つでもある場合）
                if self.table[tblpos].get_around_num() != 0 &&
                   x != 0 && y != 0 {
                    continue;
                }

                // 盤面の横位置、縦位置、テーブルインデックスを求める
                let pos_x = cursol_x + x as i32;
                let pos_y = cursol_y + y as i32;
                let index = get_index(pos_x, pos_y, self.width, self.height);
                // 盤面外ならスキップ
                if index == -1 {
                    continue;
                }

                // 連鎖的に開く
                if self.table[tblpos].get_around_num() == 0 {
                   self.openchain(false, pos_x, pos_y);
                }
            }
        }
    }

    //------------------------------
    // 自動的に判別し危険マス／安全マスにフラグを立てる
    //------------------------------
    fn auto_flag (&mut self) {
        let mut is_update= false;

        // 無限ループを考慮して、最大１０回志向する
        for _ in 0..10 {
            is_update = false;

            // 危険マスを判定
            for x in 0..self.width {
                for y in 0..self.height {
                    is_update |= self.flag_dangar(x, y);
                }
            }

            // 安全マスを判定
            for x in 0..self.width {
                for y in 0..self.height {
                    is_update |= self.flag_safety(x, y);
                }
            }

            // フラグの更新がなければループ終了
            if !is_update {
                break;
            }
        }
        if is_update {
            println!("試行回数に達した");
        }
    }

    //------------------------------
    // 自動で旗を立てる（危険フラグ）
    //------------------------------
    fn flag_dangar (&mut self, cursol_x: i32, cursol_y:i32) -> bool {
        let mut is_update = false;

        // 周囲に爆弾なしの場合はないもしない
        let tblpos = get_index(cursol_x, cursol_y, self.width, self.height);
        if tblpos == -1 || self.table[tblpos as usize].get_around_num() == 0 {
            return is_update
        }

        // 周囲マスのインデックスを取得
        let around = self.table[tblpos as usize].get_around();

        // 周囲の開いていないパネルを数える
        let mut close_list:Vec<i32> = Vec::new();
        for index in around.into_iter().flatten() {
            // 有効範囲外はスキップ
            if index == -1 {
                continue;
            }

            // 周囲の閉じているパネルのインデックスを保持
            // 安全フラグのパネルは除外
            if self.table[index as usize].getstat() == 0 && 
               self.table[index as usize].get_autoflag() != 2 {
                close_list.push(index);
            }
        }

        // 周囲の開いてないパネル数が一致しなければ終了
        if self.table[tblpos as usize].get_around_num() != close_list.len() as i32 {
            return is_update;
        }

        // 周囲の未開封マスが全て爆弾と判断できた
        for index in close_list {
            // 危険フラグを立てる
            if self.table[index as usize].get_autoflag() != 1 {
                self.table[index as usize].set_autoflag(1);
                is_update = true;
            }
        }
        is_update
    }

    //------------------------------
    // 自動で旗を立てる（安全フラグ）
    //------------------------------
    fn flag_safety (&mut self, cursol_x: i32, cursol_y:i32) -> bool {
        let mut is_update = false;

        // 周囲に爆弾なしあるいは未開封パネルの場合はないもしない
        let tblpos = get_index(cursol_x, cursol_y, self.width, self.height);
        if tblpos == -1 || self.table[tblpos as usize].get_around_num() == 0 || self.table[tblpos as usize].getstat() == 0{
            return is_update
        }

        // 周囲マスのインデックスを取得
        let around = self.table[tblpos as usize].get_around();

        // 周囲の開いていないパネルと危険マスを数える
        let mut close_list:Vec<i32> = Vec::new();
        let mut bomnum = 0;
        for index in around.into_iter().flatten() {
            // 盤面外ならスキップ
            if index == -1 {
                continue;
            }

            // 危険フラグが立っている場合爆弾数としてカウント
            if self.table[index as usize].get_autoflag() == 1 {
                bomnum += 1;
            } else if self.table[index as usize].getstat() == 0 &&
                      self.table[index as usize].get_autoflag() != 2 {
                // 周囲の閉じているパネルのインデックスを保持
                close_list.push(index);
            }
        }

        // 危険フラグの数が周囲の爆弾数と一致していなければ抜ける
        if self.table[tblpos as usize].get_around_num() != bomnum  {
            return is_update
        }

        // 爆弾数と危険フラグ数が一致しているなら
        // 残りの未開封パネルに安全フラグを立てる
        for index in close_list {
            if self.table[index as usize].get_autoflag() != 2 {
                self.table[index as usize].set_autoflag(2);
                is_update = true;
            }
        }
        is_update
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