use image::RgbImage;
use rav1e::prelude::*;
use std::fs::File;
use std::io::Write;

use crate::av1::convert;
use crate::{HEIGHT, WIDTH};

pub struct Animation {
    frames: Vec<RgbImage>,
}

impl Animation {
    pub fn new(job_size: usize) -> Self {
        let mut frames: Vec<RgbImage> = Vec::new();
        frames.resize(job_size, RgbImage::new(WIDTH as u32, HEIGHT as u32));
        Animation { frames: frames }
    }

    pub fn data(self) -> Vec<u8> {
        self.frames
            .into_iter()
            .map(|f| f.into_raw())
            .flatten()
            .collect::<Vec<u8>>()
    }

    pub fn preallocate_root_buffer(total_size: usize) -> Vec<u8> {
        let mut root_buf = Vec::new();
        root_buf.resize(total_size * WIDTH * HEIGHT * 3, 0);
        root_buf
    }

    pub fn reconstruct(data: Vec<u8>) -> Self {
        Animation {
            frames: data
                .chunks(WIDTH * HEIGHT * 3)
                .map(|frame| {
                    RgbImage::from_raw(WIDTH as u32, HEIGHT as u32, frame.to_vec()).unwrap()
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn frames(&mut self) -> &mut Vec<RgbImage> {
        &mut self.frames
    }

    pub fn len(&self) -> usize {
        self.frames.len()
    }

    pub fn save(&self, filename: &str) {
        let mut ecfg = EncoderConfig::with_speed_preset(10);
        ecfg.width = WIDTH;
        ecfg.height = HEIGHT;
        ecfg.chroma_sampling = ChromaSampling::Cs444;
        let cfg = Config::default().with_encoder_config(ecfg);
        let mut ctx: Context<u8> = cfg.new_context().unwrap();

        let mut output_file = File::create(filename).unwrap();

        self.frames.iter().for_each(|frame| {
            ctx.send_frame(convert(&ctx, frame)).unwrap();

            while let Ok(packet) = ctx.receive_packet() {
                output_file.write_all(&packet.data).unwrap();
            }
        });

        ctx.flush();
        dbg!(&ctx);

        while let Ok(packet) = ctx.receive_packet() {
            output_file.write_all(&packet.data).unwrap();
        }
    }
}
