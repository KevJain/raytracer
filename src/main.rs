use std::fs::File;
use std::io::{self, Write};

struct Image {
    height : u32,
    width : u32,
}

fn main() {
    let img = Image { height : 50, width : 100};
    match render(&img) {
        Ok(_) => println!("Rendered file successfully."),
        Err(_) => println!("Failed to render file.")
    }
}

// Outputs the image to output.ppm
fn render(img : &Image) -> io::Result<()> {
    let mut file = File::create("output.ppm")?;

    writeln!(file, "P3");
    writeln!(file, "{} {}", img.width, img.height);
    writeln!(file, "255");
    
    for row in 0..(img.height) {
        for col in 0..(img.width) {
            if (row + col) % 2 == 0 {
                writeln!(file, "0 0 0");
            } else {
                writeln!(file, "255 255 255");
            }
            
        }
    }
    Ok(())

}