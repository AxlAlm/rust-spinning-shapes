const CUBE_WIDTH: i32 = 20;
const HORIZONTL_OFFSET: i32 = -2 * CUBE_WIDTH;

const WIDTH: usize = 160;
const HEIGHT: usize = 44;
const K1: usize = 40;

use std::{thread, time, time::Instant};
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

fn calculate_x(pos: &Vec3, angle: &Vec3) -> f32 {
    return pos.y * angle.x.sin() * angle.y.sin() * angle.z.cos()
        - pos.z * angle.x.cos() * angle.y.sin() * angle.z.cos()
        + pos.y * angle.x.cos() * angle.z.sin()
        + pos.z * angle.x.sin() * angle.z.sin()
        + pos.x * angle.y.cos() * angle.z.cos();
}

fn calculate_y(pos: &Vec3, angle: &Vec3) -> f32 {
    return pos.y * angle.x.cos() * angle.z.cos() + pos.z * angle.x.sin() * angle.z.cos()
        - pos.y * angle.x.sin() * angle.y.sin() * angle.z.sin()
        + pos.z * angle.x.cos() * angle.y.sin() * angle.z.sin()
        - pos.x * angle.y.cos() * angle.z.sin();
}

fn calculate_z(pos: &Vec3, angle: &Vec3) -> f32 {
    return pos.z * angle.x.cos() * angle.y.cos() - pos.y * angle.x.sin() * angle.y.cos()
        + pos.x * angle.y.sin();
}

// fn get_idx_to_update(pos: &Vec3, angle: &Vec3) -> (usize, f32) {
//     let x = calculate_x(&pos, &angle);
//     let y = calculate_y(&pos, &angle);
//     let z = calculate_z(&pos, &angle) + 1.0;
//     // let ooz = 1 / z;
//     let xp = WIDTH / 2 + HORIZONTL_OFFSET + K1 * ooz * x * 2;
//     let yp = HEIGHT / 2 + K1 * ooz * y;
//     return (xp + yp * WIDTH, ooz);
// // }

// //z_buffer: Vec<f32>,
// fn update_surface(idx: usize, buffer: Vec<u8>, ch: &u8) {
//     if idx < (&WIDTH * &HEIGHT) {
//         buffer[idx] = *ch;

//         // if ooz > z_buffer[idx] {
//         //     z_buffer[idx] = ooz;
//         //     buffer[idx] = *ch;
//         // }
//     }
// }

fn update_surface(
    pos: &Vec3,
    angle: &Vec3,
    buffer: &mut Vec<u8>,
    z_buffer: &mut Vec<f32>,
    ch: &u8,
) {
    let x = calculate_x(pos, angle);
    let y = calculate_y(pos, angle);
    let z = calculate_z(pos, angle) + 100.0;

    let ooz = 1.0 / z;
    let xp = WIDTH as f32 / 2.0 + HORIZONTL_OFFSET as f32 + (K1 as f32 * ooz * x * 2.0);
    let yp = HEIGHT as f32 / 2.0 + (K1 as f32 * ooz * y);

    let idx = xp as usize + (yp as usize * WIDTH);

    if idx < (WIDTH * HEIGHT) {
        if ooz > z_buffer[idx] {
            z_buffer[idx] = ooz;
            buffer[idx] = *ch;
        }
    }
}

// fn calculate_surface_pixel(
//     pos: &Vec3,
//     angle: &Vec3,
//     buffer: &mut Vec<u8>,
//     z_buffer: &mut Vec<f32>,
//     ch: &u8,
// ) {
//     let mx = calculate_x(pos, angle);
//     let my = calculate_y(pos, angle);
//     let mz = calculate_z(pos, angle) + 100f32;
//     let ooz = 1f32 / mz;
//     let xp = WIDTH as isize / 2 + HORIZONTL_OFFSET as isize + (50f32 * ooz * mx * 2f32) as isize;
//     let yp = HEIGHT as isize / 2 + (50f32 * ooz * my) as isize;
//     let idx = xp as usize + (yp as usize * WIDTH);
//     if idx < (WIDTH * HEIGHT) {
//         if ooz > z_buffer[idx] {
//             z_buffer[idx] = ooz;
//             buffer[idx] = *ch;
//         }
//     }
// }

// fn calculate_for_surface(pos: &Vec3, angle: &Vec3, ch: &u8) -> usize {
//     let idx = get_idx_to_update(&pos, &angle);
// }

fn render_x(buffer: &Vec<u8>) {
    print!("{}[1;1H", 27 as char);
    for k in 0..&WIDTH * &HEIGHT {
        if k % &WIDTH == 0 {
            println!();
        } else {
            print!("{}", *buffer.get(k).unwrap() as char)
        }
    }
}

// fn render() {
//     let bg = ' ' as u8;
//     let z = -1f32 * CUBE_WIDTH as f32;

//     let buffer = vec![0f32; HEIGHT * WIDTH];
//     let z_buffer = vec![' ' as u8; HEIGHT * WIDTH];

//     // self.renderer.buffer.fill(bg);
//     // self.renderer.z_buffer.fill(0f32);

//     loop {}
// }

fn main() {
    let bg = ' ' as u8;
    // let z = -1f32 * CUBE_WIDTH as f32;

    let mut z_buffer = vec![0f32; HEIGHT * WIDTH];
    let mut buffer = vec![' ' as u8; HEIGHT * WIDTH];

    let mut angle = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -20.0,
    };

    let sides = vec!['@', '$', 'O', '#', ';', '+'];
    let sides: Vec<u8> = sides.iter().map(|c| *c as u8).collect();

    loop {
        buffer.fill(bg);
        z_buffer.fill(0f32);
        let start = Instant::now();

        for cube_x in -20..20 {
            for cube_y in -20..20 {
                // let surface = &Vec3 {
                //     x: -cube_x as f32,
                //     y: cube_y as f32,
                //     z: 20.0,
                // };
                // update_surface(&surface, &angle, &mut buffer, &mut z_buffer, &sides[0])
                //calculate_surface_pixel(&surface, &angle, &mut buffer, &mut z_buffer, &ch);

                let sidex = vec![
                    Vec3 {
                        x: -cube_x as f32,
                        y: cube_y as f32,
                        z: 20.0,
                    },
                    Vec3 {
                        x: cube_x as f32,
                        y: cube_y as f32,
                        z: -20.0,
                    },
                    Vec3 {
                        x: -20.0,
                        y: cube_y as f32,
                        z: -cube_x as f32,
                    },
                    Vec3 {
                        x: 20.0,
                        y: cube_y as f32,
                        z: cube_x as f32,
                    },
                    Vec3 {
                        x: cube_x as f32,
                        y: -20.0,
                        z: -cube_y as f32,
                    },
                    Vec3 {
                        x: cube_x as f32,
                        y: 20.0,
                        z: cube_y as f32,
                    },
                ];

                for side_idx in 0..5 {
                    // let surface = &Vec3 {
                    //     x: -cube_x as f32,
                    //     y: cube_y as f32,
                    //     z: 20.0,
                    // };
                    update_surface(
                        &sidex[side_idx],
                        &angle,
                        &mut buffer,
                        &mut z_buffer,
                        &sides[side_idx],
                    )
                    //calculate_surface_pixel(&surface, &angle, &mut buffer, &mut z_buffer, &ch);
                }
            }
        }

        angle.x += 0.05;
        angle.y += 0.05;
        angle.z += 0.01;

        render_x(&buffer);

        let elapsed = start.elapsed();
        let frame_time = 1000 / 30;

        let wait_time =
            time::Duration::from_millis((frame_time - elapsed.as_millis() as i32) as u64);
        thread::sleep(wait_time);
    }
}
