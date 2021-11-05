use std::fs::File;
use std::io::Cursor;
use csv::{Position, Reader, ReaderBuilder, StringRecord};
use memmap::{Mmap, MmapOptions};

pub struct Model {
    reader: Reader<Cursor<Mmap>>,
    index_step: u64,
    byte_index_list: Vec<u64>,
}

impl Model {
    pub fn new(filename: &str) -> Self {
        let file = File::open(filename).expect(&format!("cannot open file: {}", filename));
        let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
        let reader = ReaderBuilder::new().has_headers(false).from_reader(Cursor::new(mmap));
        Self {
            reader,
            index_step: 10,
            byte_index_list: Vec::new(),
        }
    }

    pub fn scan_index(&mut self) {
        let mut byte_index_list: Vec<u64> = Vec::new();
        let r = &mut self.reader;
        let mut iter = r.records();
        byte_index_list.push(0);
        loop {
            let pos = iter.reader().position();
            if pos.line() % self.index_step == 0 {
                byte_index_list.push(pos.byte());
            }
            if iter.next().is_none() {
                break;
            }
        }
        self.byte_index_list = byte_index_list;
    }

    pub fn fetch_lines(&mut self, start_line: u64, n_lines: u64) -> Vec<StringRecord> {
        // start line with 0
        let idx = ((start_line / self.index_step) as f64).floor() as u64;
        {
            let mut pos = Position::new();
            if (self.byte_index_list.len() as u64) <= idx {
                return Vec::new();
            }
            pos.set_byte(self.byte_index_list[idx as usize]);
            self.reader.seek(pos).unwrap();
        }

        let mut lines: Vec<StringRecord> = Vec::new();
        let mut cur_line_no = idx * self.index_step;
        for rec in self.reader.records() {
            if start_line + n_lines <= cur_line_no {
                break;
            }
            if start_line <= cur_line_no {
                if let Ok(r) = rec {
                    lines.push(r);
                }
            }
            cur_line_no += 1;
        }
        lines
    }
}
