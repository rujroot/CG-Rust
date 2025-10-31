use image::{ ImageBuffer, Pixel, Rgb };
use std::time::Instant;
use num_complex::Complex;
use hsv_to_rgb::hsv_to_rgb;

fn main() {
    let image_width:u32 = 1920;
    let image_height:u32 = 1080;
    let max_iterations:u32 = 1000;

    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(image_width, image_height);

    let x_min:f64 = -2.0;
    let x_max:f64 = 1.0;
    let y_min:f64 = -1.0;
    let y_max:f64 = 1.0;

    let start = Instant::now();
    for y in 0..image_height {
        for x in 0..image_width {
            let c_real = x_min + (x as f64 / image_width as f64) * (x_max - x_min);
            let c_imag = y_min + (y as f64 / image_height as f64) * (y_max - y_min);
            let c = Complex::new(c_real, c_imag);

            let mut z = Complex::new(0.0, 0.0);
            let mut iterations = 0;
            while z.norm_sqr() <= 4.0 && iterations < max_iterations {
                z = z * z + c;
                iterations += 1;
            }

            let pixel:Rgb<u8> = if iterations == max_iterations {
                let result: Rgb<u8> = Rgb([0, 0, 0]);
                result
            } else {
                let t = iterations as f32 / max_iterations as f32;
                let hue = (t * 360.0 * 3.0) % 360.0;
                let saturation = 0.8;
                let value = if t < 0.5 { 0.5 + t } else { 1.0 };
                let result: Rgb<u8> = hsv_to_rgb(hue, saturation, value);
                result
            };
            
            imgbuf.put_pixel(x, y, pixel);
        }
    }

    let duration = start.elapsed();
    println!("Rendering time: {:?}", duration);

    std::fs::create_dir_all("./out").unwrap();
    imgbuf.save("./out/mandelbrot_single.png").unwrap();
    println!("Image saved to ./out/mandelbrot_single.png");
}