extern crate image;

use core::f32;
use std::{fs::{self}, str::FromStr};
use image::{GenericImageView};
use std::path::Path;

#[tokio::main]
async fn main(){
    //let mut futs = FuturesUnordered::new();

    let mut futures = vec![];

    let files = fs::read_dir("images").unwrap();
    for file in files {
        let file_path = file.unwrap().path().display().to_string().to_owned();
        let path = Path::new(&file_path);

        let file_name = path.file_stem().unwrap().to_str().unwrap();
        match path.extension() {
            None => {
                println!("Invalid file {}", file_path);
            }
            Some(_) => {
                let input_file = String::from_str(file_path.as_str()).unwrap();

                let mut output_file = String::new();
                output_file.push_str("output/");
                output_file.push_str(file_name);
                output_file.push_str(".txt");

                println!("Queue add {}", file_path);

                /*futs.push(task::spawn_blocking(process_image(
                    input_file,
                    output_file
                )));*/
                /*let threadpool_future = task::spawn_blocking(move || process_image(
                    input_file,
                    output_file
                )).await.unwrap();
                */
                futures.push(process_image(
                    input_file,
                    output_file
                ));
            }
        }
    }
    //futures::future::join_all(futures).await;
    //while let Some(_handled) = futures.next().await {}
    //while let Some(_handled) = futs.next().await {}
    futures::future::join_all(futures).await;
}

async fn process_image(file: String, output_file: String) -> Result<(), tokio::task::JoinError>
{
    tokio::task::spawn_blocking(move || {
        let file_path = file.clone();

        let img_reader = image::open(file);

        match img_reader {
            Ok(_) => {
                let img = img_reader.unwrap();
                let dimensions = img.dimensions();

                //let ascii_scale = " .:-=+*#%@";
                let ascii_scale = " .:-=+░▒▓▓";

                let width = dimensions.0;
                let height = dimensions.1;
                let bytes_per_pixel = 4;

                let buffer: Vec<u8> = img.to_rgba8().to_vec();

                println!("Processing [{}], Total pixels: {}", file_path, (buffer.len() / bytes_per_pixel) - 1);
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
            Err(_) => {
                println!("Invalid or corrupted image {}", file_path);
            }
        }
    }).await?;
    Ok(())
}