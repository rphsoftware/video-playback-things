pub fn join_colors(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) +
        ((g as u32) << 8) +
        (b as u32)
}

pub fn split_colors(source:u32) -> (u8, u8, u8) {
    (
        ((source >> 16) & 0xFF) as u8,
        ((source >> 8) & 0xFF)  as u8,
        (source & 0xFF)         as u8
    )
}