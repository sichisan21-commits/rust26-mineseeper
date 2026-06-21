use macroquad::prelude::*;
use crate::chkboxmng::ChkBoxMng;
use crate::myconst::*;
use crate::draw::*;
use crate::utils::*;

pub struct GameSettings {
	chkbox: ChkBoxMng<CBSetting>,		// 自作チェックボックス
	mouse_pos: PosTable,				// マウス位置
	menu_pos: PosTable,					// メニュー表示位置
	help_pos: PosTable,					// ヘルプ表示位置
}

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl GameSettings {

	//------------------------------
	// 初期化
	//------------------------------
	pub fn new () -> GameSettings {
		let mut gs = GameSettings {
			chkbox: ChkBoxMng::new(),
			mouse_pos: PosTable {x:0.0, y:0.0},
			menu_pos: PosTable {x:80.0, y:100.0},
			help_pos: PosTable {x:30.0, y:10.0},
		};

		// チェックボックス作成
		gs.create_chkbox();

		gs
	}

	//------------------------------
	// チェックボックス作成
	//------------------------------
	fn create_chkbox(&mut self) {
		let left = self.menu_pos.x + 40.0;
		let top = self.menu_pos.y + 50.0;

		// チェックボックス作成
		self.chkbox.set_base(left,top,200.0, 35.0, 25.0,"000000FF", "FFFFFFFF");
		// 各種設定
		self.chkbox.add(CBSetting::CursolFlg, String::from("CURSOL FLAME"), false);
		self.chkbox.add(CBSetting::DragOpen, String::from("DRAG OPEN"), false);
		self.chkbox.add(CBSetting::UseBlueFlg, String::from("USE BLUEFLAG"), false);
		// 盤面を隠すモード
		self.chkbox.set_next_pos(PosTable{x:0.0, y:200.0});
		self.chkbox.add(CBSetting::HideOn, String::from("HIDE BORD"), false);
		self.chkbox.add(CBSetting::HideNumOn, String::from("HIDE NUMBER"), false);
		// アシスト機能
		self.chkbox.set_next_pos(PosTable{x:310.0, y:0.0});
		self.chkbox.add(CBSetting::BoldFlg, String::from("USE BOLD"), false);
		self.chkbox.add(CBSetting::Inference, String::from("USE INFERENCE"), false);
		self.chkbox.add(CBSetting::UndoFlg, String::from("USE UNDO"), false);
		self.chkbox.add(CBSetting::DispAll,String::from("All DISPLAY"), false);
		// 確定（閉じる）ボタン（絶対位置）
		self.chkbox.set_next_pos(PosTable{x:460.0, y:350.0});
		self.chkbox.add(CBSetting::Close, String::from("[CLOSE]"), true);
		self.chkbox.set_col(CBSetting::Close, "3333FFFF","");
		self.chkbox.view_box(CBSetting::Close, false);
		
		// 子のチェックボックス作成
        self.chkbox.set_base(left,top,200.0, 30.0, 20.0,"000000FF", "FFFFFFFF");
		self.chkbox.addsub(CBSetting::BlueFlgFirst, CBSetting::UseBlueFlg,String::from("BLUEFLAG FIRST"), false);
		self.chkbox.addsub(CBSetting::BoldSafeOn, CBSetting::BoldFlg,String::from("SAFETY ON"), false);
		self.chkbox.addsub(CBSetting::SafeOn,CBSetting::Inference, String::from("SAFETY ON"), false);
		self.chkbox.addsub(CBSetting::DangOn,CBSetting::Inference, String::from("DANGER ON"), true);
		self.chkbox.addsub(CBSetting::BelieveFlag,CBSetting::Inference, String::from("BELEVE FLAG"), false);
		self.chkbox.addsub(CBSetting::HideLv1, CBSetting::HideOn,String::from("LEVEL1"), true);
		self.chkbox.addsub(CBSetting::HideLv2, CBSetting::HideOn,String::from("LEVEL2"), false);
		self.chkbox.addsub(CBSetting::HideLv3, CBSetting::HideOn,String::from("LEVEL3"), false);
		self.chkbox.addsub(CBSetting::HideNumLv1, CBSetting::HideNumOn,String::from("LEVEL1"), true);
		self.chkbox.addsub(CBSetting::HideNumLv2, CBSetting::HideNumOn,String::from("LEVEL2"), false);
		self.chkbox.addsub(CBSetting::HideNumLv3, CBSetting::HideNumOn,String::from("LEVEL3"), false);

		// 推論の全面表示は隠しておく
		self.chkbox.set_active_flg(CBSetting::DispAll, false);

		self.chkbox.view_hitbox(false);

		// 説明文追加
		self.chkbox.set_help(CBSetting::CursolFlg,"[CURSOL FLAME]\n３×３のカーソルを表示します。");
		self.chkbox.set_help(CBSetting::DragOpen,"[DRAG OPEN]\n押しっぱなしでまとめてパネルを開きます。");
		self.chkbox.set_help(CBSetting::UseBlueFlg,"[USE BLUEFLAG]\n「青色の旗」を使用します、赤色の旗と区別したい場合に使用してください。");
		self.chkbox.set_help(CBSetting::BlueFlgFirst,"[BLUEFLAG FIRST]\n旗を立てる順番を「青→赤→なし」へ変更します。");
		self.chkbox.set_help(CBSetting::BoldFlg,"[USE BOLD] ※初心者にお勧め\n「数字」と「周りの未開封パネル数」が一致していると強調表示されます。\n（正しく旗を立てると強調表示は消えます）");
		self.chkbox.set_help(CBSetting::BoldSafeOn,"[SAFETY ON]\n旗の周囲の安全パネルを表示します。");
		self.chkbox.set_help(CBSetting::Inference,"[USE INFERENCE]\n見えている数字から、危険／安全パネルを推測します。");
		self.chkbox.set_help(CBSetting::DangOn,"[DANGER ON]\n推論で危険パネルを表示します。");
		self.chkbox.set_help(CBSetting::SafeOn,"[SAFETY ON]\n推論で安全パネルを表示します。");
		self.chkbox.set_help(CBSetting::DispAll,"[ALL DISPLAY]\n盤面全体に危険／安全パネルを表示します。\n（デフォルトはマウスの隣接のみ表示）");
		self.chkbox.set_help(CBSetting::BelieveFlag,"[BELIEVE FLAG]\nあなたの立てた旗を信じて推論します。");
		self.chkbox.set_help(CBSetting::UndoFlg,"[USE UNDO]\nUNDO（やり直し）を有効にします。");
		self.chkbox.set_help(CBSetting::HideOn,"[HIDE BORD]（アシスト使用不可）\nマウスの周囲だけ盤面を表示します。※タイマー起動中は変更できません。");
		self.chkbox.set_help(CBSetting::HideLv1,"[LEVEL1]\n周囲５×５まで表示します。");
		self.chkbox.set_help(CBSetting::HideLv2,"[LEVEL2]\n周囲３×３まで表示します。");
		self.chkbox.set_help(CBSetting::HideLv3,"[LEVEL3]\nマウス位置以外は表示されません");
		self.chkbox.set_help(CBSetting::HideNumOn,"[HIDE NUMBER]（アシスト使用不可）\nマウスの周囲だけ数字を表示します。※タイマー起動中は変更できません。");
		self.chkbox.set_help(CBSetting::HideNumLv1,"[LEVEL1]\n周囲５×５まで表示します。");
		self.chkbox.set_help(CBSetting::HideNumLv2,"[LEVEL2]\n周囲３×３まで表示します。");
		self.chkbox.set_help(CBSetting::HideNumLv3,"[LEVEL3]\nマウス位置以外は表示されません");
	}

	//--------------------------------------------------
	// メニューの表示位置設定
	//--------------------------------------------------
	pub fn set_menu_pos(&mut self, pos:PosTable) {
		self.menu_pos = pos;
	}

	//--------------------------------------------------
	// メニューの表示位置設定
	//--------------------------------------------------
	pub fn set_help_pos(&mut self, pos:PosTable) {
		self.help_pos = pos;
	}

	//--------------------------------------------------
	// チェックボックスのクリック処理
	//--------------------------------------------------
	pub fn set_mouse_pos(&mut self, pos:PosTable) {
		self.mouse_pos.x = pos.x;
		self.mouse_pos.y = pos.y;
	}

	//--------------------------------------------------
	// プレイを開始した場合特定のチェックボックスをロックする
	//--------------------------------------------------
	pub fn set_playing_flg(&mut self, flg:bool) {
		self.chkbox.set_lock_flg(CBSetting::HideOn, flg);
		self.chkbox.set_lock_flg(CBSetting::HideNumOn, flg);
	}

	//--------------------------------------------------
	// チェックボックスのクリック処理
	//--------------------------------------------------
	pub fn click(&mut self) {
		// 閉じられている場合は表示しない
		if self.chkbox.get_flg(CBSetting::Close) {
			return
		}

		// クリックされていない場合は何もしない
		if !is_mouse_button_pressed(MouseButton::Left) {
			return
		}

		// チェックボックスのクリック処理
		if let Some((kind, flg)) =
			self.chkbox.click(self.mouse_pos) {
			match kind {

				// 強調フラグが選択された場合
				CBSetting::BoldFlg => {
					if flg {
						// 推論フラグをオフにする
						self.chkbox.set_flg(CBSetting::CursolFlg, true);
						self.chkbox.set_flg(CBSetting::Inference, false);
					}
				}

				// 推論フラグが選択された場合
				CBSetting::Inference => {
					if flg {
						// 強調フラグをオフにする
						self.chkbox.set_flg(CBSetting::CursolFlg, true);
						self.chkbox.set_flg(CBSetting::BoldFlg, false);
					}
				}

				// 推論全表示が選択された場合
				CBSetting::DispAll => {
					// オンの場合安全マス危険マス全部表示
					if flg {
						self.chkbox.set_flg(CBSetting::SafeOn, true);
						self.chkbox.set_flg(CBSetting::DangOn, true);
					}
				}

				// HIDE BORD が選択された
				CBSetting::HideOn => {
					self.chkbox.set_flg(CBSetting::HideNumOn, false);
				}

				// HIDE LVEL1 が選択された
				CBSetting::HideLv1 => {
					self.chkbox.set_flg(CBSetting::HideLv1, true);
					self.chkbox.set_flg(CBSetting::HideLv2, false);
					self.chkbox.set_flg(CBSetting::HideLv3, false);
				}

				// HIDE LVEL2 が選択された
				CBSetting::HideLv2 => {
					self.chkbox.set_flg(CBSetting::HideLv1, false);
					self.chkbox.set_flg(CBSetting::HideLv2, true);
					self.chkbox.set_flg(CBSetting::HideLv3, false);
				}

				// HIDE LVEL3 が選択された
				CBSetting::HideLv3 => {
					self.chkbox.set_flg(CBSetting::HideLv1, false);
					self.chkbox.set_flg(CBSetting::HideLv2, false);
					self.chkbox.set_flg(CBSetting::HideLv3, true);
				}

				// HIDE NUMBER が選択された
				CBSetting::HideNumOn => {
					self.chkbox.set_flg(CBSetting::HideOn, false);
				}

				// HIDE NUM LVEL1 が選択された
				CBSetting::HideNumLv1 => {
					self.chkbox.set_flg(CBSetting::HideNumLv1, true);
					self.chkbox.set_flg(CBSetting::HideNumLv2, false);
					self.chkbox.set_flg(CBSetting::HideNumLv3, false);
				}

				// HIDE NUM LVEL2 が選択された
				CBSetting::HideNumLv2 => {
					self.chkbox.set_flg(CBSetting::HideNumLv1, false);
					self.chkbox.set_flg(CBSetting::HideNumLv2, true);
					self.chkbox.set_flg(CBSetting::HideNumLv3, false);
				}

				// HIDE LNUM VEL3 が選択された
				CBSetting::HideNumLv3 => {
					self.chkbox.set_flg(CBSetting::HideNumLv1, false);
					self.chkbox.set_flg(CBSetting::HideNumLv2, false);
					self.chkbox.set_flg(CBSetting::HideNumLv3, true);
				}

				// それ以外は何もしない
				_ => {}
			}

			// 盤面隠し、番号隠しが有効ならサポートをオフにする
			if self.chkbox.get_flg(CBSetting::HideOn) ||
			   self.chkbox.get_flg(CBSetting::HideNumOn) {
					self.chkbox.set_flg(CBSetting::BoldFlg, false);
					self.chkbox.set_flg(CBSetting::Inference, false);
					self.chkbox.set_flg(CBSetting::UndoFlg, false);
					self.chkbox.set_lock_flg(CBSetting::BoldFlg, true);
					self.chkbox.set_lock_flg(CBSetting::Inference, true);
					self.chkbox.set_lock_flg(CBSetting::UndoFlg, true);
			} else {
					self.chkbox.set_lock_flg(CBSetting::BoldFlg, false);
					self.chkbox.set_lock_flg(CBSetting::Inference, false);
					self.chkbox.set_lock_flg(CBSetting::UndoFlg, false);
			}
		}
	}

	//--------------------------------------------------
	// 設定画面を開く
	//--------------------------------------------------
	pub fn open(&mut self) {
		self.chkbox.set_flg(CBSetting::Close, false);
	}

	//--------------------------------------------------
	// 設定画面を開いているか
	//--------------------------------------------------
	pub fn is_open(&self) -> bool {
		!self.chkbox.get_flg(CBSetting::Close)
	}

	//--------------------------------------------------
	// 設定画面を開いているか
	//--------------------------------------------------
	pub fn get_flg(&self, mytype:CBSetting) -> bool {
		self.chkbox.get_flg(mytype)
	}

	//------------------------------
	// 描画
	//------------------------------
	pub fn draw(&self, myfont: &Font) {
		// 閉じられている場合は表示しない
		if self.chkbox.get_flg(CBSetting::Close) {
			return
		}
		dr_rect(0.0,0.0,screen_width(),screen_height(),
			0.0, "00000090","");

		//--------------------------------------------------
		// メニューの描画
		//--------------------------------------------------
		// カメラをセット
		// 設定画面表示
		dr_rect(self.menu_pos.x,self.menu_pos.y,600.0,450.0,
			3.0,"00000099","FF0000FF");
		dr_text_ex("-- SETTING --", self.menu_pos.x + 30.0, self.menu_pos.y + 10.0,
			25.0, "A0A0FFFF", "000000FF", myfont);
		dr_text_ex("-- GAME MODE --", self.menu_pos.x + 30.0, self.menu_pos.y + 210.0,
			25.0, "A0A0FFFF", "000000FF", myfont);
		dr_text_ex("-- ASSIST --", self.menu_pos.x + 340.0, self.menu_pos.y + 10.0,
			25.0, "A0A0FFFF", "000000FF", myfont);
		self.chkbox.draw(myfont);

		//--------------------------------------------------
		// ヘルプの描画
		//--------------------------------------------------
		self.draw_help(myfont);

	}

	//------------------------------
	// 盤面の描画
	//------------------------------
	fn draw_help(&self, myfont:&Font) {
		if let Some((_typ, help_lines)) =
		   self.chkbox.gethelp(self.mouse_pos) {
			// ヘルプが設定されていない場合何もしない
			if help_lines.len() == 0 {
				return;
			}
			// ヘルプ周囲を塗りつぶす
			let fontsize = 20.0;
			let offs = 5.0;
			let left = self.help_pos.x;
			let top = self.help_pos.y;
			let height = help_lines.len() as f32 * (fontsize + offs);
			dr_rect(left - 5.0, top - 5.0,
				750.0 + 10.0, height + 10.0, 0.0,
				"000000AA", "");
			// ヘルプテキストを表示する
			for (i, line) in help_lines.iter().enumerate() {
				dr_text_ex(line, left, top + i as f32 * (fontsize + offs),
					fontsize, "FFFFFFFF", "005500FF", myfont);
			}
		}
	}

}