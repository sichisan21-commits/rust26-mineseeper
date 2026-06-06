use macroquad::prelude::*;

// 座標データ
pub struct PosTable {
    pub x: f32,
    pub y: f32,
}

//------------------------------
// 座標をインデックスへ変換
//------------------------------
pub fn get_index(cursol_x:i32, cursol_y:i32, width:i32, height:i32) -> i32 {
    if cursol_x < 0 || cursol_x >= width ||
       cursol_y < 0 || cursol_y >= height {
        return -1;
       }
    cursol_y * width + cursol_x
}
