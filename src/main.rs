use std::fs;

pub mod debug_guides;
pub mod image_processing;

fn main() -> anyhow::Result<()> {
    let images = image::open("emotes.png")?.to_rgba8();

    let pixels = image_processing::find_non_transparent_pixels(&images);
    let groups = image_processing::find_connected_groups(&pixels, &images);
    let bounding_boxes = image_processing::calculate_bounding_boxes(&groups);

    if std::env::var("DEBUG").unwrap_or_default() == "true" {
        let mut debug_pixels = images.clone();
        fs::create_dir_all("debug")?;

        debug_guides::highlight_non_transparent_pixels(&mut debug_pixels, &pixels);
        debug_pixels.save("debug/non_transparent_pixels.png")?;

        debug_guides::highlight_connected_groups(&mut debug_pixels, &groups);
        debug_pixels.save("debug/connected_groups.png")?;

        debug_guides::highlight_bounding_boxes(&mut debug_pixels, &bounding_boxes);
        debug_pixels.save("debug/bounding_boxes.png")?;
    }

    fs::create_dir_all("emotes")?;
    for (i, group) in groups.iter().enumerate() {
        println!("Processing {}/{}", i + 1, groups.len());
        fs::create_dir_all(format!("emotes/{}", i))?;

        let image = image_processing::make_group_image(&images, &group);
        image.save(format!("emotes/{}/original.png", i))?;

        image_processing::make_resized_images(&i.to_string(), &image)?;
    }

    println!("Done! Press any key to exit.");
    std::io::stdin().read_line(&mut String::new())?;

    Ok(())
}
