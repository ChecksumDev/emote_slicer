use std::collections::HashSet;

use image::{ImageBuffer, Rgba};

pub fn find_non_transparent_pixels(
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> HashSet<(usize, usize)> {
    let mut non_transparent_pixels = HashSet::new();
    for (x, y, pixel) in image.enumerate_pixels() {
        if pixel[3] > 64 {
            non_transparent_pixels.insert((x as usize, y as usize));
        }
    }
    non_transparent_pixels
}

pub fn find_connected_groups(
    non_transparent_pixels: &HashSet<(usize, usize)>,
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

                    if !visited[ny][nx] && pixels.get_pixel(nx as u32, ny as u32)[3] > 64 {
                        visited[ny][nx] = true;
                        stack.push((nx, ny));
                    }
                }
            }
        }

        groups.push(group);
    }

    groups
}