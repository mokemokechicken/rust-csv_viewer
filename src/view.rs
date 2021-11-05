use crate::model::Model;
use pancurses;
use pancurses::Window;

pub struct View {
    window: Window,
    model: Model,
    top_line: u64,
}

const PADDING_Y: u64 = 5;

impl View {
    pub fn new(model: Model) -> Self {
        let window = pancurses::initscr();
        Self {
            window,
            model,
            top_line: 0,
        }
    }

    pub fn init(&mut self) {
        pancurses::start_color();
        pancurses::use_default_colors();
    }

    pub fn show(&mut self) {
        self.update_view();
    }

    pub fn update_view(&mut self) {
        let (my, mx) = self.window.get_max_yx();
        let lines = self.model.fetch_lines(self.top_line, my as u64 - PADDING_Y);
        let column_width: usize = 20;
        for (iy, line) in lines.iter().enumerate() {
            for (ix, value) in line.iter().enumerate() {
                let st = String::from(value);
                let tx = (ix * column_width) as i32;
                if mx  <= tx {
                    break;
                } else {
                    let ch_len = ((mx - tx) as usize).min(st.len()).min(column_width-1);
                    self.window.mv(iy as i32, tx);
                    self.window.addstr(&st[..ch_len]);
                }
            }
        }
        self.window.refresh();
    }
}
