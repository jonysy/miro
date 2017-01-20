use image::{GrayImage, Luma};
use imageproc::definitions::Clamp;
use imageproc::filter::Kernel;

/// Anti-aliasing filter kernel is used for pyramid construction (used for image anti-aliasing 
/// before image subsampling).
fn filter(im: &GrayImage) -> GrayImage {
    // [1/16 1/4 3/8 1/4 1/16] × [1/16 1/4 3/8 1/4 1/16]T
    let slice = &[0.2734375];
    let ker = Kernel::new(slice, 1, 1);
    ker.filter(im, |channel, acc| *channel = Clamp::clamp(acc))
}

/// Builds a pyramid representation of the provided image.
///
/// The pyramid representation is built in a recursive fashion: compute I¹ from I⁰, then 
/// compute I² from I¹, and so on..
///
/// # Arguments
///
/// * `im` - The highest resolution image (the raw image or the "zeroᵗʰ" level image).
/// * `nlayers` - The height of the pyramid.
///
/// # Returns
///
/// Returns the pyramid representation of image `im`
pub fn pyramid(im: &GrayImage, nlayers: usize) -> Vec<GrayImage> {
    let mut vec = Vec::with_capacity(nlayers);
    
    vec.push(im.clone());
    
    for level in 1..nlayers {
        let previous = filter(&vec[level - 1]);
        
        let (prev_w, prev_h) = previous.dimensions();
        
        let curr_w = (prev_w + 1) / 2;
        let curr_h = (prev_h + 1) / 2;

        let mut curr = GrayImage::new(curr_w, curr_h);
        
        for x in 0..curr_w { for y in 0..curr_h {
            let prev_x = 2.0 * x as f32;
            let prev_y = 2.0 * y as f32;

            let sum1 = safepx(&previous, prev_x, prev_y) / 4.0;

            let sum2 = {
                let mut sum = 0.0;
                sum += safepx(&previous, prev_x - 1.0, prev_y);
                sum += safepx(&previous, prev_x + 1.0, prev_y);
                sum += safepx(&previous, prev_x, prev_y - 1.0);
                sum += safepx(&previous, prev_x, prev_y + 1.0);
                sum /= 8.0;
                sum
            };

            let sum3 = {
                let mut sum = 0.0;
                sum += safepx(&previous, prev_x - 1.0, prev_y - 1.0);
                sum += safepx(&previous, prev_x + 1.0, prev_y - 1.0);
                sum += safepx(&previous, prev_x - 1.0, prev_y + 1.0);
                sum += safepx(&previous, prev_x + 1.0, prev_y + 1.0);
                sum /= 16.0;
                sum
            };

            let sum = sum1 + sum2 + sum3;

            let data = [sum as u8];
            let luma = Luma { data: data };

            curr.put_pixel(x, y, luma);
        }}
        
        vec.push(curr);
    }
    
    vec
}

fn safepx(im: &GrayImage, x: f32, y: f32) -> f32 {
    let (w, h) = im.dimensions();
    let (w, h) = (w as f32, h as f32);

    let x = x.min(w - 1.0).max(0.0) as u32;
    let y = y.min(h - 1.0).max(0.0) as u32;

    im[(x, y)].data[0] as f32
}

/// Subpixel computation
pub fn subpx(im: &GrayImage, x: f32, y: f32) -> f32 {
    let x_0 = x.floor();
    let y_0 = y.floor();
        
    let α_x = x - x_0;
    let α_y = y - y_0;
        
    (1.0 - α_x) * (1.0 - α_y) * safepx(im, x_0, y_0)
        + α_x * (1.0 - α_y) * safepx(im, x_0 + 1.0, y_0)
        + (1.0 - α_x) * α_y * safepx(im, x_0, y_0 + 1.0)
        + α_x * α_y * safepx(im, x_0 + 1.0, y_0 + 1.0)
}