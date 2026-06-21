mod setting;
mod myconst;
mod txtbox;
mod chkboxmng;
mod chkbox;
mod titlemain;
mod gamemain;
mod gametable;
mod panel;
mod inference;
mod utils;
mod draw;

use macroquad::prelude::*;
use gamemain::GameMain;
use titlemain::TitleMain;
use setting::GameSettings;
use myconst::*;
use utils::*;

// 設定画面オブジェクトをゲーム内で共有する
use std::rc::Rc;
use std::cell::RefCell;
pub type SharedSettings = Rc<RefCell<GameSettings>>;

//--------------------------------------------------
// main
//--------------------------------------------------
#[macroquad::main("Let's MINE SWEEPER")]
async fn main()
	{
	// ライブラリの初期化
	rand::srand(miniquad::date::now() as u64);

	// フォントを読み込む
	let myfont = load_ttf_font("assets/msgothic.ttc").await.unwrap();

	// 設定画面を初期化する
	let my_setting = Rc::new(RefCell::new(GameSettings::new()));
	my_setting.borrow_mut().set_help_pos(PosTable{x:30.0,y:10.0});
	my_setting.borrow_mut().set_menu_pos(PosTable{x:80.0,y:100.0});

	// タイトル画面とゲームメインを初期化する
	let mut title_data = TitleMain::new();
	title_data.setting_obj(my_setting.clone());

	let mut game_data = GameMain::new();
	game_data.setting_obj(my_setting.clone());

	// タイトル画面から始める
	let mut appmode = GameMode::Title;
	loop {

		//--------------------------------------------------
		// タイトル画面
		//--------------------------------------------------
		if appmode == GameMode::Title {
			// タイトル画面の操作
			appmode = title_data.titlecontrol();

			// タイトル画面の描画
			title_data.draw(&myfont);

			// 終了が選択されたらループを抜ける
			if appmode == GameMode::Quit {
				break;
			}

			// 状態が「ゲーム」に遷移した場合、ゲームを初期化
			if appmode == GameMode::Game {
				let (width, height, bom_num) = title_data.get_setting();
				game_data.set_gameinfo(width, height, bom_num);
				game_data.initial_game(START_WAIT);
			}
		}

		//--------------------------------------------------
		// ゲーム画面
		//--------------------------------------------------
		if appmode == GameMode::Game {
			// ゲーム画面の操作
			appmode = game_data.playcontrol();

			// ゲーム画面の描画
			game_data.draw(&myfont);
		}

		// 画面更新
		next_frame().await;
	}
}
