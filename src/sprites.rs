use bytemuck::{Pod, Zeroable};
use rand::Rng;
use crate::{WINDOW_WIDTH, WINDOW_HEIGHT, NUMBER_OF_CELLS_W, NUMBER_OF_CELLS_H, CELL_WIDTH, CELL_HEIGHT};
use crate::input::Input;

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct GPUSprite {
    pub screen_region: [f32; 4],
    pub sheet_region: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Zeroable, Pod)]
pub struct GPUCamera {
    pub screen_pos: [f32; 2],
    pub screen_size: [f32; 2],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SpriteOption {
    Storage,
    Uniform,
    VertexBuffer,
}

//value to not hard code the door placement
pub const door_xvalue: i32 = NUMBER_OF_CELLS_W/3; 

pub fn create_sprites() ->  Vec<GPUSprite> {
    // sheet region: left x, top y,  width, height
    let mut sprites: Vec<GPUSprite> = vec![GPUSprite {
        screen_region: [WINDOW_WIDTH/2.0, 32.0, 50.0, 50.0],
        sheet_region: [0.0625, 0.0625, 0.3125, 0.375], // duck
    }];

    //WALLS: sprite[0] to sprite[70]!!
    for y in 0..NUMBER_OF_CELLS_H {
        let y_value = y as f32 * CELL_HEIGHT;
        // ASTEROIDS == wall
        // bottom on the wall 
        if y == 0 {
            for x in 0..NUMBER_OF_CELLS_W {
                sprites.push(GPUSprite {
                    screen_region: [x as f32 * CELL_WIDTH, y_value, 50.0, 50.0],
                    sheet_region: [0.5625, 0.6, 0.375, 0.25], 
                });
            }
        }
        // top of the wall & door
        else if y == (NUMBER_OF_CELLS_H - 1) {
            for x in 0..door_xvalue {
                sprites.push(GPUSprite {
                    screen_region: [x as f32 * CELL_WIDTH, y_value, 50.0, 50.0],
                    sheet_region: [0.5625, 0.6, 0.375, 0.25], 
                });
            } //DOOR 
            for x in door_xvalue..door_xvalue + 1 {
                sprites.push(GPUSprite {
                    screen_region: [x as f32 * CELL_WIDTH, y_value, 50.0, 50.0],
                    sheet_region: [0.625, 0.125, 0.25, 0.25], // bomb
                });
                print!("{:#?}", sprites.len());
            } for x in door_xvalue+1..NUMBER_OF_CELLS_W {
                sprites.push(GPUSprite {
                    screen_region: [x as f32 * CELL_WIDTH, y_value, 50.0, 50.0],
                    sheet_region: [0.5625, 0.6, 0.375, 0.25], 
                });
            }
        }
        // sides of the wall 
        else {
            sprites.push(GPUSprite {
                screen_region: [0 as f32 * CELL_WIDTH, y_value, 50.0, 50.0],
                sheet_region: [0.5625, 0.6, 0.375, 0.25], 
            });
            //right side of the wall 
            sprites.push(GPUSprite {
                screen_region: [(NUMBER_OF_CELLS_W - 1) as f32 * CELL_WIDTH, y_value, 50.0, 50.0],
                sheet_region: [0.5625, 0.6, 0.375, 0.25], 
            });
        }


    }

    // creating ENEMIES
    for y in 0..7 {
        // Create a horizontal line of stars, asteroids, and bombs
            let num_x= rand::thread_rng().gen_range(1..NUMBER_OF_CELLS_W) as f32; 
            let num_y= rand::thread_rng().gen_range(1..NUMBER_OF_CELLS_H) as f32; 

            //star == associate
            sprites.push(GPUSprite {
                screen_region: [num_x * CELL_WIDTH, num_y * CELL_HEIGHT, 50.0, 50.0],
                sheet_region: [0.125, 0.625, 0.25, 0.25], // star
            });

            /*
            // BOMBS
            sprites.push(GPUSprite {
                screen_region: [11 as f32 * CELL_WIDTH, y_value, 45.0, 45.0],
                sheet_region: [0.625, 0.125, 0.25, 0.25], // bomb
            });
            */
    }
    sprites

}

pub fn move_sprite_input(input: &Input, mut sprite_position: [f32; 2], collided_wall: bool) -> [f32; 2] {
        // Update sprite position based on keyboard input
        if input.is_key_pressed(winit::event::VirtualKeyCode::Up) {
            if sprite_position[1] + 2.0 * CELL_HEIGHT >= WINDOW_HEIGHT && sprite_position[0] == door_xvalue as f32 * CELL_WIDTH {
                sprite_position[1] += CELL_HEIGHT;
            }
            else if collided_wall && sprite_position[1] + 2.0 * CELL_HEIGHT >= WINDOW_HEIGHT {
                sprite_position[1] = WINDOW_HEIGHT - 2.0 * CELL_HEIGHT;
            } else {
                sprite_position[1] += CELL_HEIGHT;
            }
        }
        
        if input.is_key_pressed(winit::event::VirtualKeyCode::Down) {
            if collided_wall && sprite_position[1] - CELL_HEIGHT <= 0.0 {
                sprite_position[1] = CELL_HEIGHT;
            } else {
            sprite_position[1] -= CELL_HEIGHT;
            }
        }
        if input.is_key_pressed(winit::event::VirtualKeyCode::Left) {
            if collided_wall && sprite_position[0] - 1.5 * CELL_WIDTH <= 0.0 {
                sprite_position[0] = CELL_WIDTH;
            } else {
            sprite_position[0] -= CELL_WIDTH;
            }
        }
        if input.is_key_pressed(winit::event::VirtualKeyCode::Right) {
            if collided_wall && sprite_position[0] + 2.0 * CELL_WIDTH >= WINDOW_WIDTH {
                sprite_position[0] = WINDOW_WIDTH - 2.0 * CELL_WIDTH;
            } else {
                sprite_position[0] += CELL_WIDTH;
            }
        }  
        sprite_position
}

