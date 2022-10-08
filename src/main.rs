const CUBE_SIDE_WIDTH: i32 = 20; // decrease to add space between panels
const HORIZONTL_OFFSET: i32 = -0 * CUBE_SIDE_WIDTH; // set the horizontal location
const WIDTH: usize = 160;
const HEIGHT: usize = 44;
const K1: usize = 40;
const CAMERA_DISTANCE: f32 = 80.0; // pretty much the size?
const X_AXL_ROT_SPEED: f32 = 0.05; // spinning up
const Y_AXL_ROT_SPEED: f32 = 0.05; // spinning side
const Z_AXL_ROT_SPEED: f32 = 0.02; // turning
const START_TILT_ANGLE: f32 = 100.0;

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

fn get_rotation_update(cube_side: &CubeSide, angle: &Vec3) -> (usize, f32) {
    let x = calculate_x(&cube_side.pos, angle);
    let y = calculate_y(&cube_side.pos, angle);
    let z = calculate_z(&cube_side.pos, angle) + CAMERA_DISTANCE;

    let ooz = 1.0 / z;
    let xp = WIDTH as f32 / 2.0 + HORIZONTL_OFFSET as f32 + (K1 as f32 * ooz * x * 2.0);
    let yp = HEIGHT as f32 / 2.0 + (K1 as f32 * ooz * y);

    let idx = xp as usize + (yp as usize * WIDTH);

    return (idx, ooz);
}

fn create_side(x: f32, y: f32, z: f32, ch: u8) -> CubeSide {
    return CubeSide {
        pos: Vec3 { x: x, y: y, z: z },
        ch: ch,
    };
}

fn create_sides(x: f32, y: f32, z: f32) -> Vec<CubeSide> {
    let sides = vec![
        // oposite sides n.1
        create_side(x, y, -z, '@' as u8),
        create_side(-x, y, z, '$' as u8),
        // oposite sides n.2
        create_side(z, y, x, '¤' as u8),
        create_side(-z, y, -x, '#' as u8),
        // oposite sides n.3
        create_side(x, -z, -y, '=' as u8),
        create_side(x, z, y, '+' as u8),
    ];
    return sides;
}

fn update_buffers(idx: usize, ooz: f32, ch: u8, buffer: &mut Vec<u8>, z_buffer: &mut Vec<f32>) {
    if idx < (WIDTH * HEIGHT) {
        if ooz > z_buffer[idx] {
            z_buffer[idx] = ooz;
            buffer[idx] = ch;
        }
    }
}

fn render(buffer: &Vec<u8>) {
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
    let mut z_buffer = vec![0f32; HEIGHT * WIDTH];
    let mut buffer = vec![' ' as u8; HEIGHT * WIDTH];
    let mut angle = Vec3 {
        x: 0.0,
        y: 0.0,
        z: START_TILT_ANGLE,
    };

    loop {
        buffer.fill(bg);
        z_buffer.fill(0f32);
        let start = Instant::now();

        for cube_x in -CUBE_SIDE_WIDTH..CUBE_SIDE_WIDTH {
            for cube_y in -CUBE_SIDE_WIDTH..CUBE_SIDE_WIDTH {
                let cube_sides = create_sides(cube_x as f32, cube_y as f32, CUBE_SIDE_WIDTH as f32);
                for side in cube_sides {
                    let (idx, ooz) = get_rotation_update(&side, &angle);
                    update_buffers(idx, ooz, side.ch, &mut buffer, &mut z_buffer)
                }
            }
        }

        angle.x += X_AXL_ROT_SPEED;
        angle.y += Y_AXL_ROT_SPEED;
        angle.z += Z_AXL_ROT_SPEED;

        render(&buffer);

        let elapsed = start.elapsed();
        let frame_time = 1000 / 20;

        let wait_time =
            time::Duration::from_millis((frame_time - elapsed.as_millis() as i32) as u64);
        thread::sleep(wait_time);
    }
}
