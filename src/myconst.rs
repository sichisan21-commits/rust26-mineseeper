use macroquad::prelude::*;

//----------------------------------------
// ゲーム全体
//----------------------------------------
// ウィンドウの最小・最大
pub const WIN_MIN_X: f32 = 800.0;
pub const WIN_MIN_Y: f32 = 600.0;
pub const WIN_MAX_X: f32 = 1280.0;
pub const WIN_MAX_Y: f32 = 780.0;
pub const MIN_ZOOM: f32 = 0.2;
pub const MAX_ZOOMX: f32 = 10.0;
pub const MAX_ZOOMY: f32 = 10.0;

// 位置・サイズ
pub const WALL_LEFT: f32 = 280.0;               // 左の壁
pub const WALL_TOP: f32 = 60.0;                 // 上の壁
pub const WALL_RIGHT: f32 = 20.0;               // 右の壁
pub const WALL_BOTTOM: f32 = 20.0;              // 下の壁
pub const SCROLL_LEFT: f32 = 150.0;				// スクロールを開始する横位置
pub const SCROLL_TOP: f32 = 150.0;				// スクロールを開始する立て位置
pub const FONT_SIZE: f32 = 50.0;                // フォントサイズ
pub const SUB_FONT_SIZE: f32 = 30.0;            // フォントサイズ
// 背景色
pub const LAYOUT_COLOR: Color = Color::from_rgba(220, 220, 220, 255);

//----------------------------------------
// パネル情報
//----------------------------------------
// 位置・サイズ
pub const PANEL_WIDTH: f32 = 25.0 * 2.0;          // 描画幅
pub const PANEL_HEIGHT: f32 = 25.0 * 2.0;         // 描画高さ
pub const PANEL_THICK: f32 = 2.0 * 2.0;           // 厚さ
pub const PANEL_FONT_SIZE: f32 = 25.0 * 2.0;      // フォントサイズ
pub const PANEL_FONT_OFFSX: f32 = 12.0;           // 位置調整
pub const PANEL_FONT_OFFSY: f32 = 37.0;           // 位置調整

// 色
pub const PANEL_COL_SHADOW: Color = Color::from_rgba(80, 80, 128, 255); 
pub const PANEL_COL_CLOSE: Color = Color::from_rgba(180, 180, 180, 255);
pub const PANEL_COL_OPEN: Color = Color::from_rgba(230, 230, 230, 255);
pub const PANEL_COL_DANGER: Color = Color::from_rgba(220, 180, 180, 255);
pub const PANEL_COL_SAFETY: Color = Color::from_rgba(180, 220, 220, 255);

//----------------------------------------
// enum
//----------------------------------------
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ChkBoxTitle {						// タイトル用チェックボックス群
	Easy,									// イージー
	Normal,									// ノーマル
	Hard,									// ハード
	Edit,									// エディット
	Start,									// 開始
	Quit,									// 終了
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ChkBoxGame {						// ゲーム用チェックボックス
	CursolFlg,								// カーソル表示
	UndoFlg,								// UNDO使用
	BoldFlg,								// 確実に旗が立てられるマスの強調
	DangOn, 	        	        	    // 危険マスの表示
	SafeOn,    		        	        	// 安全マスの表示
	DispAll,       		        	        // 前面表示
	Title,									// タイトルへ戻る
}

#[derive(Debug,PartialEq)]
pub enum GameMode {							// ゲームの状態（全体）
	Title,									// タイトル
	Game,									// ゲームスタート（入力待ち）
	Quit,									// ゲーム終了
}

#[derive(PartialEq)]
pub enum GameStat {							// ゲームの状態
	Ready,									// ゲームスタート（入力待ち）
	Playing,								// プレイ中
	Failed,									// ゲームオーバー
	Success,								// ステージクリア
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserFlg {                          // プレイヤーの立てた旗
	None,									// なし
	RedFlg,									// 旗（赤）
	BlueFlg,								// 旗（青）
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoSts {                          // 自動判定フラグ
	None,                                   // なにもなし
	Safety,                                 // 安全マス
	Danger,                                 // 危険マス
	Unknown,                                // 不明
}