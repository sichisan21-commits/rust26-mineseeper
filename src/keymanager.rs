//use std::char;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub struct KeyManager {
    key: char,
}

//------------------------------------------------------------------
// 単語のデータ構造
//------------------------------------------------------------------
impl KeyManager {
    //------------------------------------------------------------------
    // キー管理の初期化
    //------------------------------------------------------------------    
    pub fn new() -> KeyManager {
        KeyManager {
            key: ' ',
        }
    }
    
    //------------------------------------------------------------------
    // キー入力の取得
    //------------------------------------------------------------------
    pub fn get_key(&mut self) -> char {
        if let Some((c, kind)) = self.get_key_event() {
            match kind {
                KeyEventKind::Press => {
                    self.key = c;
                }
                KeyEventKind::Repeat => {
                    self.key = c;
                }
                KeyEventKind::Release => {
                    self.key = ' ';
                }
            }
        }else{
            self.key = ' '; 
        }
        self.key
    }

    //------------------------------------------------------------------
    // キー入力の取得
    //------------------------------------------------------------------
    pub fn get_key_event(&mut self) -> Option<(char, KeyEventKind)> {
        if event::poll(std::time::Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if let KeyCode::Char(c) = key_event.code {
                    return Some((c, key_event.kind));
                }
            }
        }
        None
    }

}
