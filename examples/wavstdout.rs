// Generate endless screeching noise to stdout

// Usage: cargo run --example wavstdout | mpv -

extern crate hnd;

use hnd::Sample;
use std::io::Write;

fn main() {
    let spec = hnd::WavSpec {
        bits_per_sample: 16,
        channels: 1,
        sample_format: hnd::SampleFormat::Int,
        sample_rate: 16000,
    };

    let v = spec.into_header_for_infinite_file();

    let so = std::io::stdout();
    let mut so = so.lock();
    so.write_all(&v[..]).unwrap();

    loop {
        for i in 0..126 {
            let x: i16 = (i * 256) as i16;
            if x.write(&mut so, 16).is_err() {
                return;
            }
        }
    }
}
