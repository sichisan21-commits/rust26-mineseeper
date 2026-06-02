use macroquad::prelude::*;
use crate::panel::Panel;

pub struct GameTable {
    width: i32,                             // 盤面の幅
    height: i32,                            // 盤面の高さ
    num_bom: i32,                           // 爆弾の数
    table: Vec<Panel>,                      // 盤面データ
    table_undo: Vec<Vec<Panel>>,            // 盤面データ（履歴）
}

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl GameTable {
    //------------------------------
    // 初期化
    //------------------------------
    pub fn new() -> GameTable {
        GameTable {
            width: 0,
            height: 0,
            num_bom: 0,
            table: Vec::new(),
            table_undo: Vec::new(),
        }
    }

    //------------------------------
    // 盤面の初期化
    //------------------------------
    pub fn initial(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;

        // 盤面を初期化する
        self.table.clear();
        self.table_undo.clear();
        for y in 0..self.height {
            for x in 0..self.width {
                self.table.push(Panel::new(x, y, self.width, self.height));
            }
        }
    }

    //------------------------------
    // 爆弾を配置する
    //------------------------------
    pub fn setting_bom(&mut self, num_bom: i32, cursol_x: i32, cursol_y: i32) {
        self. num_bom = num_bom;

        // 爆弾をランダム生成する
        for _ in 0..self.num_bom{
            loop {
                // 爆弾位置をランダム生成する
                let land_x = rand::gen_range(0, self.width);
                let land_y = rand::gen_range(0, self.height);

                // カーソル位置と一致する位置には配置しない
                if land_x == cursol_x && land_y == cursol_y {
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
    fn bomon(&mut self, pos_x:i32, pos_y:i32) -> bool {
        // 座標不正または配置済みの場合 false
        let tblpos = get_index(pos_x, pos_y, self.width, self.height);
        if tblpos == -1 || self.table[tblpos as usize].is_bom() {
            return false;
        }

        // 該当パネルに爆弾を置き
        self.table[tblpos as usize].bomon();

        // 周囲のパネルのカウントを増やす
        let around = self.table[tblpos as usize].get_around();      
        for index in around.into_iter().flatten() {
            if index != -1 {
                self.table[index as usize].num_up();
            }
        }
        true
    }

    //------------------------------
    // 今の盤面をアンドゥ領域に保持
    //------------------------------
    pub fn undo_push(&mut self) {
        self.table_undo.push(self.table.clone());
    }

    //------------------------------
    // 最新のUNDO情報を破棄する
    //------------------------------
    pub fn undo_remove(&mut self) {
        let undonum = self.table_undo.len();
        if undonum > 0 {
            self.table_undo.remove(undonum - 1);
        }
    }

    //------------------------------
    // テーブルをUNDOする
    //------------------------------
    pub fn tableUndo(&mut self) {
        let undonum = self.table_undo.len();
        if undonum > 0 {
            // 一番最後の履歴へ戻す
            self.table = self.table_undo[undonum - 1].clone();
            self.table_undo.remove(undonum - 1);
        }
    }

    //------------------------------
    // 踏まれた爆弾数を取得
    //------------------------------
    pub fn open_bomnum(&mut self) -> usize {
        let op_bomnum = self.table
            .iter()
            .filter(|p| p.is_bom() && p.getstat() == 1)
            .count();
        op_bomnum
    }

    //------------------------------
    // 左クリック処理
    // 変更があった場合 true、ない場合は false を返す
    //------------------------------
    pub fn click_left(&mut self, cursol_x: i32, cursol_y: i32) -> bool {
        // 座標をインデックスに変換
        let cursol_index = get_index(cursol_x, cursol_y, self.width, self.height);
 
        // クリック位置が盤面外、あるいはすでに開かれているなら何もしない
        if cursol_index == -1 || self.table[cursol_index as usize].getstat() == 1{
            return false
        }

        // クリック位置を開く
        self.table[cursol_index as usize].open();

        // クリック位置が爆弾の場合終了
        if self.table[cursol_index as usize].is_bom() {
            return true
        }

        // クリック位置からパネルを連鎖的に開く
        self.openchain(cursol_index);
        self.auto_flag();
        true
    }

    //------------------------------
    // 連鎖的に開く
    //------------------------------
    fn openchain(&mut self, cursol_index: i32) {
        // クリック位置を開く
        self.table[cursol_index as usize].open();

        // 周囲の爆弾数が０でなければ抜ける
        if self.table[cursol_index as usize].get_around_num() != 0{
            return;
        }

        // 周囲をチェックし開いていく
        // 自身の座標情報と周りのインデックス配列を取得
        let around = self.table[cursol_index as usize].get_around();
        for index in around.into_iter().flatten() {
            // クリック位置と同じ場所はスキップ
            // 盤面外ならスキップ
            // 爆弾マスはスキップ（自動で開かない）
            if index == -1 || self.table[index as usize].is_bom() ||
               self.table[index as usize].getstat() != 0 {
                continue;
            }

            // 参照位置を開く
            self.openchain(index);
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
            for pos_x in 0..self.width {
                for pos_y in 0..self.height {
                    is_update |= self.flag_dangar(pos_x, pos_y);
                }
            }
            // 安全マスを判定
            for pos_x in 0..self.width {
                for pos_y in 0..self.height {
                    is_update |= self.flag_safety(pos_x, pos_y);
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
        let cursol_index = get_index(cursol_x, cursol_y, self.width, self.height);

        // 開いていないマスは対象外
        if self.table[cursol_index as usize].getstat() == 0 {
            return is_update;
        }

        // 周囲マスのインデックスを取得
        let around = self.table[cursol_index as usize].get_around();

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
        if self.table[cursol_index as usize].get_around_num() != close_list.len() as i32 {
            return is_update;
        }

        // 周囲の未開封マスが全て爆弾と判断できた
        for index in close_list {
            if  self.table[index as usize].get_autoflag() != 1 {
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
        let cursol_index = get_index(cursol_x, cursol_y, self.width, self.height);

        // 周囲に爆弾なしあるいは未開封パネルの場合はないもしない
        if self.table[cursol_index as usize].get_around_num() == 0 ||
           self.table[cursol_index as usize].getstat() == 0 {
            return is_update
        }

        // 周囲マスのインデックスを取得
        let around = self.table[cursol_index as usize].get_around();

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
        if self.table[cursol_index as usize].get_around_num() != bomnum  {
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

    //------------------------------
    // 盤面を描画する
    //------------------------------
    pub fn draw_panel(&self) {
        for panel in &self.table {
            panel.draw_panel();
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