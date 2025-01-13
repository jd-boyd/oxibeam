use tempfile::NamedTempFile;

mod imgfile;
mod vec3;
mod ray;

fn main() {
    println!("Hello, world!");

    let temp_file = NamedTempFile::new().unwrap();
    let rgb_data = vec![255, 0, 0, 0, 255, 0]; // 2x1 image: red and green pixels

    let result = imgfile::write_rgb_to_png(temp_file.path(), 2, 1, &rgb_data);
    assert!(result.is_ok());
}
