use std::fs::File;
use std::path::PathBuf;
extern crate obj_exporter as obj;

use bch3ds::bch_to_obj;
use bch3ds::BCH;

fn main() {
    env_logger::init();
    let file_path = PathBuf::from("./tw02_cafe.bch");
    let mut file = File::open(file_path).unwrap();

    let bch = BCH::read(&mut file).unwrap();
    obj::export_to_file(&bch_to_obj(&bch), "output_single.obj").unwrap();
}
