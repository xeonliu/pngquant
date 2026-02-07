/*
** © 2024 by Kornel Lesiński.
**
** Core pngquant compression logic
*/

use imagequant;
use std::path::Path;

/// Options for PNG quantization
#[derive(Debug, Clone)]
pub struct QuantOptions {
    pub quality_min: u8,
    pub quality_max: u8,
    pub speed: u32,
    pub colors: u32,
    pub dithering_level: f32,
    pub posterize: u32,
}

impl Default for QuantOptions {
    fn default() -> Self {
        QuantOptions {
            quality_min: 0,
            quality_max: 100,
            speed: 4,
            colors: 256,
            dithering_level: 1.0,
            posterize: 0,
        }
    }
}

/// Result type for PNG operations
pub type PngResult<T> = Result<T, String>;

/// Compress PNG data from bytes to bytes
pub fn compress_png_bytes(
    input_data: &[u8],
    options: &QuantOptions,
) -> PngResult<Vec<u8>> {
    // Create temporary files for processing
    let temp_dir = std::env::temp_dir();
    let input_path = temp_dir.join(format!("pngquant_input_{}.png", std::process::id()));
    let output_path = temp_dir.join(format!("pngquant_output_{}.png", std::process::id()));
    
    // Write input data to temporary file
    std::fs::write(&input_path, input_data)
        .map_err(|e| format!("Failed to write input file: {}", e))?;
    
    // Process the file
    let result = compress_png_file(
        input_path.to_str().unwrap(),
        output_path.to_str().unwrap(),
        options,
    );
    
    // Read output and cleanup
    let output_data = match result {
        Ok(_) => {
            let data = std::fs::read(&output_path)
                .map_err(|e| format!("Failed to read output file: {}", e));
            let _ = std::fs::remove_file(&input_path);
            let _ = std::fs::remove_file(&output_path);
            data
        }
        Err(e) => {
            let _ = std::fs::remove_file(&input_path);
            let _ = std::fs::remove_file(&output_path);
            Err(e)
        }
    };
    
    output_data
}

/// Compress PNG file using libimagequant
pub fn compress_png_file(
    input_path: &str,
    output_path: &str,
    options: &QuantOptions,
) -> PngResult<()> {
    // Read PNG file using image crate
    let img = image::open(Path::new(input_path))
        .map_err(|e| format!("Failed to open input file: {}", e))?;

    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let raw_pixels = rgba.into_raw();
    
    // Convert u8 slice to RGBA slice
    let rgba_pixels: Vec<imagequant::RGBA> = raw_pixels
        .chunks_exact(4)
        .map(|chunk| imagequant::RGBA { r: chunk[0], g: chunk[1], b: chunk[2], a: chunk[3] })
        .collect();

    // Create imagequant attributes
    let mut liq = imagequant::new();
    
    // Set quality
    liq.set_quality(options.quality_min, options.quality_max)
        .map_err(|e| format!("Failed to set quality: {:?}", e))?;

    // Set speed
    liq.set_speed(options.speed as i32)
        .map_err(|e| format!("Failed to set speed: {:?}", e))?;

    // Set max colors
    if options.colors < 256 {
        liq.set_max_colors(options.colors)
            .map_err(|e| format!("Failed to set max colors: {:?}", e))?;
    }

    // Create image from RGBA buffer
    let mut img = liq.new_image(
        rgba_pixels,
        width as usize,
        height as usize,
        0.0,
    ).map_err(|e| format!("Failed to create image: {:?}", e))?;

    // Quantize
    let mut res = liq.quantize(&mut img)
        .map_err(|e| format!("Quantization failed: {:?}", e))?;

    // Set dithering level
    res.set_dithering_level(options.dithering_level)
        .map_err(|e| format!("Failed to set dithering level: {:?}", e))?;

    // Remap image
    let (palette, pixels) = res.remapped(&mut img)
        .map_err(|e| format!("Failed to remap image: {:?}", e))?;

    // Convert indexed pixels back to RGBA
    let mut rgba_output: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);
    for &idx in &pixels {
        let color = &palette[idx as usize];
        rgba_output.push(color.r);
        rgba_output.push(color.g);
        rgba_output.push(color.b);
        rgba_output.push(color.a);
    }

    // Create output image
    use image::{ImageBuffer, RgbaImage};
    let output_img: RgbaImage = ImageBuffer::from_raw(width, height, rgba_output)
        .ok_or_else(|| "Failed to create output image buffer".to_string())?;

    // Save the output image
    output_img.save(Path::new(output_path))
        .map_err(|e| format!("Failed to save output file: {}", e))?;

    Ok(())
}
