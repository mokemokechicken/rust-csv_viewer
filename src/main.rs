mod view;
mod model;

use std::env;
use crate::model::Model;
use crate::view::View;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("please specify csv file.");
        return;
    }

    let filename = args.get(1).unwrap();
    let mut model = Model::new(filename);
    model.scan_index();

    let mut view = View::new(model);
    view.init();
    view.show();
}
