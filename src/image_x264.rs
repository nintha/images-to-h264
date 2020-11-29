use std::{fs::File};
use std::{io::Write};
use std::time::Instant;

use x264::{Colorspace, Image, Preset, Setup, Tune};

use crate::yuv_util::convert_rgb_to_yuv420p;
use image::imageops::FilterType;

pub fn gen_h264(
    fps: u32,
    out_width: u32,
    out_height: u32,
    input: &str,
    start_num: u32,
    output: &str,
) -> anyhow::Result<()> {
    // Initialize things.
    let mut encoder = Setup::preset(Preset::Ultrafast, Tune::None, false, true)
        .fps(fps, 1)
        .annexb(true)
        .keyint_max(fps as i32 * 2)
        .build(Colorspace::I420, out_width as _, out_height as _)
        .unwrap();
    let mut file = File::create(output).unwrap();

    println!("Initialized!");

    // Write the headers.
    {
        let headers = encoder.headers().unwrap();
        let header = headers.entirety().to_vec();
        file.write_all(&header)?;
    }

    // Queue each frame.
    let mut frame_index = 0;
    let mut encode_cost_sum = 0;
    for num in start_num.. {
        let filename = input.replace("%d", &num.to_string());
        let img = if let Ok(r) = image::io::Reader::open(filename) {
            r.decode()?
                .resize(out_width as _, out_height as _, FilterType::Nearest)
                .to_rgb8()
        } else {
            break;
        };

        let instant = Instant::now();
        let yuv = convert_rgb_to_yuv420p(img.as_ref(), img.width(), img.height(), 3);
        let image = Image::yuv420p(img.width() as _, img.height() as _, &yuv);
        let (data, _) = encoder.encode(frame_index as _, image).unwrap();
        if !data.entirety().is_empty() {
            let cost = instant.elapsed().as_millis();
            encode_cost_sum += cost;
            println!("#{:04} len={}, cost={}ms", frame_index, &data.entirety().len(), cost);
            frame_index += 1;
            file.write_all(data.entirety())?;
        }
    }
    println!("Done! encode {} frames, avg_cost={}ms", frame_index, encode_cost_sum as f64 / 300.0);
    Ok(())
}

