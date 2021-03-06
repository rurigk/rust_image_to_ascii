extern crate image;

use core::f32;
use std::{fs::{self}, io::{self}};
use image::{GenericImageView};
use std::path::Path;

fn main() -> io::Result<()>{
    let files = fs::read_dir("images").unwrap();
    for file in files {
        let file_path = file.unwrap().path().display().to_string();
        let path = Path::new(&file_path);

        let file_name = path.file_stem().unwrap().to_str().unwrap();

        process_image(file_path.as_str(), &("output/".to_owned() + file_name + ".txt"));
    }

    Ok(())
}

fn process_image(file:&str, output_file:&str) 
{

    let img = image::open(file).unwrap();
    let dimensions = img.dimensions();

    //let ascii_scale = " .:-=+*#%@";
    let ascii_scale = " .:-=+░▒▓▓";

    let width = dimensions.0;
    let height = dimensions.1;
    let bytes_per_pixel = 4;

    let buffer: Vec<u8> = img.to_rgba8().to_vec();

    println!("Processing [{}], Total pixels: {}", file, (buffer.len() / bytes_per_pixel) - 1);
    let mut output_buffer = String::new();

    for y in 0..height {
        for x in 0..width {

            let pixel = img.get_pixel(x, y);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];

            let avg = 255_f32 - ((r as f32 + g as f32 + b as f32) * 0.3333_f32);
            let luminance_to_index = ((ascii_scale.chars().count() as f32 - 1_f32) * (avg as f32/ 255 as f32)).round();
            
            output_buffer.push(ascii_scale.chars().nth(luminance_to_index as usize).unwrap());
            output_buffer.push(ascii_scale.chars().nth(luminance_to_index as usize).unwrap());
        }
        output_buffer.push('\n');
    }

    fs::write(output_file, output_buffer).expect("Error writing file");
}