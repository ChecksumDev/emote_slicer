use std::collections::HashSet;

use image::Rgba;

use image::ImageBuffer;
use rand::Rng;

pub fn highlight_non_transparent_pixels(
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    non_transparent_pixels: &HashSet<(usize, usize)>,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut highlighted_pixels = image.clone();
    for &(x, y) in non_transparent_pixels {
        highlighted_pixels.put_pixel(x as u32, y as u32, Rgba([255, 0, 0, 255]));
    }
    highlighted_pixels
}

pub fn highlight_connected_groups(
    pixels: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    groups: &[Vec<(usize, usize)>],
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut highlighted_groups = pixels.clone();
    let mut rng = rand::thread_rng();
    for group in groups {
        let color = Rgba([
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            rng.gen_range(0..=255),
            255,
        ]);

        for &(x, y) in group {
            highlighted_groups.put_pixel(x as u32, y as u32, color);
        }
    }
    highlighted_groups
}

pub fn create_group_image(
    pixels: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    group: &[(usize, usize)],
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (mut min_x, mut min_y) = (pixels.width(), pixels.height());
    let (mut max_x, mut max_y) = (0, 0);

    for &(x, y) in group {
        min_x = min_x.min(x as u32);
        min_y = min_y.min(y as u32);
        max_x = max_x.max(x as u32);
        max_y = max_y.max(y as u32);
    }

    let group_width = max_x - min_x + 1;
    let group_height = max_y - min_y + 1;

    let mut group_image = ImageBuffer::new(group_width, group_height);
    for &(x, y) in group {
        group_image.put_pixel(
            x as u32 - min_x,
            y as u32 - min_y,
            *pixels.get_pixel(x as u32, y as u32),
        );
    }

    group_image
}
