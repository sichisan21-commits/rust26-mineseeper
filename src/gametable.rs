use macroquad::prelude::*;
use crate::panel::Panel;
use crate::panel::AutoSts;
use crate::utils::*;

pub struct GameTable {
    width: i32,                             // 盤面の幅
    height: i32,                            // 盤面の高さ
    cursol_x: i32,                          // カーソル横位置
    cursol_y: i32,                          // カーソル縦位置
    cursol_index: i32,                      // カーソル位置の配列番号
    num_bom: i32,                           // 爆弾の数
    table: Vec<Panel>,                      // 盤面データ
    table_undo: Vec<Vec<Panel>>,            // 盤面データ（履歴）
    useundo: usize,                         // 使用されるundo番号
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
            cursol_x: 0,
            cursol_y: 0,
            cursol_index: 0,
            num_bom: 0,
            table: Vec::new(),
            table_undo: Vec::new(),
            useundo: 0,
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
        for y in 0..self.height {
            for x in 0..self.width {
                self.table.push(Panel::new(x, y, self.width, self.height));
            }
        }
    }

    //------------------------------
    // カーソル位置をセットする
    //------------------------------
    pub fn set_cursol(&mut self, cursol_x:i32, cursol_y:i32, cursol_index:i32) {
        self.cursol_x = cursol_x;
        self.cursol_y = cursol_y;
        self.cursol_index = cursol_index;
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
        self.table_undo.clear();
        self.useundo = 0;
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
        let around = self.table[tblpos as usize].get_around_tbl();      
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
        // 今参照されている UNDO 情報より後ろは破棄
        self.table_undo.truncate(self.useundo + 1);

        // 最後に保存した UNDO 情報と今の盤面が一致するなら UNDO 情報は保持しない
        if let Some(last) = self.table_undo.last() {
            if last == &self.table {
                return;
            }
        }

        // UNDO 情報を追加
        self.table_undo.push(self.table.clone());
        self.useundo = self.table_undo.len();
    }

    //------------------------------
    // UNDO 実行中か
    //------------------------------
    pub fn is_useundo(&self) -> bool {
        self.useundo < self.table_undo.len()
    }

    //------------------------------
    // テーブルをUNDOする
    //------------------------------
    pub fn table_undo(&mut self) {
        if self.useundo > 0 {
            // 一番最後の履歴へ戻す
            self.useundo -= 1;
            self.table = self.table_undo[self.useundo].clone();
        }
    }

    //------------------------------
    // テーブルをREDOする
    //------------------------------
    pub fn table_redo(&mut self) {
        if self.useundo < self.table_undo.len() -1 {
            // 次の履歴へ戻す
            self.useundo += 1;
            self.table = self.table_undo[self.useundo].clone();
        }
    }

    //------------------------------
    // 踏まれた爆弾数を取得
    //------------------------------
    pub fn open_bomnum(&mut self) -> usize {
        // 開かれている爆弾を数える
        self.table
            .iter()
            .filter(|p| p.is_bom() && p.is_open())
            .count()
    }

    //------------------------------
    // 左クリック処理
    // 変更があった場合 true、ない場合は false を返す
    //------------------------------
    pub fn click_left(&mut self, cursol_x: i32, cursol_y: i32) -> bool {
        // 座標をインデックスに変換
        let cursol_index = get_index(cursol_x, cursol_y, self.width, self.height);
 
        // クリック位置が盤面外、あるいはすでに開かれているなら何もしない
        if cursol_index == -1 || self.table[cursol_index as usize].is_open() {
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
        true
    }

    //------------------------------
    // 左クリック処理
    // 変更があった場合 true、ない場合は false を返す
    //------------------------------
    pub fn click_right(&mut self, cursol_x: i32, cursol_y: i32) -> bool {
        // 座標をインデックスに変換
        let cursol_index = get_index(cursol_x, cursol_y, self.width, self.height);
 
        // クリック位置が盤面外、あるいはすでに開かれているなら何もしない
        if cursol_index == -1 || self.table[cursol_index as usize].is_open() {
            return false
        }

        // 旗の操作
        self.table[cursol_index as usize].set_userflag();
        true
    }

    //------------------------------
    // 全ての補助フラグをクリアする
    //------------------------------
    pub fn clear_help(&mut self) {
        for index in 0..self.width * self.height {
            self.table[index as usize].bold_off();
            self.table[index as usize].set_autoflag(AutoSts::None);
        }
    }

    //------------------------------
    // 強調表示オン
    //------------------------------
    pub fn set_bold(&mut self) {
        for index in 0..self.width * self.height {
            // パネルは開いていて、周囲の爆弾数が１以上なら
            // 強調判定
            if self.table[index as usize].is_open() &&
               self.table[index as usize].get_around_num() > 0 {
                self.update_bold(index);
            }
        }
    }

    //------------------------------
    // 指定マスの周囲９マスをチェックし、旗が立てられそうなら強調表示する
    //------------------------------
    fn update_bold(&mut self, cursol_index:i32) {
        // カーソルの周囲９マスをチェックする
        let around = self.table[cursol_index as usize].get_around_tbl();      

        // 閉じているマスと旗の立てられているマスをカウントする
        let mut close_cnt = 0;
        let mut close_list:Vec<i32> = Vec::new();
        let mut flag_cnt = 0;
        let mut miss_cnt = 0;
        for index in around.into_iter().flatten() {
            // 範囲外のマス、開封済みのマスはスキップ
            if index == -1 ||
                self.table[index as usize].is_open() {
                continue;
            }

            // 未開封のマスをカウントする
            close_cnt += 1;

            if !self.table[index as usize].is_userflag() {
                // 旗が立てられていない場合、未開封位置を保持する
                close_list.push(index);
            } else {
                // 旗が立てられている場合カウントする
                if !self.table[index as usize].is_bom() {
                    // 間違った旗の数をカウント
                    miss_cnt += 1;
                } else {
                    // 正しい旗の数をカウント
                    flag_cnt += 1;                            
                } 
            }
        }

        // フラグが正しく立てられていて爆弾数と一致している場合
        // 安全マスとしてフラグを立てる
        let around_num = self.table[cursol_index as usize].get_around_num();
        if close_cnt > around_num && flag_cnt == around_num && miss_cnt == 0 {
            for index in close_list {
                self.table[index as usize].set_autoflag(AutoSts::Safety);                
            }
            return
        }

        // 周囲の爆弾の数と未開封のマスの数が一致していて
        // 旗の数が一致していない場合強調表示
        if close_cnt == around_num &&
           (flag_cnt != around_num || miss_cnt > 0) {
            self.table[cursol_index as usize].bold_on();
        }
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
        let around = self.table[cursol_index as usize].get_around_tbl();
        for index in around.into_iter().flatten() {
            // 盤面外ならスキップ
            // 爆弾マスはスキップ（自動で開かない）
            // 既に開かれている
            if index == -1 || self.table[index as usize].is_bom() ||
               self.table[index as usize].is_open() {
                continue;
            }

            // 参照位置を開く
            self.openchain(index);
        }
    }

    //------------------------------
    // 自動的に判別し危険マス／安全マスにフラグを立てる
    //------------------------------
    pub fn _auto_flag (&mut self) {
        let mut is_update= false;

        // 一旦全部のフラグを消去する
        for index in 0..self.width * self.height {
            self.table[index as usize].set_autoflag(AutoSts::None);
        }        

        // 無限ループを考慮して、最大１０回試行する
        for _ in 0..1 {
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
    }
 
    //------------------------------
    // 自動で旗を立てる（危険フラグ）
    //------------------------------
    fn flag_dangar (&mut self, cursol_x: i32, cursol_y:i32) -> bool {
        let mut is_update = false;
        let cursol_index = get_index(cursol_x, cursol_y, self.width, self.height);

        // 開いていないマスは対象外
        if !self.table[cursol_index as usize].is_open() {
            return is_update;
        }

        // 周囲マスのインデックスを取得
        let around = self.table[cursol_index as usize].get_around_tbl();

        // 周囲の開いていないパネルを数える
        let mut close_list:Vec<i32> = Vec::new();
        for index in around.into_iter().flatten() {
            // 有効範囲外はまたは開いているパネルはスキップ
            if index == -1 || self.table[index as usize].is_open() {
                continue;
            }

            // 周囲の閉じているパネルのインデックスを保持
            // 安全フラグのパネルは除外
            if self.table[index as usize].get_autoflag() != AutoSts::Safety {
                close_list.push(index);
            }
        }

        // 周囲の開いてないパネル数が一致しなければ終了
        if self.table[cursol_index as usize].get_around_num() != close_list.len() as i32 {
            return is_update;
        }

        // 周囲の未開封マスが全て爆弾と判断できた
        for index in close_list {
            if  self.table[index as usize].get_autoflag() != AutoSts::Danger {
                self.table[index as usize].set_autoflag(AutoSts::Danger);
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
           !self.table[cursol_index as usize].is_open() {
            return is_update
        }

        // 周囲マスのインデックスを取得
        let around = self.table[cursol_index as usize].get_around_tbl();

        // 周囲の開いていないパネルと危険マスを数える
        let mut close_list:Vec<i32> = Vec::new();
        let mut bomnum = 0;
        for index in around.into_iter().flatten() {
            // 盤面外ならスキップ
            if index == -1 {
                continue;
            }

            // 危険フラグが立っている場合爆弾数としてカウント
            if self.table[index as usize].get_autoflag() == AutoSts::Danger {
                bomnum += 1;
            } else if !self.table[index as usize].is_open() &&
                      self.table[index as usize].get_autoflag() != AutoSts::Safety {
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
            if self.table[index as usize].get_autoflag() != AutoSts::Safety {
                self.table[index as usize].set_autoflag(AutoSts::Safety);
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
            panel.draw_panel(self.cursol_x, self.cursol_y);
        }
    }
}