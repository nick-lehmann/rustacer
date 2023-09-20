mod ppm;
use ppm::Image;

const HEIGHT: usize = 255;
const WIDTH: usize = 255;

fn main() {
    let mut image: Image<HEIGHT, WIDTH> = Image::new();

    for row in 0..HEIGHT {
        for column in 0..WIDTH {
            image.set(
                row,
                column,
                ppm::Pixel {
                    red: row as u8,
                    green: column as u8,
                    blue: 0,
                },
            );
        }
    }

    println!("{}", image.dump());
}
