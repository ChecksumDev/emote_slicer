use image::{imageops, ImageBuffer, Rgba};

const MINIMUM_PIXEL_ALPHA: u8 = 32;
const TWITCH_EMOTE_SIZES: [u32; 3] = [28, 56, 112];
const TWITCH_BADGE_SIZES: [u32; 3] = [18, 36, 72];
const TARGET_SIZE: u32 = 448;

pub fn find_non_transparent_pixels(image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<(usize, usize)> {
    let mut non_transparent_pixels = Vec::new();
    for (x, y, pixel) in image.enumerate_pixels() {
        if pixel[3] > MINIMUM_PIXEL_ALPHA {
            non_transparent_pixels.push((x as usize, y as usize));
        }
    }
    non_transparent_pixels
}

pub fn calculate_bounding_box(group: &Vec<(usize, usize)>) -> (usize, usize, usize, usize) {
    let (min_x, min_y) = group
        .iter()
        .fold((usize::MAX, usize::MAX), |(min_x, min_y), &(x, y)| {
            (min_x.min(x), min_y.min(y))
        });

    let (max_x, max_y) = group.iter().fold((0, 0), |(max_x, max_y), &(x, y)| {
        (max_x.max(x), max_y.max(y))
    });

    (min_x, min_y, max_x, max_y)
}

pub fn calculate_bounding_boxes(
    groups: &Vec<Vec<(usize, usize)>>,
) -> Vec<(usize, usize, usize, usize)> {
    groups
        .iter()
        .map(|group| calculate_bounding_box(group))
        .collect()
}

pub fn find_connected_groups(
    non_transparent_pixels: &Vec<(usize, usize)>,
    pixels: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> Vec<Vec<(usize, usize)>> {
    let mut visited = vec![vec![false; pixels.width() as usize]; pixels.height() as usize];
    let mut groups = Vec::new();

    for &(x, y) in non_transparent_pixels {
        if visited[y][x] {
            continue;
        }

        let mut group = Vec::new();
        let mut stack = vec![(x, y)];
        visited[y][x] = true;

        while let Some((x, y)) = stack.pop() {
            group.push((x, y));

            for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < pixels.width() as i32 && ny >= 0 && ny < pixels.height() as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;

                    if !visited[ny][nx]
                        && pixels.get_pixel(nx as u32, ny as u32)[3] > MINIMUM_PIXEL_ALPHA
                    {
                        visited[ny][nx] = true;
                        stack.push((nx, ny));
                    }
                }
            }
        }

        groups.push(group);
    }

    let mut merged_groups = Vec::new();
    for group in groups {
        let mut merged = false;
        for merged_group in &mut merged_groups {
            let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&group);
            let (min_x2, min_y2, max_x2, max_y2) = calculate_bounding_box(merged_group);

            if min_x >= min_x2 && min_y >= min_y2 && max_x <= max_x2 && max_y <= max_y2 {
                merged_group.extend(group.clone());
                merged = true;
                break;
            }
        }
        if !merged {
            merged_groups.push(group);
        }
    }

    merged_groups
}

pub fn make_group_image(
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    group: &[(usize, usize)],
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (mut min_x, mut min_y) = (image.width(), image.height());
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
            *image.get_pixel(x as u32, y as u32),
        );
    }

    if group_width > TARGET_SIZE && group_height > TARGET_SIZE {
        imageops::resize(
            &group_image,
            TARGET_SIZE,
            TARGET_SIZE,
            imageops::FilterType::Lanczos3,
        )
    } else {
        let mut resized = ImageBuffer::new(TARGET_SIZE, TARGET_SIZE);
        let min_width = std::cmp::min(group_width, TARGET_SIZE);
        let min_height = std::cmp::min(group_height, TARGET_SIZE);
        let start_x = (TARGET_SIZE - min_width) / 2;
        let start_y = TARGET_SIZE - min_height;

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
    id: &str,
    pixels: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> anyhow::Result<()> {
    let sizes = [TWITCH_EMOTE_SIZES, TWITCH_BADGE_SIZES].concat();

    sizes
        .iter()
        .map(|&size| {
            let resized = imageops::resize(pixels, size, size, imageops::FilterType::Lanczos3);
            resized
                .save(format!("emotes/{}/{}.png", id, size))
                .expect("Failed to save image");
        })
        .for_each(drop);

    Ok(())
}
