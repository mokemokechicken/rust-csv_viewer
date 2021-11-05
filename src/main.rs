mod view;
mod model;

use memmap::MmapOptions;
use std::{env};
use std::fs::File;
use std::io::{Cursor, Error};
use csv::{Reader, Position, ReaderBuilder};
use crate::model::Model;
use crate::view::View;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("please specify csv file.");
        return Ok(());
    }

    let filename = args.get(1).unwrap();
    let mut model = Model::new(filename);
    model.scan_index();

    let mut view = View::new(model);
    view.init();
    view.show();
    Ok(())
}
