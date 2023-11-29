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
    // CUSTOMER (AKA PLAYER)
    let mut sprites: Vec<GPUSprite> = vec![GPUSprite {
        screen_region: [8.0*CELL_WIDTH, 2.0 * CELL_HEIGHT, CELL_WIDTH, 2.0 * CELL_HEIGHT],
        sheet_region: [384.0/1408.0, 0.0, 64.0/1408.0, 128.0/320.0], 
    }];

    //WALLS: sprite[0] to sprite[70]!! (sprite = shelf1)
    for y in 0..NUMBER_OF_CELLS_H {
        let y_value = y as f32 * CELL_HEIGHT;
        if y == 0 {
            for x in 0..NUMBER_OF_CELLS_W {
                sprites.push(GPUSprite {
                    screen_region: [x as f32 * CELL_WIDTH, y_value, 50.0, 50.0],
                    sheet_region: [1088.0/1408.0, 0.0, 64.0/1408.0, 64.0/320.0], 
                });
            }
        }
        // top of the wall & door
        else if y == (NUMBER_OF_CELLS_H - 1) {
            for x in 0..door_xvalue {
                sprites.push(GPUSprite {
                    screen_region: [x as f32 * CELL_WIDTH, y_value, 50.0, 50.0],
                    sheet_region: [1088.0/1408.0, 0.0, 64.0/1408.0, 64.0/320.0], 
                });
            } // empty door (57)
            for x in door_xvalue..door_xvalue + 1 {
                sprites.push(GPUSprite {
                    screen_region: [x as f32 * CELL_WIDTH, y_value, 50.0, 50.0],
                    sheet_region:  [832.0/1408.0, 77.0/320.0, 62.0/1408.0, 64.0/320.0],
                });
                print!("DOOR: {:#?}", sprites.len());
            } for x in door_xvalue+1..NUMBER_OF_CELLS_W {
                sprites.push(GPUSprite {
                    screen_region: [x as f32 * CELL_WIDTH, y_value, 50.0, 50.0],
                    sheet_region:  [1088.0/1408.0, 0.0, 64.0/1408.0, 64.0/320.0], 
                });
            }
        }
        // sides of the wall 
        else {
            sprites.push(GPUSprite {
                screen_region: [0 as f32 * CELL_WIDTH, y_value, 50.0, 50.0],
                sheet_region:  [1088.0/1408.0, 0.0, 64.0/1408.0, 64.0/320.0], 
            });
            //right side of the wall 
            sprites.push(GPUSprite {
                screen_region: [(NUMBER_OF_CELLS_W - 1) as f32 * CELL_WIDTH, y_value, 50.0, 50.0],
                sheet_region:  [1088.0/1408.0, 0.0, 64.0/1408.0, 64.0/320.0], 
            });
        }
    }

    let even_x= 4.0;let odd_x= 4.0;
    let even_y= 9.0; let odd_y= 4.0;

    let mut coord1 = 0.0; let mut coord2 = 0.0; let mut coord3 = 0.0; let mut coord4 = 0.0;
    //AISLES: (sprite = shelf4) & FOOD (70 - 97)
    print!("AISLE-START: {:#?}", sprites.len());
    for x in 0..4 {
        let num_a= rand::thread_rng().gen_range(1..3) as f32;
        let num_b= rand::thread_rng().gen_range(1..3) as f32;
        
        if x % 2 == 0 { 
            if x == 0 {
                coord1 = 0.0; coord2 = 64.0; coord3 = 256.0; coord4 = 320.0;
            } else {
                coord1 = 448.0; coord2 = 512.0; coord3 = 640.0; coord4 = 704.0;
            }  
            
            sprites.push(GPUSprite {
                screen_region: [4.0 * x as f32 * CELL_WIDTH + even_x * CELL_WIDTH, even_y * CELL_HEIGHT, CELL_WIDTH, 4.0*CELL_HEIGHT],
                sheet_region: [1280.0/1408.0, 0.0, 64.0/1408.0, 256.0/320.0], 
            });
            sprites.push(GPUSprite {
                screen_region: [4.0 * x as f32 * CELL_WIDTH + even_x * CELL_WIDTH, even_y * CELL_HEIGHT, 50.0, 50.0],
                sheet_region: [coord1/1408.0, 0.0, 64.0/1408.0, 64.0/320.0], 
            });
            sprites.push(GPUSprite {
                screen_region: [4.0 * x as f32 * CELL_WIDTH + even_x * CELL_WIDTH, (even_y + 1.0) * CELL_HEIGHT, 45.0, 45.0],
                sheet_region: [coord2/1408.0, 0.0, 17.0/1408.0, 32.0/320.0], 
            }); //FOOD
            sprites.push(GPUSprite {
                screen_region: [4.0 * x as f32 * CELL_WIDTH + (even_x - num_a) * CELL_WIDTH, (even_y + 1.0) * CELL_HEIGHT, CELL_WIDTH, CELL_HEIGHT],
                sheet_region: [coord2/1408.0, 0.0, 17.0/1408.0, 32.0/320.0], 
            });
            print!("FOOD: {:#?}", sprites.len());
            sprites.push(GPUSprite {
                screen_region: [4.0 * x as f32 * CELL_WIDTH + even_x * CELL_WIDTH, (even_y + 2.0) * CELL_HEIGHT, 50.0, 50.0],
                sheet_region: [coord3/1408.0, 0.0, 64.0/1408.0, 64.0/320.0], 
            });
            sprites.push(GPUSprite {
                screen_region: [4.0 * x as f32 * CELL_WIDTH + even_x * CELL_WIDTH, (even_y + 3.0) * CELL_HEIGHT, 45.0, 45.0],
                sheet_region: [coord4/1408.0, 0.0, 17.0/1408.0, 32.0/320.0], 
            }); //FOOD
            sprites.push(GPUSprite {
                screen_region: [4.0 * x as f32 * CELL_WIDTH + (even_x + num_b) * CELL_WIDTH, (even_y + 3.0) * CELL_HEIGHT, CELL_WIDTH, CELL_HEIGHT],
                sheet_region: [coord4/1408.0, 0.0, 17.0/1408.0, 32.0/320.0],
            });
            print!("FOOD: {:#?}", sprites.len());
        } else { 
            if x == 1 {
                coord1 = 128.0; coord2 = 192.0; coord3 = 960.0; coord4 = 1024.0;
            } else {
                coord1 = 832.0; coord2 = 896.0; coord3 = 0.0; coord4 = 64.0;
            }
            sprites.push(GPUSprite {
                screen_region: [4.0 * x as f32 * CELL_WIDTH + odd_x * CELL_WIDTH, odd_y * CELL_HEIGHT, CELL_WIDTH, 4.0*CELL_HEIGHT],
                sheet_region: [1280.0/1408.0, 0.0, 64.0/1408.0, 256.0/320.0], 
            });
            sprites.push(GPUSprite {
                screen_region: [4.0 * x as f32 * CELL_WIDTH + odd_x * CELL_WIDTH, odd_y * CELL_HEIGHT, 50.0, 50.0],
                sheet_region:  [coord1/1408.0, 0.0, 64.0/1408.0, 64.0/320.0], 
            });
            sprites.push(GPUSprite {
                screen_region: [4.0 * x as f32 * CELL_WIDTH + odd_x * CELL_WIDTH, (odd_y + 1.0) * CELL_HEIGHT, 45.0, 45.0],
                sheet_region: [coord2/1408.0, 0.0, 17.0/1408.0, 32.0/320.0], 
            }); 
            sprites.push(GPUSprite {
                screen_region: [4.0 * x as f32 * CELL_WIDTH + (odd_x - num_a)* CELL_WIDTH, (odd_y + 1.0) * CELL_HEIGHT,  CELL_WIDTH, CELL_HEIGHT],
                sheet_region: [coord2/1408.0, 0.0, 17.0/1408.0, 32.0/320.0], 
            });
            print!("FOOD: {:#?}", sprites.len());
            sprites.push(GPUSprite {
                screen_region: [4.0 * x as f32 * CELL_WIDTH + odd_x * CELL_WIDTH, (odd_y + 2.0) * CELL_HEIGHT, 50.0, 50.0],
                sheet_region: [coord3/1408.0, 0.0, 64.0/1408.0, 64.0/320.0], 
            });
            sprites.push(GPUSprite {
                screen_region: [4.0 * x as f32 * CELL_WIDTH + odd_x * CELL_WIDTH, (odd_y + 3.0) * CELL_HEIGHT, 45.0, 45.0],
                sheet_region: [coord4/1408.0, 0.0, 17.0/1408.0, 32.0/320.0], 
            }); 
            if x == 1 {
                sprites.push(GPUSprite {
                    screen_region: [4.0 * x as f32 * CELL_WIDTH + (odd_x + num_b) * CELL_WIDTH, (odd_y + 3.0) * CELL_HEIGHT,  CELL_WIDTH, CELL_HEIGHT],
                    sheet_region: [coord4/1408.0, 0.0, 17.0/1408.0, 32.0/320.0], 
                });
            }
            print!("FOOD: {:#?}", sprites.len());
        }
    }
    print!("AISLE-END: {:#?}", sprites.len());

    // GROCERY LIST
    sprites.push(GPUSprite {
        screen_region: [1.0 * CELL_WIDTH, 1.0 * CELL_HEIGHT, 80.0, 320.0],
        sheet_region: [960.0/1408.0, 64.0/320.0, 64.0/1408.0, 120.0/320.0], 
    }); // banana -  99
    print!("LIST: {:#?}", sprites.len());
    sprites.push(GPUSprite {
        screen_region: [1.6 * CELL_WIDTH, 1.0 * CELL_HEIGHT, 30.0, 30.0],
        sheet_region: [64.0/1408.0, 0.0, 17.0/1408.0, 32.0/320.0], 
    }); //bread - 100
    sprites.push(GPUSprite {
        screen_region: [1.6 * CELL_WIDTH, 1.8 * CELL_HEIGHT, 30.0, 30.0],
        sheet_region: [192.0/1408.0, 0.0, 17.0/1408.0, 32.0/320.0], 
    });//carrot - 101
    sprites.push(GPUSprite {
        screen_region: [1.6 * CELL_WIDTH, 2.6 * CELL_HEIGHT, 30.0, 30.0],
        sheet_region: [320.0/1408.0, 0.0, 17.0/1408.0, 32.0/320.0], 
    }); //salad - 102
    sprites.push(GPUSprite {
        screen_region: [1.6 * CELL_WIDTH, 3.4 * CELL_HEIGHT, 30.0, 30.0],
        sheet_region: [1024.0/1408.0, 0.0, 17.0/1408.0, 32.0/320.0], 
    }); //cereal - 103
    sprites.push(GPUSprite {
        screen_region: [1.6 * CELL_WIDTH, 4.2 * CELL_HEIGHT, 30.0, 30.0],
        sheet_region: [512.0/1408.0, 0.0, 17.0/1408.0, 32.0/320.0], 
    });//ketchup - 104
    sprites.push(GPUSprite {
        screen_region: [1.6 * CELL_WIDTH, 5.0 * CELL_HEIGHT, 30.0, 30.0],
        sheet_region: [704.0/1408.0, 0.0, 17.0/1408.0, 32.0/320.0], 
    }); //potato chip - 105
    sprites.push(GPUSprite {
        screen_region: [1.6 * CELL_WIDTH, 5.8 * CELL_HEIGHT, 30.0, 30.0],
        sheet_region: [896.0/1408.0, 0.0, 17.0/1408.0, 32.0/320.0], 
    });

    // CASHIER 
    sprites.push(GPUSprite {
        screen_region: [18.0 * CELL_WIDTH, 10.0 * CELL_HEIGHT, 64.0, 128.0],
        sheet_region: [576.0/1408.0, 0.0, 64.0/1408.0, 128.0/320.0], 
    });

    print!("ASSOC-START: {:#?}", sprites.len());
    // creating enemy ASSOCIATES (sprites 106-111)
    sprites.push(GPUSprite {
        screen_region: [4.0 * CELL_WIDTH, 14.0 * CELL_HEIGHT,CELL_WIDTH, CELL_HEIGHT],
        sheet_region: [0.54545454545454545454, 0.0, 0.01136364, 0.05],
    });
    sprites.push(GPUSprite {
        screen_region: [4.0 * CELL_WIDTH, 2.0 * CELL_HEIGHT, CELL_WIDTH, CELL_HEIGHT],
        sheet_region: [0.54545454545454545454, 0.0, 0.01136364, 0.05], 
    });
    sprites.push(GPUSprite {
        screen_region: [12.0 * CELL_WIDTH, 7.0 * CELL_HEIGHT, CELL_WIDTH, CELL_HEIGHT],
        sheet_region: [0.54545454545454545454, 0.0, 0.01136364, 0.05], 
    });
    sprites.push(GPUSprite {
        screen_region: [16.0 * CELL_WIDTH, 2.0 * CELL_HEIGHT, CELL_WIDTH, CELL_HEIGHT],
        sheet_region: [0.54545454545454545454, 0.0, 0.01136364, 0.05], 
    });
    sprites.push(GPUSprite {
        screen_region: [16.0 * CELL_WIDTH, 13.0 * CELL_HEIGHT, CELL_WIDTH, CELL_HEIGHT],
        sheet_region: [0.54545454545454545454, 0.0, 0.01136364, 0.05], 
    });
    
    print!("ASSOC-END: {:#?}", sprites.len());
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
                sprite_position[1] = WINDOW_HEIGHT - 3.0 * CELL_HEIGHT;
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

