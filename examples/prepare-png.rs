use anyhow::{Context, Result};
use image::{Rgba, RgbaImage};
use rusttype::{point, Font, Scale};
use std::io::{Seek, Write};

fn text_to_png<W: Write + Seek>(
    text: &str,
    font: &Font,
    // size: f32,
    mut output: W,
) -> Result<()> {
    let font_size = 108.0f32;
    let (width, height) = (68, 62);
    // Create a scale for the font size
    let scale = Scale::uniform(font_size);

    // Measure the text to determine the size of the image
    let v_metrics = font.v_metrics(scale);
    let glyphs: Vec<_> = font
        .layout(text, scale, point(0.0, v_metrics.ascent))
        .collect();

    // let width = glyphs
    //     .iter()
    //     .rev()
    //     .filter_map(|g| {
    //         g.pixel_bounding_box()
    //             .map(|b| b.min.x as f32 + g.unpositioned().h_metrics().advance_width)
    //     })
    //     .next()
    //     .unwrap_or(0.0)
    //     .ceil() as u32;
    // // let width = 100;
    // let height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    // debug!("frame {text}: {width}x{height}");

    // Create a new image with the calculated width and height
    let mut image = RgbaImage::new(width, height);

    // Draw the text onto the image
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x + bounding_box.min.x as u32 + 3;
                let y = y + bounding_box.min.y as u32 - 51;
                if x < width && y < height {
                    // image.put_pixel(x, y, Rgba([255, 255, 255, (v * 255.0) as u8]));
                    image.put_pixel(x, y, Rgba([0, 0, 0, (v * 255.0) as u8]));
                }
            });
        }
    }

    image.write_to(&mut output, image::ImageFormat::Png)?;

    Ok(())
}

fn main() -> Result<()> {
    let font = std::fs::read("./font/digits.ttf")?;
    let font = rusttype::Font::try_from_vec(font).context("load font failed")?;

    let output = std::path::PathBuf::from("src/frames");
    if !output.exists() {
        std::fs::create_dir_all(&output)?;
    }
    let mut png = Vec::new();
    for i in 0..100 {
        text_to_png(
            &format!("{i:02}"),
            &font,
            // 108.0,
            std::io::Cursor::new(&mut png),
        )?;
        std::fs::write(output.join(format!("{i:02}.png")), &png)?;
    }

    Ok(())
}
