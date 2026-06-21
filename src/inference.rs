use crate::panel::Panel;
use crate::myconst::*;

// 推論テーブル
struct Inference {
	myindex: i32,
	index: Vec<i32>,
	bomnum: i32,
}

pub struct InfTable {						// 推論処理用データ
	table: Vec<Panel>,                      // 盤面データコピー
	width: i32,								// 盤面の幅
	height: i32,							// 盤面の高さ
	infe: Vec<Inference>,					// 推論テーブル
	believe_flg: bool,						// ユーザの立てた旗を正しいと仮定
}

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl InfTable {
	//------------------------------
	// 初期化
	// table＝処理対象のテーブル（clone）
	// width＝盤面の幅
	// height＝盤面の高さ
	//------------------------------
	pub fn new(table:Vec<Panel>, width:i32, height:i32) -> InfTable {
		InfTable {
			table,
			width,
			height,
			infe: Vec::new(),
			believe_flg: false,
		}
	}

	//------------------------------
	// 加工後のテーブルを返却する
	//------------------------------
	pub fn get_table(&mut self) -> Vec<Panel> {
		self.table.clone()
	}

	//##################################################
	// 【強調処理】
	// 旗が立てられる可能性のあるマスの強調表示
	// is_safe_on＝安全マスの表示フラグ
	//##################################################
	pub fn set_bold(&mut self, is_safe_on:bool) {
		// 一旦すべてのフラグを落とす
		for index in 0..self.width * self.height {
			self.table[index as usize].set_autoflag(AutoSts::None, false);
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
	// is_safe_on＝安全マスの表示フラグ
	// cursol_index＝カーソル位置の配列番号
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
			if self.table[index as usize].get_userflg() == UserFlg::RedFlg {
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
			   self.table[index as usize].get_userflg() != UserFlg::RedFlg {
				self.table[index as usize].set_autoflag(AutoSts::Safety, self.believe_flg);
			}
		}
	}

	//------------------------------
	// 周囲に「危険」と判断できるマスがあるならフラグを立てる
	//------------------------------
	fn flag_dangar_one(&mut self, cursol_index: i32) -> bool {
		let mut is_update = false;

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
				self.table[index as usize].set_autoflag(AutoSts::Danger, self.believe_flg);
				is_update = true;
			}
		}
		is_update
	}

	//------------------------------
	// 周囲に「安全」と判断できるマスがあるならフラグを立てる
	//------------------------------
	fn flag_safety_one(&mut self, cursol_index: i32) -> bool {
		let mut is_update = false;

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
			// 有効範囲外はまたは開いているパネルはスキップ
			if index == -1 || self.table[index as usize].is_open() {
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
				self.table[index as usize].set_autoflag(AutoSts::Safety, self.believe_flg);
				is_update = true;
			}
		}
		is_update
	}

	//##################################################
	// 【危険マス・安全マス設定】
	//##################################################
	//------------------------------
	// 自動的に判別し危険マス／安全マスにフラグを立てる
	// is_dang_on＝危険マスの表示フラグ
	// is_safe_on＝安全マスの表示フラグ
	// believe_flg＝ユーザの立てた旗を信じて推論を行うか
	//------------------------------
	pub fn inference(&mut self, _is_dang_on: bool, _is_safe_on: bool, believe_flg: bool) {
//		println!("exec inference");
		// 全てのアシスト表示をオフ
		for index in 0..(self.width*self.height) as usize {
			self.table[index].bold_off();
			self.table[index].set_autoflag(AutoSts::None, false);
			self.table[index].set_infe_flg(false);
		}

		// 推論ループ（無限ループを考慮して最大１０回）
		for _ in 0..10 {
			let mut is_update = false;

			// まず単純な推論を実施
			for _ in 0..10 {
				if !self.inf_simple() {
					break;
				}
				is_update = true;
			}

			// 高度推論を行う
			for _ in 0..10 {
				if !self.inf_deep() {
					break;
				}
				is_update = true;
			}

			// 更新がなければループを抜ける
			if !is_update {
				break;
			}
		}

		// ユーザの立てた旗を信じないならここまで
		if !believe_flg {
			return
		}

		// ユーザの立てた旗を信じる
		self.believe_flg = believe_flg;
		for _ in 0..10 {
			// まず単純な推論を実施
			for _ in 0..10 {
				if !self.inf_simple() {
					break;
				}
			}

			// 高度推論を行う
			for _ in 0..10 {
				let is_update = self.inf_deep();
				if !is_update {
					break;
				}
			}
		}

	}
 
	//------------------------------
	// シンプルな危険・安全判定
	// 単一マスから危険・安全を判断
	//------------------------------
	pub fn inf_simple(&mut self) -> bool {
		let mut is_update = false;

		// ユーザの立てた旗を信じる場合
		if self.believe_flg {
			// 旗の立っているマスは危険マスとする
			for index in 0..self.width * self.height {
				if self.table[index as usize].get_userflg() == UserFlg::RedFlg {
					self.table[index as usize].set_autoflag(AutoSts::Danger, self.believe_flg);
				}
			}
		}

		// 危険マスを判定
		for index in 0..self.width * self.height {
			is_update |= self.flag_dangar_one(index);
		}

		// 安全マスを判定
		for index in 0..self.width * self.height {
			is_update |= self.flag_safety_one(index);
		}

		is_update
	}
	
	//------------------------------
	// 高度な危険・安全判定
	//------------------------------
	pub fn inf_deep(&mut self) -> bool {
		self.infe.clear();
		self.make_inference();
		self.inference_check()
	}

	//##################################################
	// 【高度推論】
	//  「複数マスに一つ爆弾」といった可能性から推測する
	//##################################################
	//------------------------------
	// まず「この範囲に爆弾が含まれるはず」というリストを作成する
	//------------------------------
	fn make_inference(&mut self) {
		for index in 0..self.width * self.height {
			if self.table[index as usize].is_open() {
				self.make_inference_one(index);
			}
		}
	}

	//------------------------------
	// 一マスに対して推論リストを作成
	//------------------------------
	fn make_inference_one(&mut self, cursol_index: i32) {
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
			} else if auto_flag == AutoSts::None {
				// 未開封パネルの保持
				close_list.push(index);
			}
		}

		// 見つかっていない爆弾数を求める
		let bomnum = self.table[cursol_index as usize].get_around_num() - dang_cnt;

		// 見つかっていない爆弾があるなら
		if bomnum > 0 {
			// 「このマスたち」には「爆弾がｎ個埋まっている」という情報を保持する
			self.infe.push(
				Inference {
					myindex: cursol_index,
					index: close_list,
					bomnum: bomnum,
				}
			)
		}
	}

	//------------------------------
	// 作成した推論リストと照合して判定する（全体ループ）
	//------------------------------
	fn inference_check(&mut self) -> bool {
		let mut is_update = false;
		for index in 0..self.width * self.height {
			if self.table[index as usize].is_open() {
				is_update |= self.inference_one(index);
			}
		}
		is_update
	}

	//------------------------------
	// 一マスずつ推論テーブルを検証
	//------------------------------
	fn inference_one(&mut self, cursol_index: i32) -> bool {
		let mut is_update = false;

		//--------------------------------------------------
		// 周囲９マスの中から、未確定のマスだけリスト化する
		//--------------------------------------------------
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
				AutoSts::None => {
					myclose_index.push(index);
				}
				_ => {}
			}
		}

		// 周りに未確定のマスがない、またはすべての爆弾は確定している
		if myclose_index.len() == 0 || around_num == 0{
			return is_update;
		}

		//--------------------------------------------------
		// 推論テーブルでループする
		//--------------------------------------------------
		for inference in &self.infe {
			// 自分の推論テーブルとは比較しない
			if inference.myindex == cursol_index {
				continue;
			}
			
			// 周囲の未確定パネルから、推論テーブルと一致するマスを除いた残りを作る
			let remain: Vec<i32> = myclose_index
				.iter()
				.cloned()
				.filter(|idx| !inference.index.contains(idx))
				.collect();
			if remain.len() == 0 {
				continue;
			}

			// その結果未確定パネルが残らなかった、またはまったく掠っていなかった（離れている）
			// ならスキップ
			let remain_len = remain.len();
			if remain_len == 0 || remain_len == myclose_index.len(){
				continue;
			}

			// 未確定パネルと推論テーブルの重複数を求める
			let overnum = (myclose_index.len() - remain_len) as i32;
			// 重複しなかった数を求める
			let outnum = inference.index.len() as i32 - overnum;
			// 重複内に必ず含まれる爆弾数を求める
			let inf_bommin = inference.bomnum - outnum;
/*
			println!("座標が一致した:{}({}/{}),{}({}/{}),outnum={}",
				cursol_index,around_num,myclose_index.len(),
				inference.myindex,inference.bomnum,inference.index.len(),
				outnum);
 */

			// 残爆弾数が一致した場合、重複していないマスは安全
			if around_num == inf_bommin {
				is_update = true;
				for index in remain {
					self.table[index as usize].set_autoflag(AutoSts::Safety, self.believe_flg);
					self.table[index as usize].set_infe_flg(true);
//					if self.table[index as usize].is_bom() {
//						println!("安全ではないマスを安全と判定した={}", index);
//					}
				}
			} else if outnum == 0 && around_num - inference.bomnum == remain.len() as i32 {
				// 推論テーブルがカーソルの周囲に完全一致しており（outnum == 0）
				// 残ったマスの数と差し引き爆弾数が同じ場合は危険
				is_update = true;
				let remain_len = remain.len();
				for index in remain {
					self.table[index as usize].set_autoflag(AutoSts::Danger, self.believe_flg);
					self.table[index as usize].set_infe_flg(true);
					if !self.table[index as usize].is_bom() {
//						println!("危険ではないマスを危険と判定した:{},around={},tgt={},infbom={},remainlen={}",
//							cursol_index, around_num, index, inference.bomnum, remain_len);
//					}
				}
			}
		}

		is_update
	}
}
