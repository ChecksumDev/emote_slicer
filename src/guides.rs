use image::ImageBuffer;
use image::Rgba;
use rand::Rng;

const TWITCH_EMOTE_SIZES: [u32; 3] = [28, 56, 112];
const TWITCH_BADGE_SIZES: [u32; 3] = [18, 36, 72];
const TARGET_SIZE: u32 = 448;

pub fn highlight_non_transparent_pixels(
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    non_transparent_pixels: &Vec<(usize, usize)>,
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
    
    if group_width > TARGET_SIZE && group_height > TARGET_SIZE {
        let resized = image::imageops::resize(
            &group_image,
            TARGET_SIZE,
            TARGET_SIZE,
            image::imageops::FilterType::Lanczos3,
        );
    
        resized
    } else {
        let mut resized = ImageBuffer::new(TARGET_SIZE, TARGET_SIZE);
        let min_width = std::cmp::min(group_width, TARGET_SIZE);
        let min_height = std::cmp::min(group_height, TARGET_SIZE);
        let start_x = (TARGET_SIZE - min_width) / 2; // Center horizontally
        let start_y = TARGET_SIZE - min_height; // Align to bottom
    
        for x in 0..min_width {
            for y in 0..min_height {
                let target_x = start_x + x;
                let target_y = start_y + y;
                resized.put_pixel(target_x, target_y, *group_image.get_pixel(x, y));
            }
        }
    
        resized
    }
    
}

pub fn make_resized_images(
    pixels: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    name: &str,
) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let sizes = [TWITCH_EMOTE_SIZES, TWITCH_BADGE_SIZES].concat();

    sizes
        .iter()
        .map(|&size| {
            let resized =
                image::imageops::resize(pixels, size, size, image::imageops::FilterType::Lanczos3);

            resized
                .save(format!("emotes/{}/{}.png", name, size))
                .unwrap();

            resized
        })
        .collect()
}

pub fn highlight_bounding_boxes(
    pixels: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    bounding_boxes: &[(usize, usize, usize, usize)],
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut highlighted_pixels = pixels.clone();
    for &(min_x, min_y, max_x, max_y) in bounding_boxes {
        for x in min_x..=max_x {
            highlighted_pixels.put_pixel(x as u32, min_y as u32, Rgba([255, 0, 0, 255]));
            highlighted_pixels.put_pixel(x as u32, max_y as u32, Rgba([255, 0, 0, 255]));
        }
        for y in min_y..=max_y {
            highlighted_pixels.put_pixel(min_x as u32, y as u32, Rgba([255, 0, 0, 255]));
            highlighted_pixels.put_pixel(max_x as u32, y as u32, Rgba([255, 0, 0, 255]));
        }
    }
    highlighted_pixels
}
