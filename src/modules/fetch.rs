use std::env;
use std::process::Command;
use ::chafa::ChafaCanvas;
use image::DynamicImage;
const SRC_WIDTH: u32 =40;
const SRC_HEIGHT: u32 =30;



pub fn run_command(command: &str, args: &[&str]) -> String {
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).to_string()
}


pub fn get_env_variable(var: &str) -> String {
    env::var(var).unwrap_or_else(|_| "Unknown".to_string())
}
// modules/display.rs

pub fn display_image_with_info(img: &DynamicImage, info: &[String; 9]) {
    let _banner = create_chafa(img);
    // Split the banner into lines
    let banner = _banner.split("\n").collect::<Vec<_>>();

    // Printing everything to the screen
    let mut info_iter = info.iter();
    for line in banner.iter() {
        if let Some(info_line) = info_iter.next() {
            println!("{} {}", line, info_line);
        } else {
            println!("{}", line);
        }
    }
}




pub fn display_default(banner:String,info: &[String; 9]){
    let split_banner= banner.split("\n").collect::<Vec<_>>();
    let mut info_iter = info.iter();
    for line in split_banner.iter() {
        if let Some(info_line) = info_iter.next() {
            println!("{}{}", line, info_line);
        } else {
            println!("{}", line);
        }
    }
}

fn create_chafa(img: &DynamicImage) -> String {
    let canvas = ChafaCanvas::from_term(SRC_WIDTH, SRC_HEIGHT);
    let pixels = img.to_rgba8();
    return canvas.draw(&pixels, img.width(), img.height());
}


