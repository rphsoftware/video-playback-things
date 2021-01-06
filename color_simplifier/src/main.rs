use std::sync::mpsc::channel;
use notify::{watcher, Watcher, RecursiveMode};
use std::time::{Duration, Instant};
use notify::DebouncedEvent::Create;
use std::thread::sleep;
use std::fs::File;
use std::io::{Error, ErrorKind, Write, BufWriter};
use crate::color_utils::{join_colors, split_colors};
use crate::color_simplifier::generate_map_palette;
use std::fs;

mod color_simplifier;
mod color_utils;


fn main() {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_millis(16)).unwrap();

    watcher.watch("/home/rph/ramdisk/desktopstreaming/input/", RecursiveMode::NonRecursive).unwrap();
    let mut pal = generate_map_palette().unwrap();

    let mut indox = 0;
    // Populate palette
    for i in 0..0xFFFFFF {
        pal.simplify(i);

        if i % 0xFFF == 0 {
            println!("{} / {}", i, 0xFFFFFF);
        }
    }
    loop {
        match rx.recv() {
            Ok(Create(event)) => {
                sleep(Duration::from_millis(1));
                let i = Instant::now();

                let fd = File::open(event.clone()).unwrap();
                let decoder = png::Decoder::new(fd);
                let (info, mut reader) = decoder.read_info()
                    .unwrap();

                let mut buf = vec![0u8; info.buffer_size()];

                let multiplier = match info.color_type {
                    png::ColorType::RGB => Ok(3),
                    png::ColorType::RGBA => Ok(4),
                    _ => Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Unsupported color space"
                    ))
                }
                    .unwrap();
                reader.next_frame(&mut buf)
                    .unwrap();
                let mut write_buf = Vec::with_capacity(16384 * 24);

                // Create small buffers for every
                for i in 0..40 {
                    let y_offset = (i / 8) * 128;
                    let x_offset = (i % 8) * 128;

                    for y in 0..128 {
                        for x in 0..128 {
                            let index = (
                                (
                                    (y + y_offset) * (info.width as usize)
                                ) * multiplier
                            ) + (
                                    (x + x_offset) * multiplier
                            );

                            let r = buf[index];
                            let g = buf[index + 1];
                            let b = buf[index + 2];
                            write_buf.push((pal.simplify(join_colors(r, g, b)) as u8) + 4);
                        }
                    }


                }
                let mut write_fd = File::create(
                    format!("/home/rph/ramdisk/desktopstreaming/maps/map{}", indox)
                ).unwrap();
                indox += 1;
                write_fd.write(&*write_buf);
                fs::remove_file(event);
                println!("{:?}", Instant::now() - i);

            },
            Err(e) => println!("watch error: {:?}", e),
            _ => {}
        }
    }
}
