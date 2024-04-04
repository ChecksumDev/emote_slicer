use image::{ImageBuffer, Rgba};

const MINIMUM_PIXEL_ALPHA: u8 = 32;

pub fn calculate_bounding_box(group: &Vec<(usize, usize)>) -> (usize, usize, usize, usize) {
    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    for &(x, y) in group {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    (min_x, min_y, max_x, max_y)
}

pub fn calculate_bounding_boxes(groups: &Vec<Vec<(usize, usize)>>) -> Vec<(usize, usize, usize, usize)> {
    groups.iter().map(|group| calculate_bounding_box(group)).collect()
}

pub fn find_non_transparent_pixels(image: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<(usize, usize)> {
    let mut non_transparent_pixels = Vec::new();
    for (x, y, pixel) in image.enumerate_pixels() {
        if pixel[3] > MINIMUM_PIXEL_ALPHA {
            non_transparent_pixels.push((x as usize, y as usize));
        }
    }
    non_transparent_pixels
}

pub fn find_connected_groups(
    non_transparent_pixels: &Vec<(usize, usize)>,
    pixels: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> Vec<Vec<(usize, usize)>> {
    let mut visited = vec![vec![false; pixels.width() as usize]; pixels.height() as usize];
    let mut groups = Vec::new();

    for (x, y) in non_transparent_pixels {
        if visited[*y][*x] {
            continue;
        }

        let mut group = Vec::new();
        let mut stack = vec![(*x, *y)];
        visited[*y][*x] = true;

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
        for (_i, merged_group) in merged_groups.iter_mut().enumerate() {
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
