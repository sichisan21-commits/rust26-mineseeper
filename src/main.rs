mod myconst;
mod txtbox;
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
use myconst::*;

//--------------------------------------------------
// main
//--------------------------------------------------
#[macroquad::main("Test Window")]
async fn main()
	{
	rand::srand(miniquad::date::now() as u64);

	// フォントを読み込む
	let myfont = load_ttf_font("assets/msgothic.ttc").await.unwrap();

	// タイトル画面とゲームメインを初期化する
	let mut title_data = TitleMain::new(&myfont);
	let mut game_data = GameMain::new(&myfont);

	// タイトル画面から始める
	let mut appmode = GameMode::Title;
	loop {
		// 終了が選択された
		if appmode == GameMode::Quit {
			break;
		}

		// タイトル画面
		if appmode == GameMode::Title {
			appmode = title_data.titlecontrol();
			title_data.draw();
			// 状態がゲームに遷移した場合、ゲームを初期化
			if appmode == GameMode::Game {
				let (width, height, bom_num) = title_data.get_setting();
				game_data.set_gameinfo(width, height, bom_num);
				game_data.initial_game(START_WAIT);
			}
		}

		// ゲーム制御
		if appmode == GameMode::Game {
			appmode = game_data.playcontrol();
			game_data.draw();
		}

		// 画面更新
		next_frame().await;
	}
}
