use image::DynamicImage;
use std::fs::File;
fn main() {
    let file_path = "../assets/20170203-_MG_7535.jpg";
    // let now_read_file = Instant::now();
    let file = File::open(file_path).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut read_buffer = Vec::new();

    reader.read_to_end(&mut read_buffer).unwrap();
    println!("{:?}", &reader);

    ////// image
    let img = image::load_from_memory(&read_buffer).unwrap();

    // println!("{:?}", img.dimensions());
    //
    // let exifreader = exif::Reader::new();
    // let exif = exifreader.read_from_container(&mut reader).unwrap();
    // println!("{:?}", exif);
    // for f in exif.fields() {
    // println!(
    // "{} {} {}",
    // f.tag,
    // f.ifd_num,
    // f.display_value().with_unit(&exif)
    // );
    // }
}
