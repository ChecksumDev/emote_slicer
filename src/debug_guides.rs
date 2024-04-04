use image::{ImageBuffer, Rgba};
use rand::Rng;

pub fn highlight_non_transparent_pixels(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    non_transparent_pixels: &[(usize, usize)],
) {
    non_transparent_pixels.iter().for_each(|&(x, y)| {
        image.put_pixel(x as u32, y as u32, Rgba([255, 0, 0, 255]));
    });
}

pub fn highlight_connected_groups(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    groups: &[Vec<(usize, usize)>],
) {
    let mut rng = rand::thread_rng();
    for group in groups {
        let color = Rgba([
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            255,
        ]);

        for &(x, y) in group {
            image.put_pixel(x as u32, y as u32, color);
        }
    }
}

pub fn highlight_bounding_boxes(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    bounding_boxes: &[(usize, usize, usize, usize)],
) {
    for &(min_x, min_y, max_x, max_y) in bounding_boxes {
        for x in min_x..=max_x {
            for y in min_y..min_y + 4 {
                image.put_pixel(x as u32, y as u32, Rgba([255, 0, 0, 255]));
            }
            for y in max_y - 3..=max_y {
                image.put_pixel(x as u32, y as u32, Rgba([255, 0, 0, 255]));
            }
        }

        for y in min_y..=max_y {
            for x in min_x..min_x + 4 {
                image.put_pixel(x as u32, y as u32, Rgba([255, 0, 0, 255]));
            }
            for x in max_x - 3..=max_x {
                image.put_pixel(x as u32, y as u32, Rgba([255, 0, 0, 255]));
            }
        }
    }
}
