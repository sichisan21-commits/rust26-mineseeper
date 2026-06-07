use macroquad::prelude::*;
use crate::draw::*;

#[derive(Copy, Clone, PartialEq)]
pub enum ChkBoxType {
	CursolFlg,								// カーソル表示
	UndoFlg,								// UNDO使用
	BoldFlg,								// 確実に旗が立てられるマスの強調
	DangOn, 	        	        	    // 危険マスの表示
	SafeOn,    	        	        	// 安全マスの表示
	DispAll,       	        	        // 前面表示
}

pub struct ChkBox{
	mytype: ChkBoxType,						// チェックボックスのタイプ
    text: String,
    left: f32,
    top: f32,
    size: f32,
    fgcol: (u8,u8,u8,u8),
    bgcol: (u8,u8,u8,u8),
    flg: bool,
}

//--------------------------------------------------
// 実装
//--------------------------------------------------
impl ChkBox {

    //--------------------------------------------------
    // 初期化
    //--------------------------------------------------
    pub fn new(mytype: ChkBoxType, text: String, left: f32, top: f32, size: f32,
        fgcol: (u8,u8,u8,u8),bgcol: (u8,u8,u8,u8), flg: bool) -> ChkBox {
        ChkBox {
            mytype,
            text,
            left,
            top,
            size,
            fgcol,
            bgcol,
            flg,
        }
    }

    //--------------------------------------------------
    // チェックボックスをクリック（座標が一致していれば）
    //--------------------------------------------------
    pub fn click(&mut self, mouse_x:f32, mouse_y: f32) -> bool {
        let right = self.left + self.size as f32 * self.text.len() as f32 * 0.7;
        let bottom = self.top + self.size as f32;
        if mouse_x >= self.left && mouse_x <= right &&
           mouse_y >= self.top && mouse_y <= bottom {
            self.flg ^= true;
            return true
        }
        false
    }

    //--------------------------------------------------
    // 描画
    //--------------------------------------------------
    pub fn draw(&self) {
        let check = {
            if self.flg {
                "[*]"
            } else {
                "[-]"
            }
        };
        dr_text(&format!("{}{}",check, self.text),
            self.left, self.top, self.size, self.fgcol, self.bgcol);
    }

    //
    pub fn get_type(&self) -> ChkBoxType {
        self.mytype
    }

    //
    pub fn get_flg(&self) -> bool {
        self.flg
    }

    //--------------------------------------------------
    // オン／オフ
    //--------------------------------------------------
    pub fn set_flg(&mut self, flg: bool) {
        self.flg = flg;
    }

}
