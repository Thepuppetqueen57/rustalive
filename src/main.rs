use raylib::prelude::*;
use rand::Rng;

#[derive(PartialEq)]
enum CellType {
    Alive,
    Food
}

struct Cell {
    x: i32,
    y: i32,
    ctype: CellType,
    food: i16,
    last_move: u8
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .resizable()
        .title("Geometry Rays")
        .build();

    rl.set_target_fps(60);

    let mut grid: Vec<Cell> = vec![];
    let pixel_size: u8 = 40;
    let mut delta_time: u16 = 0;

    grid.push(Cell {
        x: 2 * pixel_size as i32,
        y: 2 * pixel_size as i32,
        ctype: CellType::Alive,
        food: 0,
        last_move: 0
    });

    grid.push(Cell {
        x: 2 * pixel_size as i32,
        y: 1 * pixel_size as i32,
        ctype: CellType::Alive,
        food: 0,
        last_move: 0
    });

    grid.push(Cell {
        x: 2 * pixel_size as i32,
        y: 3 * pixel_size as i32,
        ctype: CellType::Food,
        food: 0,
        last_move: 0
    });

    while !rl.window_should_close() {
        if delta_time == 30 || delta_time == 60 || delta_time == 90 {
            let mut cell_index = 0;
            while cell_index < grid.len() {
                if grid[cell_index].ctype == CellType::Alive {
                    let mut food_cell_index = 0;
                    while food_cell_index < grid.len() {
                        if grid[food_cell_index].ctype == CellType::Food {
                            if grid[food_cell_index].x <= grid[cell_index].x + 5 {
                                if delta_time == 30 || delta_time == 60 || delta_time == 90 {
                                    grid[cell_index].x += pixel_size as i32;
                                }
                            } else if grid[food_cell_index].x >= grid[cell_index].x - 5 {
                                if delta_time == 30 || delta_time == 60 || delta_time == 90 {
                                    grid[cell_index].x -= pixel_size as i32;
                                }
                            } else if grid[food_cell_index].y <= grid[cell_index].y + 5 {
                                if delta_time == 30 || delta_time == 60 || delta_time == 90 {
                                    grid[cell_index].y += pixel_size as i32;
                                }
                            } else if grid[food_cell_index].y >= grid[cell_index].y - 5 {
                                if delta_time == 30 || delta_time == 60 || delta_time == 90 {
                                    grid[cell_index].y -= pixel_size as i32;
                                }
                            } else {
                                let direction = rand::rng().random_range(1..4);
                
                                match direction {
                                    1 => {
                                        grid[cell_index].x += pixel_size as i32;
                                        grid[cell_index].last_move = 1
                                    }
                
                                    2 => {
                                        grid[cell_index].x -= pixel_size as i32;
                                        grid[cell_index].last_move = 2
                                    }
                
                                    3 => {
                                        grid[cell_index].y += pixel_size as i32;
                                        grid[cell_index].last_move = 3
                                    }
                
                                    _ => {
                                        grid[cell_index].y -= pixel_size as i32;
                                        grid[cell_index].last_move = 4
                                    }
                                }
                            }
                        }
        
                        food_cell_index += 1
                    }
                }
    
                cell_index += 1
            }
        }

        if delta_time == 100 {
            let mut cell_index = 0;
            while cell_index < grid.len() {
                if grid[cell_index].ctype == CellType::Alive {
                    grid[cell_index].food -= 1
                }

                cell_index += 1
            }
        }

        let mut cell_index = 0;
        while cell_index < grid.len() {
            if grid[cell_index].food < 0 {
                grid.remove(cell_index);
            }

            cell_index += 1
        }

        let mut cell_index = 0;
        while cell_index < grid.len() {
            let mut food_cell_index = 0;
            while food_cell_index < grid.len() {
                if grid[food_cell_index].ctype == CellType::Food &&
                grid[food_cell_index].x == grid[cell_index].x &&
                grid[food_cell_index].y == grid[cell_index].y {
                    if delta_time == 30 || delta_time == 60 || delta_time == 90 {
                        grid[cell_index].food += 1;
                    }
                } else if grid[food_cell_index].ctype == CellType::Alive &&
                grid[food_cell_index].x == grid[cell_index].x &&
                grid[food_cell_index].y == grid[cell_index].y &&
                food_cell_index != cell_index {
                    if delta_time == 30 || delta_time == 60 || delta_time == 90 {
                        grid.push(Cell {
                            x: cell_index as i32,
                            y: cell_index as i32 + 80,
                            ctype: CellType::Alive,
                            food: 0,
                            last_move: 0
                        });
                    }
                }

                food_cell_index += 1
            }

            cell_index += 1
        }

        let mut rld = rl.begin_drawing(&thread);
        rld.clear_background(Color::WHITE);

        for cell in &grid {
            rld.draw_rectangle(
                cell.x,
                cell.y,
                pixel_size as i32,
                pixel_size as i32,
                if cell.ctype == CellType::Alive {
                    Color::BLUE
                } else {
                    Color::RED
                }
            );
        }

        let mut cell_index = 0;
        for cell in &grid {
            if cell.ctype == CellType::Alive {
                rld.draw_text(
                    &format!("{}", grid[cell_index].food),
                    20,
                    20 + cell_index as i32 * 20,
                    20,
                    Color::LIME
                );
            }

            cell_index += 1;
        }

        if delta_time == 100 {
            delta_time = 0
        } else {
            delta_time += 1;
        }
    }
}