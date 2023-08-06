use std::vec;

use image::io::Reader as ImageReader;

fn map_range(s: u32, from_range: (u32, u32), to_range: (u32, u32)) -> u32 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

fn main() {
    //let density: Vec<&str> = "Ñ@#W$9876543210?!abc;:+=-,._ ".split("").collect();
    let density_light = vec![
        "Ñ", "@", "#", "W", "$", "9", "8", "7", "6", "5", "4", "3", "2", "1", "0", "?", "!", "a",
        "b", "c", ";", ":", "+", "=", "-", ",", ".", "_", " ",
    ];

    let density_dark = vec![
        " ", "_", ".", ",", "-", "=", "+", ":", ";", "c", "b", "a", "!", "?", "0", "1", "2", "3",
        "4", "5", "6", "7", "8", "9", "$", "W", "#", "@", "Ñ",
    ];
    //let density_light = vec!["▓", "▒", "░"];
    //let density_dark = vec!["░", "▒", "▓"];

    let img = ImageReader::open("assets/chainsawman-50.png")
        .unwrap()
        .decode()
        .unwrap();
    let rgb = img.into_rgb8();

    let mut final_vec = vec![];

    //for x in 0..rgb.width() {
    for y in 0..rgb.height() {
        let mut vec = vec![];
        // for y in 0..rgb.height() {
        for x in 0..rgb.width() {
            let pixel = rgb.get_pixel_checked(x, y).unwrap();
            let r = pixel.0[0] as u32;
            let g = pixel.0[1] as u32;
            let b = pixel.0[2] as u32;
            let prom = (r + g + b) / 3;

            let mut index = map_range(prom, (0, 255), (0, density_dark.len() as u32));

            if index >= density_dark.len() as u32 {
                index -= 1;
            }

            vec.push(density_dark[index as usize]);
        }

        final_vec.push(vec);
    }

    println!("{:?}", final_vec);
    //for vec in final_vec {
    //    print!("\x1b[?25l");
    //    let string = vec.join("");
    //    println!("{:?}", string);
    //}

    // TODO: export a new image

    /*
    for x in 0..rgb.width() {
        for y in 0..rgb.height() {
            let pixel = rgb.get_pixel_checked(x, y).unwrap();
            let r = pixel.0[0] as u32;
            let g = pixel.0[1] as u32;
            let b = pixel.0[2] as u32;
            let prom = (r + g + b) / 3;

            let mut index = map_range(prom, (0, 255), (0, density.len() as u32));

            if index >= density.len() as u32 {
                index -= 1;
            }
            final_vec.push(density[index as usize]);
        }
    }

    println!("{:?}", final_vec);
    */
}
