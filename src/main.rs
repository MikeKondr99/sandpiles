use image::ImageBuffer;
use image::Rgb;
use nalgebra::SMatrix;
use show_image::create_window;

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    const N:usize = 231;
    let window = create_window("image", Default::default())?;
    let mut grid = SMatrix::<u32, N, N>::zeros();
    grid[(N/2, N/2)] = 100000;
    loop {
        for x in 0..N {
            for y in 0..N {
                let d = grid[(x, y)] / 4;
                grid[(x, y)] %= 4;
                if x > 0 {
                    grid[(x - 1, y)] += d;
                }
                if x < N - 1 {
                    grid[(x + 1, y)] += d;
                }
                if y < N-1 {
                    grid[(x, y + 1)] += d;
                }
                if y > 0 {
                    grid[(x, y - 1)] += d;
                }
            }
        }
        window.set_image(
            "image-001",
            ImageBuffer::from_fn(N as u32, N as u32, |x, y| {
                let color = grid[(x as usize, y as usize)] % 4;
                match color {
                    0 => Rgb([158, 0, 89]),
                    1 => Rgb([255, 0, 84]),
                    2 => Rgb([255, 84, 0]),
                    _ => Rgb([255, 189, 0]),
                }
            }),
        )?;
    }
}
