use macroquad::prelude::*;
use crate::draw::*;
use crate::myconst::*;

pub struct ChkBox{
    text: String,
    txtcol: Color,
    left: i32,
    top: i32,
    flg: bool,
}

impl ChkBox {
    pub fn new(text: String, left: i32, top: i32, txtcol: Color, flg: bool) -> ChkBox {
        ChkBox {
            text,
            txtcol,
            left,
            top,
            flg,
        }
    }

    pub fn draw(&self) {
        let check = {
            if self.flg {
                "[*]"
            } else {
                "[-]"
            }
        };
        drawtextln(&format!("{}{}",check, self.text), self.left, self.top, FONT_SIZE, self.txtcol);
    }

    pub fn set_flg(&mut self, flg: bool) {
        self.flg = flg;
    }

}
