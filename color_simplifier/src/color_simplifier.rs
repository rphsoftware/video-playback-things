// Colors are stored in the following byte format:
// <unused> <R> <G> <B>
// Reading colors:
// r = (color >> 16) & 0xFF
// g = (color >> 8) & 0xFF
// b = color & 0xFF
//
// Other way around just bitshift in the other direction

// SOURCES FOR COLOR SIMPLIFICATION ALGHORITMS:
// https://github.com/fifoc/encoder

use crate::color_utils::{join_colors, split_colors};
use std::collections::HashMap;
use std::i64::MAX;
use serde_json::Value;

#[allow(dead_code)]
pub struct ColorPalette {
    pub pal: Vec<u32>,
    simplification_cache: Vec<u8>
}

#[allow(dead_code)]
impl ColorPalette {
    fn add(&mut self, r: u8, g: u8, b: u8) {
        self.pal.push(join_colors(r, g, b));
    }

    // This function returns the closest* color in the palette to the parameter
    // Based in part on
    // https://github.com/fifoc/encoder/blob/master/fifEncoder.go#L9
    pub fn simplify(&mut self, color: u32) -> usize {
        if self.simplification_cache[color as usize] != 255 {
            #[cfg(debug_assertions)]
            println!("DEBUG: Used color cache!");

            return self.simplification_cache[color as usize] as usize;
        }

        let mut closest_delta: i64 = MAX;
        let mut pick : usize = 0;

        let (r, g, b) = split_colors(color);
        for i in 0..self.pal.len() {
            let iter_color = self.pal[i];
            if iter_color == color {
                self.simplification_cache[color as usize] = i as u8;
                return i;
            } else {
                let (p_r, p_g, p_b) = split_colors(iter_color as u32);
                let factor_r = (p_r as i64) - (r as i64);
                let factor_g = (p_g as i64) - (g as i64);
                let factor_b = (p_b as i64) - (b as i64);

                let delta = (factor_r * factor_r) + (factor_g * factor_g) + (factor_b * factor_b);
                if delta < closest_delta {
                    closest_delta = delta;
                    pick = i;
                }
            }
        }


        self.simplification_cache[color as usize] = pick as u8;
        return pick;
    }
}

pub fn generate_map_palette() -> std::io::Result<ColorPalette> {
    let hm : HashMap<u32, usize> = HashMap::new();
    let mut c = ColorPalette{ pal: vec![], simplification_cache: vec![255; 0x1000000] };

    let v: Value = serde_json::from_str(include_str!("colors.json"))?;

    if let Value::Array(v) = v {
        for e in v {
            if let Value::Array(e) = e {
                let mut index = 0;
                let mut color = 0;
                for c in e {
                    if let Value::Number(c) = c {
                        if let Some(c) = c.as_i64() {
                            let c = c as u32;
                            color += c << match index {
                                0 => 16,
                                1 => 8,
                                _ => 0
                            };

                            index += 1;
                        }
                    }
                }
                c.pal.push({
                    let mut color = color.clone();
                    let r = (color >> 16) & 0xFF;
                    let g = (color >> 8) & 0xFF;
                    let b = color & 0xFF;
                    let r = (r * 180) / 255;
                    let g = (g * 180) / 255;
                    let b = (b * 180) / 255;

                    (r << 16) + (g << 8) + b
                });
                c.pal.push({
                    let mut color = color.clone();
                    let mut r = (color >> 16) & 0xFF;
                    let mut g = (color >> 8) & 0xFF;
                    let mut b = color & 0xFF;
                    let r = (r * 220) / 255;
                    let g = (g * 220) / 255;
                    let b = (b * 220) / 255;

                    (r << 16) + (g << 8) + b
                });
                c.pal.push(color);
                c.pal.push({
                    let mut color = color.clone();
                    let mut r = (color >> 16) & 0xFF;
                    let mut g = (color >> 8) & 0xFF;
                    let mut b = color & 0xFF;
                    let r = (r * 135) / 255;
                    let g = (g * 135) / 255;
                    let b = (b * 135) / 255;

                    (r << 16) + (g << 8) + b
                });
            }
        }
    }

    println!("{:?}", c.pal);

    Ok(c)
}