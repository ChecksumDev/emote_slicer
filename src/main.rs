pub mod guides;
pub mod image_processing;

use image::ImageBuffer;
use std::{error::Error, fs};

use crate::guides::make_resized_images;

fn main() -> Result<(), Box<dyn Error>> {
    let image = image::open("emotes.png")?.to_rgba8();
    let (width, height) = image.dimensions();

    let pixels = image.into_raw();
    let pixels = ImageBuffer::from_raw(width, height, pixels).unwrap();
    let non_transparent_pixels = image_processing::find_non_transparent_pixels(&pixels);

    std::fs::create_dir_all("debug")?;
    let highlighted_pixels =
        guides::highlight_non_transparent_pixels(&pixels, &non_transparent_pixels);
    highlighted_pixels.save("debug/highlighted_emotes.png")?;

    let emotes = image_processing::find_connected_groups(&non_transparent_pixels, &pixels);
    let highlighted_groups = guides::highlight_connected_groups(&pixels, &emotes);
    highlighted_groups.save("debug/highlighted_groups_emotes.png")?;

    for (i, group) in emotes.iter().enumerate() {
        let group_image = guides::create_group_image(&pixels, group);
        fs::create_dir_all(format!("emotes/{}", i))?;
        group_image.save(format!("emotes/{}/original.png", i))?;

        make_resized_images(&group_image, &i.to_string());
    }

    println!("Connected groups: {}", emotes.len());

    Ok(())
}
