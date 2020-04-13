use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Read;

fn main() {
    let data_path = Path::new("data/HRSz0yd020fm_c2f.fits");
    let mut file = match File::open(data_path) {
        Err(e) => panic!("Couldn't open {}: {}", data_path.display(), e.description()),
        Ok(file) => file,
    };
    let mut fits_data:Vec<u8> = Vec::new();
    file.read_to_end(&mut fits_data);
}
