//======================================================================
//  COLORS
//======================================================================
pub const C_BLACK: [u8; 4] = [0x00, 0x00, 0x00, 0xff];
pub const C_WHITE: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
pub const C_RED: [u8; 4] = [0xff, 0x00, 0x00, 0xff];
pub const C_GREEN: [u8; 4] = [0x00, 0xff, 0x00, 0xff];
pub const C_BLUE: [u8; 4] = [0x00, 0x00, 0xff, 0xff];

//======================================================================
//  VALUES
//======================================================================
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 500;
pub const BACKGROUND_COLOR: [u8; 4] = C_BLACK;
pub const FPS: u32 = 60;
pub const FRAME_TARGET_TIME: f32 = 1000.0 / FPS as f32;
