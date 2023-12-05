pub mod input;
pub mod gpu;
pub mod sprite;
pub mod gamestate; 
//pub mod sound;

// get the width and height of the whole game screen
pub const  WINDOW_WIDTH: f32 = 1024.0;
pub const  WINDOW_HEIGHT: f32 = 768.0;
pub const NUMBER_OF_CELLS_H: i32 = 16;
pub const NUMBER_OF_CELLS_W: i32 = 21;
// here divide by a number to create the number of grids
pub const CELL_WIDTH: f32 = WINDOW_WIDTH / NUMBER_OF_CELLS_W as f32;
pub const CELL_HEIGHT: f32 = WINDOW_HEIGHT / NUMBER_OF_CELLS_H as f32;

