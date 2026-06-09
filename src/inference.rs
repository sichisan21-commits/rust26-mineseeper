use crate::panel::Panel;
use crate::myconst::*;

pub struct InfTable {						// 推論処理用データ
	table: Vec<Panel>,                      // 盤面データコピー
	width: i32,								// 盤面の幅
	height: i32,							// 盤面の高さ
}

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl InfTable {
	//------------------------------
	// 初期化
	//------------------------------
	pub fn new(table:Vec<Panel>, width:i32, height:i32) -> InfTable {
		InfTable {
			table,
			width,
			height,
		}
	}

	//------------------------------
	// 加工後のテーブルを返却する
	//------------------------------
	pub fn get_table(&mut self) -> Vec<Panel> {
		self.table.clone()
	}

	//------------------------------
	// 旗が立てられる可能性のあるマスの強調表示
	//------------------------------
	pub fn set_bold(&mut self, is_safe_on:bool) {
		// 一旦すべてのフラグを落とす
		for index in 0..self.width * self.height {
			self.table[index as usize].set_autoflag(AutoSts::Unknown);
		}

		// 強調フラグ判定
		for index in 0..self.width * self.height {
			// 開いているパネルで
			// 周囲の爆弾数が１以上なら強調判定
			if self.table[index as usize].is_open() &&
			   self.table[index as usize].get_around_num() > 0 {
				self.table[index as usize].bold_off();
				self.update_bold(is_safe_on, index);
			}
		}
	}

	//------------------------------
	// 指定マスの周囲９マスをチェックし、旗が立てられそうなら強調表示する
	//------------------------------
	fn update_bold(&mut self, is_safe_on: bool, cursol_index:i32) {
		// カーソルの周囲９マスをチェックする
		let around = self.table[cursol_index as usize].get_around_tbl();      

		// 閉じているマスと旗の立てられているマスをカウントする
		let mut close_cnt = 0;
		let mut flag_cnt = 0;
		let mut miss_cnt = 0;

		for index in around.into_iter().flatten() {
			// 範囲外、または開封済みのマスはスキップ
			if index == -1 || self.table[index as usize].is_open() {
				continue;
			}

			// 未開封のマスをカウントする
			close_cnt += 1;

			// 旗が立てられている場合カウントする
			if self.table[index as usize].get_userflg() != UserFlg::None {
				if !self.table[index as usize].is_bom() {
					// 間違った旗の数をカウント
					miss_cnt += 1;
				} else {
					// 正しい旗の数をカウント
					flag_cnt += 1;                            
				} 
			}
		}

		// 周囲の爆弾数を取得
		let around_num = self.table[cursol_index as usize].get_around_num();

		// 周囲の爆弾の数と未開封のマスの数が一致していて
		// 旗の数が一致していない（旗を立てるべきマスがある）場合強調表示
		if close_cnt == around_num && flag_cnt != around_num {
			self.table[cursol_index as usize].bold_on();
		}

		// 安全マスの表示オフ、または旗が正しく立てられていない
		// 場合は安全マスは表示しない
		if !is_safe_on || flag_cnt != around_num || miss_cnt > 0 {
			return
		}
		
		// 閉じているマスに安全フラグを立てる
		for index in around.into_iter().flatten() {
			if index != -1 &&
			   !self.table[index as usize].is_open () &&
			   self.table[index as usize].get_userflg() == UserFlg::None {
				self.table[index as usize].set_autoflag(AutoSts::Safety);
			}
		}
	}
/*
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
 */

}
