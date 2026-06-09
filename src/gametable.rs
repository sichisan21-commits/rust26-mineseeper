use macroquad::prelude::*;
use crate::panel::Panel;
use crate::utils::*;
use crate::draw::*;
use crate::myconst::*;

#[derive(Debug, Copy, Clone)]
pub struct MyCursol {                               // カーソル情報
    pub x: i32,                                     // 横位置
    pub y: i32,                                     // 縦位置
    pub index: i32,                                 // インデックス
}

// 推論テーブル
struct Inference {
    myindex: i32,
    index: Vec<i32>,
    bomnum: i32,
}

// ゲーム情報
pub struct GameTable {
    width: i32,                             // 盤面の幅
    height: i32,                            // 盤面の高さ
    num_bom: i32,                           // 爆弾の数
    mouse_pos: Vec2,                        // マウス位置
    cursol: MyCursol,                       // カーソル位置
    table: Vec<Panel>,                      // 盤面データ
    table_undo: Vec<Vec<Panel>>,            // 盤面データ（履歴）
    useundo: usize,                         // 使用されるundo番号
    inference: Vec<Inference>,              // 推論テーブル
}


//--------------------------------------------------
// 実装
//--------------------------------------------------
impl GameTable {
    //------------------------------
    // 初期化
    //------------------------------
    pub fn new(width:i32, height:i32, num_bom:i32) -> GameTable {
        GameTable {
            width,
            height,
            num_bom,
            mouse_pos: Vec2 {x:0.0, y:0.0},
            cursol: MyCursol {x:0, y:0, index:0},
            table: Vec::new(),
            table_undo: Vec::new(),
            useundo: 0,
            inference: Vec::new(),
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
    pub fn set_mousepos(&mut self, mouse_pos: Vec2) -> MyCursol {
        self.mouse_pos = mouse_pos;
        let cursol_x = (mouse_pos.x / PANEL_WIDTH) as i32;
        let cursol_y = (mouse_pos.y / PANEL_HEIGHT) as i32;
        self.cursol = MyCursol {
            x: cursol_x, y: cursol_y,
            index: get_index(cursol_x, cursol_y, self.width, self.height),
        };
        self.cursol
    }
/*
    //------------------------------
    // 爆弾を配置する(テスト用)
    //------------------------------
    pub fn _setting_bom(&mut self, num_bom: i32) {
        for x in 0..self.width {
            for y in 0..self.height {
                if y > 2  || x > 6 {
                    if x % 3 == 0 || x > 6{
                        self.bomon(x, y);
                    }
                }
            }
        }
    }
 */

    //------------------------------
    // 爆弾を配置する
    //------------------------------
    pub fn setting_bom(&mut self, num_bom: i32) {
        self. num_bom = num_bom;

        // 爆弾をランダム生成する
        for _ in 0..self.num_bom{
            loop {
                // 爆弾位置をランダム生成する
                let land_x = rand::gen_range(0, self.width);
                let land_y = rand::gen_range(0, self.height);

                // カーソル位置と一致する位置には配置しない
                if land_x == self.cursol.x && land_y == self.cursol.y {
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
    // 開かれたパネルの数を取得
    //------------------------------
    pub fn get_opennum(&self) -> usize {
        self.table
            .iter()
            .filter(|p| p.is_open())
            .count()
    }

    //------------------------------
    // 赤い旗の数を取得
    //------------------------------
    pub fn get_num_redflag(&self) -> usize {
        self.table
            .iter()
            .filter(|p| p.get_userflg() == UserFlg::RedFlg)
            .count()
    }

    //------------------------------
    // すべての危険マスに旗を立てる
    //------------------------------
    pub fn set_all_redflag(&mut self) {
        for index in 0..self.width * self.height {
            if self.table[index as usize].get_autoflag() == AutoSts::Danger {
                self.table[index as usize].set_userflg(UserFlg::RedFlg);
            }
        }
    }

    //------------------------------
    // 盤面のバックアップ
    //------------------------------
    pub fn tbl_backup(&self) -> Vec<Panel> {
        self.table.clone()
    }

    //------------------------------
    // バックアップから復帰
    //------------------------------
    pub fn tbl_restore(&mut self, table:Vec<Panel>) {
        self.table = table.clone();
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
    // UNDO 実行中か
    //------------------------------
    pub fn is_useundo(&self) -> bool {
        self.useundo < self.table_undo.len()
    }

    //------------------------------
    // 踏まれた爆弾数を取得
    //------------------------------
    pub fn open_bomnum(&mut self) -> usize {
        // 開かれている爆弾を数える
        self.table
            .iter()
            .filter(|p| p.is_bomopen())
            .count()
    }

    //------------------------------
    // 左クリック処理
    // 変更があった場合 true、ない場合は false を返す
    //------------------------------
    pub fn click_left(&mut self) -> bool { 
        // クリック位置が盤面外、あるいはすでに開かれているなら何もしない
        if self.cursol.index == -1 || self.table[self.cursol.index as usize].is_open() {
            return false
        }

        // クリック位置を開く
        self.table[self.cursol.index as usize].open();
        
        // クリック位置が爆弾の場合終了
        if self.table[self.cursol.index as usize].is_bom() {
            return true
        }

        // クリック位置からパネルを連鎖的に開く
        self.openchain(self.cursol.index);
        true
    }

    //------------------------------
    // →クリック処理
    // 変更があった場合 true、ない場合は false を返す
    //------------------------------
    pub fn click_right(&mut self) -> bool {
        // 座標をインデックスに変換
        // クリック位置が盤面外、あるいはすでに開かれているなら何もしない
        if self.cursol.index == -1 || self.table[self.cursol.index as usize].is_open() {
            return false
        }

        // 旗の操作
        self.table[self.cursol.index as usize].userflg();
        true
    }

    //------------------------------
    // 全ての補助フラグをクリアする
    //------------------------------
    pub fn clear_help(&mut self, helplv:i32) {
        for index in 0..self.width * self.height {
            self.table[index as usize].bold_off();
            if helplv == 0 {
                self.table[index as usize].set_autoflag(AutoSts::None);
            } else {
                self.table[index as usize].set_autoflag(AutoSts::Unknown);
            }
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
    pub fn auto_flag (&mut self, _is_dang_on: bool, is_safe_on: bool) {
        let mut is_update;

        // 無限ループを考慮して、最大１０回試行する
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

            // 高度推論を行う
            for _ in 0..10 {
                self.inference.clear();
                self.make_inference();
                self.find_inference();
            }

            // フラグの更新がなければループ終了
            if !is_update {
                break;
            }
        }

        // 安全フラグ非表示の場合、安全マスを消す
        if !is_safe_on {
            for index in 0..self.width * self.height {
                if self.table[index as usize].get_autoflag() == AutoSts::Safety {
                    self.table[index as usize].set_autoflag(AutoSts::Unknown);
                }
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

    fn make_inference(&mut self) {
        for index in 0..self.width * self.height {
                self.create_inference(index);
        }
    }

    fn create_inference(&mut self, cursol_index: i32) {
        // 閉じているマスは何もしない
        if !self.table[cursol_index as usize].is_open() {
            return
        }

        // 対象マスの周囲９マスを取得する
        let around = self.table[cursol_index as usize].get_around_tbl();

        // 周囲の閉じているマス（安全でも危険でもないマス）をカウントする
        let mut close_list:Vec<i32> = Vec::new();
        let mut dang_cnt = 0;
        for index in around.into_iter().flatten() {
            // 範囲外および開いているマスはスキップ
            if index == -1 || self.table[index as usize].is_open() {
                continue;
            }
            // パネルの自動フラグを取得
            let auto_flag = self.table[index as usize].get_autoflag();
            if auto_flag == AutoSts::Danger {
                // 危険パネルのカウント
                dang_cnt += 1;
            } else if auto_flag == AutoSts::None || auto_flag == AutoSts::Unknown {
                // 未開封パネルの保持
                close_list.push(index);
            }
        }
        // 見つかっていない爆弾数を求める
        let bomnum = self.table[cursol_index as usize].get_around_num() - dang_cnt;
        if bomnum > 0 {
            self.inference.push(
                Inference {
                    myindex: cursol_index,
                    index: close_list,
                    bomnum: bomnum,
                }
            )
        }
    }

    fn find_inference(&mut self) {
        println!("find_inference");
        println!("----------");
        for index in 0..self.width * self.height {
            self.match_inference(index);
        }
    }

    fn match_inference(&mut self, cursol_index: i32) {
        // 閉じているマスは何もしない
        if !self.table[cursol_index as usize].is_open() {
            return
        }

        // 周囲９マスの中から、未確定のマスだけリスト化する
        let around = self.table[cursol_index as usize].get_around_tbl();
        let mut around_num = self.table[cursol_index as usize].get_around_num();
        let mut myclose_index:Vec<i32> = Vec::new();
        for index in around.into_iter().flatten() {
            // 開いているマスはスキップ
            if index == -1 || self.table[index as usize].is_open() {
                continue
            }

            // テーブル内の自動判定フラグを確認
            match self.table[index as usize].get_autoflag() {
                // 確定している危険マスがあるなら爆弾数から差し引く
                AutoSts::Danger => {
                    around_num -= 1;
                }
                // 周囲の未確定のマスは保存する
                AutoSts::None | AutoSts::Unknown => {
                    myclose_index.push(index);
                }
                _ => {}
            }
        }

        // 周りに未確定のマスがない、またはすべての爆弾は確定している
        if myclose_index.is_empty() || around_num == 0{
            return;
        }

        // 推論テーブルでループする
        for inference in &self.inference {
            // 自分自身とは比較しない
            if inference.myindex == cursol_index {
                continue;
            }

            // 何度も検証するためテーブルはコピーしておく
//            let mut myclose_clone = myclose_index.clone();
//            let mut is_exists = true;

            // 推論テーブルのマスが現在の周囲９マスに含まれるか
            // 含まれなければスキップ
            if !inference.index.iter().all(|idx| myclose_index.contains(idx)) {
                continue;
            }

            // myclose_index から inference.index を除いた残りを作る
            let remain: Vec<i32> = myclose_index
                .iter()
                .cloned()
                .filter(|idx| !inference.index.contains(idx))
                .collect();

            // 判定中に -1 した要素を削除
//            myclose_clone.retain(|&x| x != -1);

            // 残爆弾数が一致した場合、重複していないマスは安全
            if around_num == inference.bomnum {
                for index in remain  {
                    self.table[index as usize].set_autoflag(AutoSts::Safety);
                }
            } else if around_num - inference.bomnum == remain.len() as i32 {
            // 残ったマスの数と差し引き爆弾数が同じ場合は危険
                for index in remain {
                    self.table[index as usize].set_autoflag(AutoSts::Danger);
                }
            }
            break;
        }
    }

    //------------------------------
    // 盤面を描画する
    //------------------------------
    pub fn draw_panel(&self, is_allhint: bool) {
        // 盤面を表示
        for panel in &self.table {
            panel.draw_panel(self.cursol.x, self.cursol.y, is_allhint);

        }
    }

    //------------------------------
    // カーソルを描画する
    //------------------------------
    pub fn draw_curasol(&self) {
        let border = 6.0;
        // 一旦上下左右のパネル位置を求める
        let left  = self.cursol.x.clamp(0,self.width - 1);
        let top  = self.cursol.y.clamp(0,self.height - 1);

        // カーソル枠の描画
        draw_rectangle_lines(
            (left - 1) as f32 * PANEL_WIDTH, (top - 1) as f32 * PANEL_HEIGHT,
            PANEL_WIDTH * 3.0, PANEL_HEIGHT * 3.0,
            border + 3.0, RED);
    }
}