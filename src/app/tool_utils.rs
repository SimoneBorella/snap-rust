use eframe::egui::{Pos2, Vec2};
use image::DynamicImage;
use imageproc::drawing::draw_filled_circle_mut;

#[derive(PartialEq)]
pub enum Tool {
    None,
    Pen,
    Crop
}

pub fn get_line_points(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let (mut x, mut y) = (x0, y0);
    let mut err = dx - dy;

    loop {
        result.push((x, y));

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }

    result
}



pub fn draw_line(img: &mut DynamicImage, start:(f32, f32), end:(f32, f32), t: i32, color: [u8; 4]) {
    let segment = get_line_points(start.0 as i32, start.1 as i32, end.0 as i32, end.1 as i32);
    for point in segment {
        draw_filled_circle_mut(img, (point.0, point.1), t, color.into());
    }
}



pub fn linear_to_srgb(lin_rgb: [f32; 3]) -> [u8; 3] {
    let mut srgb: [u8; 3] = [0; 3];

    for i in 0..3 {
        let v = lin_rgb[i];
        if v <= 0.0 {
            srgb[i] = 0 as u8;
        } else if v >= 1.0 {
            srgb[i] = 255 as u8;
        } else {
            srgb[i] = (v.powf(1.0 / 2.2) * 255.0 + 0.5) as u8; // Corrected formula
        }
    }

    return srgb;
}

pub fn get_real_image_pos(pos: Pos2, image_rect_size: Vec2, real_image_size: [usize; 2]) -> Pos2{
    return Pos2::new(pos[0]*real_image_size[0] as f32/image_rect_size[0], pos[1]*real_image_size[1] as f32/image_rect_size[1]);
}