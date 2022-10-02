// #![feature(generators, generator_trait)]

const CUBE_WIDTH: i32 = 15;
const HORIZONTL_OFFSET: i32 = -0 * CUBE_WIDTH;
const WIDTH: usize = 160;
const HEIGHT: usize = 44;
const K1: usize = 40;

use std::{thread, time, time::Instant};

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct CubeSide {
    pub pos: Vec3,
    pub ch: u8,
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
    cube_side: &CubeSide,
    angle: &Vec3,
    // buffer: &mut Vec<u8>,
    // z_buffer: &mut Vec<f32>,
) -> (usize, f32) {
    let x = calculate_x(&cube_side.pos, angle);
    let y = calculate_y(&cube_side.pos, angle);
    let z = calculate_z(&cube_side.pos, angle) + 100.0;

    let ooz = 1.0 / z;
    let xp = WIDTH as f32 / 2.0 + HORIZONTL_OFFSET as f32 + (K1 as f32 * ooz * x * 2.0);
    let yp = HEIGHT as f32 / 2.0 + (K1 as f32 * ooz * y);

    let idx = xp as usize + (yp as usize * WIDTH);

    // if idx < (WIDTH * HEIGHT) {
    //     if ooz > z_buffer[idx] {
    //         z_buffer[idx] = ooz;
    //         buffer[idx] = cube_side.ch;
    //     }
    // }
    return (idx, ooz);
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

fn create_side(x: f32, y: f32, z: f32, ch: u8) -> CubeSide {
    return CubeSide {
        pos: Vec3 { x: x, y: y, z: z },
        ch: ch,
    };
}

fn update_buffers(idx: usize, ooz: f32, ch: u8, buffer: &mut Vec<u8>, z_buffer: &mut Vec<f32>) {
    if idx < (WIDTH * HEIGHT) {
        if ooz > z_buffer[idx] {
            z_buffer[idx] = ooz;
            buffer[idx] = ch;
        }
    }
}

fn create_sides(x: f32, y: f32, z: f32) -> Vec<CubeSide> {
    let sides = vec![
        // oposite sides 1
        create_side(x, y, -z, '@' as u8),
        create_side(-x, y, z, '$' as u8),
        // oposite sides 2
        create_side(z, y, x, '¤' as u8),
        create_side(-z, y, -x, '#' as u8),
        // oposite sides 3
        create_side(x, -z, -y, '=' as u8),
        create_side(x, z, y, '+' as u8),
    ];
    return sides;
}

// fn get_buffer_updates(sides: Vec<CubeSide>) {
//     for side in sides {
//         update_surface(&side, &angle)
//         // update_surface(&side, &angle, &mut buffer, &mut z_buffer)
//         //calculate_surface_pixel(&surface, &angle, &mut buffer, &mut z_buffer, &ch);
//     }
// }

// fn get_cube_stuff() -> (usize, f32, u8)  {
//     for cube_x in -CUBE_WIDTH..CUBE_WIDTH {
//         for cube_y in -CUBE_WIDTH..CUBE_WIDTH {
//             let cube_sides = create_sides(cube_x as f32, cube_y as f32, cube_z);

//             for side in cube_sides {
//                 let (idx, ooz) = update_surface(&side, &angle);
//                 yield (idx, ooz, side.ch)
//                 // update_surface(&side, &angle, &mut buffer, &mut z_buffer)
//                 //calculate_surface_pixel(&surface, &angle, &mut buffer, &mut z_buffer, &ch);
//             }
//     }
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

    let cube_z: f32 = 20.0;

    loop {
        buffer.fill(bg);
        z_buffer.fill(0f32);
        let start = Instant::now();

        for cube_x in -CUBE_WIDTH..CUBE_WIDTH {
            for cube_y in -CUBE_WIDTH..CUBE_WIDTH {
                let cube_sides = create_sides(cube_x as f32, cube_y as f32, cube_z);

                for side in cube_sides {
                    let (idx, ooz) = update_surface(&side, &angle);

                    update_buffers(idx, ooz, side.ch, &mut buffer, &mut z_buffer)
                    // yield (idx, ooz, side.ch)
                    // update_surface(&side, &angle, &mut buffer, &mut z_buffer)
                    //calculate_surface_pixel(&surface, &angle, &mut buffer, &mut z_buffer, &ch);
                }
            }
        }

        // for cube_x in -CUBE_WIDTH..CUBE_WIDTH {
        //     for cube_y in -CUBE_WIDTH..CUBE_WIDTH {
        //         let cube_sides = create_sides(cube_x as f32, cube_y as f32, cube_z);

        //         for side in cube_sides {
        //             let (idx, ooz) = update_surface(&side, &angle);
        //             yield (idx, ooz, ch)
        //             // update_surface(&side, &angle, &mut buffer, &mut z_buffer)
        //             //calculate_surface_pixel(&surface, &angle, &mut buffer, &mut z_buffer, &ch);
        //         }

        //         // for side_idx in 0..6 {
        //         //     update_surface(&cube_sides[side_idx], &angle, &mut buffer, &mut z_buffer)
        //         //     //calculate_surface_pixel(&surface, &angle, &mut buffer, &mut z_buffer, &ch);
        //         // }
        //     }
        // }

        // spin rotation
        angle.x += 0.06;
        angle.y += 0.03;
        angle.z += 0.01;

        render_x(&buffer);

        let elapsed = start.elapsed();
        let frame_time = 1000 / 30;

        let wait_time =
            time::Duration::from_millis((frame_time - elapsed.as_millis() as i32) as u64);
        thread::sleep(wait_time);
    }
}
