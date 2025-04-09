use crate::day14::part1::Lobby;
use crate::util::point::Point;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

#[allow(dead_code)]
fn write_to_std_out(iteration: usize, points: &Vec<Point>) {
    let mut display: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];

    points
        .iter()
        .for_each(|p| display[p.y as usize][p.x as usize] = true);

    println!("#{iteration}:");
    display
        .iter()
        .map(|row| {
            row.iter()
                .map(|&cell_occupied| if cell_occupied { 'X' } else { ' ' })
                .collect::<String>()
        })
        .for_each(|line| println!("{line}"));
}

#[allow(dead_code)]
fn save_to_ppm(iteration: usize, points: &Vec<Point>) {
    let mut display: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];

    points
        .iter()
        .for_each(|p| display[p.y as usize][p.x as usize] = true);

    let file_name = format!("./{iteration}.ppm");
    let mut file = File::create(file_name).expect("Failed to create file");

    // Write PPM header (P3 means ASCII, 255 is max color value)
    writeln!(file, "P3\n{} {}\n255", WIDTH, HEIGHT).unwrap();

    // Write pixel data
    for row in &display {
        for &cell in row {
            let pixel_value = if cell { "255 255 255" } else { "0 0 0" }; // RGB format
            writeln!(file, "{}", pixel_value).unwrap();
        }
    }
}

fn write_to_bmp(iteration: u32, points: &Vec<Point>) -> Result<(), io::Error> {
    let mut display: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];

    points
        .iter()
        .for_each(|p| display[p.y as usize][p.x as usize] = true);

    // For 24bpp: each pixel uses 3 bytes.
    let row_size = WIDTH * 3;
    // Each row must be a multiple of 4 bytes.
    let padding = (4 - (row_size % 4)) % 4;
    let padded_row_size = row_size + padding;
    let pixel_array_size = padded_row_size * HEIGHT;
    let header_size = 14 + 40; // BMP header (14 bytes) + DIB header (40 bytes)
    let file_size = header_size + pixel_array_size;

    let file_name = format!("./tmp/{iteration}.bmp");
    let mut file = File::create(file_name)?;

    // --- BMP File Header (14 bytes) ---
    // bfType: "BM"
    file.write_all(b"BM")?;
    // bfSize: file size (4 bytes, little-endian)
    file.write_all(&(file_size as u32).to_le_bytes())?;
    // bfReserved1: 2 bytes, 0
    file.write_all(&0u16.to_le_bytes())?;
    // bfReserved2: 2 bytes, 0
    file.write_all(&0u16.to_le_bytes())?;
    // bfOffBits: offset to pixel data (14 + 40 = 54)
    file.write_all(&(header_size as u32).to_le_bytes())?;

    // --- DIB Header (BITMAPINFOHEADER, 40 bytes) ---
    // biSize: header size (40 bytes)
    file.write_all(&40u32.to_le_bytes())?;
    // biWidth: image width
    file.write_all(&(WIDTH as i32).to_le_bytes())?;
    // biHeight: image height. Positive means bottom-up.
    file.write_all(&(HEIGHT as i32).to_le_bytes())?;
    // biPlanes: must be 1
    file.write_all(&1u16.to_le_bytes())?;
    // biBitCount: 24 bits per pixel
    file.write_all(&24u16.to_le_bytes())?;
    // biCompression: 0 (BI_RGB, no compression)
    file.write_all(&0u32.to_le_bytes())?;
    // biSizeImage: size of pixel data (can be 0 for BI_RGB, but we set it here)
    file.write_all(&(pixel_array_size as u32).to_le_bytes())?;
    // biXPelsPerMeter: horizontal resolution (0 for default)
    file.write_all(&0u32.to_le_bytes())?;
    // biYPelsPerMeter: vertical resolution (0 for default)
    file.write_all(&0u32.to_le_bytes())?;
    // biClrUsed: number of colors in palette (0 for no palette)
    file.write_all(&0u32.to_le_bytes())?;
    // biClrImportant: 0 (all colors are important)
    file.write_all(&0u32.to_le_bytes())?;

    // --- Pixel Data ---
    // Remember: BMP stores rows bottom-up.
    for row in display.iter().rev() {
        for &cell in row.iter() {
            if cell {
                // White pixel: B, G, R
                file.write_all(&[255, 255, 255])?;
            } else {
                // Black pixel: B, G, R
                file.write_all(&[0, 0, 0])?;
            }
        }
        // Write padding for the row.
        file.write_all(&vec![0; padding])?;
    }
    Ok(())
}

pub fn solve_day_14_part_02(input: &str) {
    let lobby = Lobby::try_from((input, WIDTH as u32, HEIGHT as u32)).unwrap();
    let tmp_dir = Path::new("./tmp");
    if !tmp_dir.exists() {
        fs::create_dir("tmp").expect("should create temp dir");
    }

    for i in 8175..8180 {
        let positions_at_target_time: Vec<Point> = lobby
            .robots
            .iter()
            .map(|r| crate::day14::part1::Robot::project_pos(&r, i, lobby.width, lobby.height))
            .collect();

        write_to_bmp(i, &positions_at_target_time).expect("should write bmp");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::file::read_string;

    #[test]
    fn should_solve_day_14_part_02() {
        let input = read_string("./src/day14/input.txt").unwrap();

        solve_day_14_part_02(&input);
    }

    #[test]
    fn should_print_little_x() {
        let points = vec![
            Point::new(0, 0),
            Point::new(1, 1),
            Point::new(2, 2),
            Point::new(0, 2),
            Point::new(2, 0),
        ];

        write_to_std_out(0, &points);
    }
}
