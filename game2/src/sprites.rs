use rand::Rng;
use engine::{WINDOW_WIDTH, WINDOW_HEIGHT, NUMBER_OF_CELLS_W, NUMBER_OF_CELLS_H, CELL_WIDTH, CELL_HEIGHT};
use engine::input::Input;
use engine::sprite::GPUSprite;

//value to not hard code the door placement
pub const door_xvalue: i32 = NUMBER_OF_CELLS_W/3; 

pub fn create_sprites() ->  Vec<GPUSprite> {
    // CUSTOMER (AKA PLAYER)
    let mut sprites: Vec<GPUSprite> = vec![];

    //WALLS: sprite[0] to sprite[69]!! 
    for y in 0..NUMBER_OF_CELLS_H {
        let y_value = y as f32 * CELL_HEIGHT;
        if y == 0 {
            for x in 0..NUMBER_OF_CELLS_W {
                sprites.push(GPUSprite {
                    screen_region: [x as f32 * CELL_WIDTH, y_value, 50.0, 50.0],
                    sheet_region: [1088.0/1408.0, 0.0, 1.0/1408.0, 1.0/320.0], 
                });
            }
        }
        // top of the wall & door
        else if y == (NUMBER_OF_CELLS_H - 1) {
            for x in 0..door_xvalue {
                sprites.push(GPUSprite {
                    screen_region: [x as f32 * CELL_WIDTH, y_value, 50.0, 50.0],
                    sheet_region: [1088.0/1408.0, 0.0, 1.0/1408.0, 1.0/320.0], 
                });
            } 
            for x in door_xvalue..door_xvalue + 1 {
                sprites.push(GPUSprite {
                    screen_region: [x as f32 * CELL_WIDTH, y_value, 50.0, 50.0],
                    sheet_region:  [1088.0/1408.0, 0.0, 1.0/1408.0, 1.0/320.0],
                });
            } for x in door_xvalue+1..NUMBER_OF_CELLS_W {
                sprites.push(GPUSprite {
                    screen_region: [x as f32 * CELL_WIDTH, y_value, 50.0, 50.0],
                    sheet_region:  [1088.0/1408.0, 0.0, 1.0/1408.0, 1.0/320.0], 
                });
            }
        }
        // sides of the wall 
        else {
            sprites.push(GPUSprite {
                screen_region: [0 as f32 * CELL_WIDTH, y_value, 50.0, 50.0],
                sheet_region:  [1088.0/1408.0, 0.0, 1.0/1408.0, 1.0/320.0], 
            });
            //right side of the wall 
            sprites.push(GPUSprite {
                screen_region: [(NUMBER_OF_CELLS_W - 1) as f32 * CELL_WIDTH, y_value, 50.0, 50.0],
                sheet_region:  [1088.0/1408.0, 0.0, 1.0/1408.0, 1.0/320.0], 
            });
        }
    }

    //BANANAS: 70-202
    for x in 1..NUMBER_OF_CELLS_W-1 {
        for y in 1..8 {
            sprites.push(GPUSprite {
                screen_region: [x as f32 * CELL_WIDTH, y as f32 * CELL_HEIGHT, 50.0, 50.0],
                sheet_region: [0.0, 0.0, 64.0/1408.0, 0.2], 
            });
        }
    }
    
    //CABBAGE: 203-335
    for x in 1..NUMBER_OF_CELLS_W-1 {
        for y in 8..NUMBER_OF_CELLS_H-1 {
            sprites.push(GPUSprite {
                screen_region: [x as f32 * CELL_WIDTH, y as f32 * CELL_HEIGHT, 50.0, 50.0],
                sheet_region: [960.0/1408.0, 0.0, 64.0/1408.0, 64.0/320.0], 
            });
        }
    }

    //ASSOCIATE PLAYER - FOR GAME 2 (green)
    sprites.push(GPUSprite {
        screen_region: [9.0 * CELL_WIDTH, 8.0 * CELL_HEIGHT, CELL_WIDTH, CELL_HEIGHT],
        sheet_region: [0.54545454545454545454, 0.05, 0.01136364, 0.05],
    });
    //ASSOCIATE PLAYER - FOR GAME 2 (red)
    sprites.push(GPUSprite {
        screen_region: [10.0 * CELL_WIDTH, 7.0 * CELL_HEIGHT, CELL_WIDTH, CELL_HEIGHT],
        sheet_region: [0.54545454545454545454, 0.0, 0.01136364, 0.05],
    });

    sprites

}

pub fn move_sprite_input(input: &Input, mut sprite_position: [f32; 2], collided_wall: bool, at_door: bool, aisle_left: bool, aisle_right: bool, aisle_top:bool, aisle_bottom:bool) -> [f32; 2] {
        // Update sprite position based on keyboard input
        if input.is_key_pressed(winit::event::VirtualKeyCode::Up) {
            if aisle_top {
                sprite_position[1]+=0.0; 
            }
            else if at_door {
                sprite_position[1] += CELL_HEIGHT;
            }
            else if collided_wall && sprite_position[1] + 3.0 * CELL_HEIGHT >= WINDOW_HEIGHT {
                sprite_position[1] = WINDOW_HEIGHT - 2.0 * CELL_HEIGHT;
            } else {
                sprite_position[1] += CELL_HEIGHT;
            }
        }
        
        if input.is_key_pressed(winit::event::VirtualKeyCode::Down) {
            if aisle_bottom {
                sprite_position[1]+=0.0; 
            }
            else if collided_wall && sprite_position[1] - CELL_HEIGHT <= 0.0 {
                sprite_position[1] = CELL_HEIGHT;
            } else {
            sprite_position[1] -= CELL_HEIGHT;
            }
        }
        if input.is_key_pressed(winit::event::VirtualKeyCode::Left) {
            if aisle_left  {
                sprite_position[0] += 0.0; 
            }
            else if collided_wall && sprite_position[0] - 1.5 * CELL_WIDTH <= 0.0 {
                sprite_position[0] = CELL_WIDTH;
            } else {
            sprite_position[0] -= CELL_WIDTH;
            }
        }
        if input.is_key_pressed(winit::event::VirtualKeyCode::Right) {
            if aisle_right{
                sprite_position[0] += 0.0; 
            }
            else if collided_wall && sprite_position[0] + 2.0 * CELL_WIDTH >= WINDOW_WIDTH {
                sprite_position[0] = WINDOW_WIDTH - 2.0 * CELL_WIDTH;
            } else {
                sprite_position[0] += CELL_WIDTH;
            }
        }  
        sprite_position
}


pub fn move_sprite_input_2(input: &Input, mut sprite_position: [f32; 2], collided_wall: bool, at_door: bool, aisle_left: bool, aisle_right: bool, aisle_top:bool, aisle_bottom:bool) -> [f32; 2] {
        // Update sprite position based on keyboard input
        if input.is_key_pressed(winit::event::VirtualKeyCode::W) {
            if aisle_top {
                sprite_position[1]+=0.0; 
            }
            else if at_door {
                sprite_position[1] += CELL_HEIGHT;
            }
            else if collided_wall && sprite_position[1] + 2.0 * CELL_HEIGHT >= WINDOW_HEIGHT {
                sprite_position[1] = WINDOW_HEIGHT - 2.0 * CELL_HEIGHT;
            } else {
                sprite_position[1] += CELL_HEIGHT;
            }
        }
        
        if input.is_key_pressed(winit::event::VirtualKeyCode::S) {
            if aisle_bottom {
                sprite_position[1]+=0.0; 
            }
            else if collided_wall && sprite_position[1] - CELL_HEIGHT <= 0.0 {
                sprite_position[1] = CELL_HEIGHT;
            } else {
            sprite_position[1] -= CELL_HEIGHT;
            }
        }
        if input.is_key_pressed(winit::event::VirtualKeyCode::A) {
            if aisle_left  {
                sprite_position[0] += 0.0; 
            }
            else if collided_wall && sprite_position[0] - 1.5 * CELL_WIDTH <= 0.0 {
                sprite_position[0] = CELL_WIDTH;
            } else {
            sprite_position[0] -= CELL_WIDTH;
            }
        }
        if input.is_key_pressed(winit::event::VirtualKeyCode::D) {
            if aisle_right{
                sprite_position[0] += 0.0; 
            }
            else if collided_wall && sprite_position[0] + 2.0 * CELL_WIDTH >= WINDOW_WIDTH {
                sprite_position[0] = WINDOW_WIDTH - 2.0 * CELL_WIDTH;
            } else {
                sprite_position[0] += CELL_WIDTH;
            }
        }  
        sprite_position
}
