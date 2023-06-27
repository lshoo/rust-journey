use effective::read_image_dimension;

fn main() {
    let img_path = "data/panda.png";
    match read_image_dimension::get_image_dimension(img_path) {
        Ok((w, h)) => println!("dimensions: {} x {}", w, h),
        Err(e) => println!("error: {}", e),
    }
}
