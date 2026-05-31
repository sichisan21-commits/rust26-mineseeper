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
            stat: 0,
            width: 10,
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
        for y in 0..self.height {
            for x in 0..self.width {
                self.table.push(Panel::new(x, y));
            }
        }

        // カーソル位置を初期化
        self.cursol_x = 0;
        self.cursol_y = 0;

        // クリック待ち
        self.stat = 1;
    }

    //------------------------------
    // 爆弾を配置する
    //------------------------------
    pub fn setting_bom(&mut self) {
        for x in 0..self.bom_num{
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
        for y in -1..2 {
            for x in -1..2 {
                let tblpos = get_index(
                    cursol_x + x, cursol_y + y,
                    self.width, self.height);
                if tblpos != -1 {
                    self.table[tblpos as usize].numup();
                }
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

        for panel in &self.table {
            panel.draw_panel();
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
        self.set_around();

        // マウスクリック判定（左）
        self.click_tbl_left();

        // マウスクリック判定（右）
        self.click_tbl_right();
    }

    //------------------------------
    // 盤面左クリック処理
    //------------------------------
    fn click_tbl_right (&mut self) {
        // マウス右クリックされていない、マウスが盤面上ではない、なら何もしない
        if !is_mouse_button_pressed(MouseButton::Right) || !self.on_table {
            return
        }

        // フラグ処理を行う
        let tblpos = get_index(self.cursol_x, self.cursol_y, self.width, self.height);
        if tblpos != -1 {
            self.table[tblpos as usize].set_userflag();
        }
    }

    //------------------------------
    // 盤面左クリック処理
    //------------------------------
    fn click_tbl_left (&mut self) {
        // マウス左クリックされていない、マウスが盤面上ではない、なら何もしない
        if !is_mouse_button_pressed(MouseButton::Left) || !self.on_table {
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
        if self.table[tblpos].get_around_num() > 0 {
            return;
        }

        // 周囲をチェックし開いていく
        for y in -1..2 {
            for x in -1..2 {
                // 斜めは参照しない
                if x != 0 && y != 0 {
                    continue;
                }

                // 盤面の横位置、縦位置、テーブルインデックスを求める
                let pos_x = cursol_x + x as i32;
                let pos_y = cursol_y + y as i32;

                // 盤面外ならスキップ
                if pos_x < 0 || pos_x >= self.width ||
                   pos_y < 0 || pos_y >= self.height {
                    continue;
                }

                // 連鎖的に開く
                self.openchain(false, pos_x, pos_y);
            }
        }
    }

    //------------------------------
    // カーソル位置の処理
    //------------------------------
    fn set_around(&mut self) {
        // マウスの周囲９マスのインデックスを求める
        let tblpos = self.cursol_y * self.width + self.cursol_x;
        for y in 0..3 {
            for x in 0..3 {
                if self.around[y][x] != -1 {
                    self.table[self.around[y][x] as usize].setflg(0);
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
                    self.table[self.around[y][x] as usize].setflg(1);
                }
            }
        }
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