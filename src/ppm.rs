#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Pixel {
    fn as_string(&self) -> String {
        format!("{} {} {}", self.red, self.green, self.blue)
    }
}

pub(crate) struct Image<const HEIGHT: usize, const WIDTH: usize>(pub [[Pixel; WIDTH]; HEIGHT]);

const ASCII_HEADER: &str = "P3";

impl<const HEIGHT: usize, const WIDTH: usize> Image<HEIGHT, WIDTH> {
    pub fn new() -> Self {
        Self([[Default::default(); WIDTH]; HEIGHT])
    }

    pub fn set(&mut self, row: usize, column: usize, pixel: Pixel) {
        self.0[row][column] = pixel;
    }

    /// TODO: Allocate str with correct size upfront
    pub fn dump(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("{ASCII_HEADER}\n{WIDTH} {HEIGHT}\n255\n"));

        for row in 0..HEIGHT {
            for column in 0..WIDTH {
                let pixel = &self.0[row][column];
                output.push_str(&pixel.as_string());
                output.push(' ');
            }
            output.push('\n');
        }

        output
    }
}
