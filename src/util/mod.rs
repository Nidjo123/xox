
pub fn color_to_rgba(color: u32) -> [u8; 4] {
    [
        ((color >> 24) & 0xff) as u8, 
        ((color >> 16) & 0xff) as u8, 
        ((color >> 8) & 0xff) as u8, 
        (color & 0xff) as u8
    ]
}