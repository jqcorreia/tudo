use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    ttf::Font,
    video::{Window, WindowContext},
};

pub fn draw_string(s: String, canvas: &mut Canvas<Window>, font: &Font, fg: Color, x: i32, y: i32) {
    let tc = canvas.texture_creator();
    let surf = font.render(&s).blended(fg).unwrap();
    let texture = tc.create_texture_from_surface(surf).unwrap();

    let query = texture.query();
    let (w, h) = (query.width, query.height);
    let rect = Rect::new(x, y, w, h);
    canvas.copy(&texture, None, Some(rect)).unwrap();
}

pub fn draw_string_texture<'a>(
    s: String,
    tc: &'a TextureCreator<WindowContext>,
    font: &Font,
    fg: Color,
) -> Texture<'a> {
    let surf = font.render(&s).blended(fg).unwrap();
    let texture = tc.create_texture_from_surface(surf).unwrap();
    texture
}

pub fn draw_rounded_rect(canvas: &mut Canvas<Window>, rect: Rect, radius: i32, color: Color) {
    let rx = rect.x;
    let ry = rect.y;
    let rw = rect.w;
    let rh = rect.h;
    let mut buf: Vec<u8> = vec![0; (rw * rh * 4) as usize];
    let corners = [
        (radius, radius, 1),
        (radius, rh - radius - 1, 2),
        (rw - radius - 1, rh - radius - 1, 3),
        (rw - radius - 1, radius, 0),
    ];

    let rect_lines = [
        (0, radius, 0, rh - radius),
        (radius, rh - 1, rw - radius, rh - 1),
        (rw - 1, radius, rw - 1, rh - radius - 1),
        (radius, 0, rw - radius, 0),
    ];

    // for (cx, cy) in _corners {
    //     draw_circle(canvas, cx, cy, radius, Color::RGBA(255, 255, 255, 255));
    // for y in 0..rh {
    //     for x in 0..rw {
    //         let mut draw = false;
    //         if quadrant == 0 && x < cx as usize && y < cy as usize {
    //             draw = true
    //         }
    //         if quadrant == 1 && x < cx as usize && y > cy as usize {
    //             draw = true
    //         }
    //         if quadrant == 2 && x > cx as usize && y > cy as usize {
    //             draw = true
    //         }
    //         if quadrant == 3 && x > cx as usize && y < cy as usize {
    //             draw = true
    //         }

    //         if draw {
    //             let _x = x as i32;
    //             let _y = y as i32;
    //             if ((_x - cx as i32).pow(2) + (_y - cy as i32).pow(2)) - (radius.pow(2) as i32)
    //                 < -10
    //             {
    //                 let blue = 255.0;

    //                 buf[y * (rw * 4) + (x * 4)] = 0;
    //                 buf[y * (rw * 4) + (x * 4) + 1] = 0;
    //                 buf[y * (rw * 4) + (x * 4) + 2] = blue as u8;
    //                 buf[y * (rw * 4) + (x * 4) + 3] = 255;
    //             }
    //         }
    //     }
    // }
    // }

    let tc = canvas.texture_creator();
    let mut tex = tc
        .create_texture_target(PixelFormatEnum::RGBA32, rw as u32, rh as u32)
        .unwrap();

    canvas
        .with_texture_canvas(&mut tex, |c| {
            c.set_draw_color(color);
            for line in rect_lines {
                let (sx, sy, ex, ey) = line;
                c.draw_line((sx as i32, sy as i32), (ex as i32, ey as i32))
                    .unwrap();
            }
            for (cx, cy, quadrant) in corners {
                draw_circle_quadrants(c, cx, cy, radius, color, Some(vec![quadrant]));
            }
        })
        .unwrap();
    canvas.copy(&tex, None, rect).unwrap();
}

fn error_radius(cx: i32, cy: i32, x: i32, y: i32, radius: i32) -> i32 {
    // Simply calculate for a given point the distance to the circle of center 'c' and radius
    // 'radius'
    (((x - cx as i32).pow(2) + (y - cy as i32).pow(2)) - (radius.pow(2) as i32)).abs()
}
pub fn draw_circle(canvas: &mut Canvas<Window>, cx: i32, cy: i32, radius: i32, color: Color) {
    draw_circle_quadrants(canvas, cx, cy, radius, color, None);
}

pub fn draw_circle_quadrants(
    canvas: &mut Canvas<Window>,
    cx: i32,
    cy: i32,
    radius: i32,
    color: Color,
    quadrants: Option<Vec<usize>>,
) {
    // Function based on the midpoint circle algorithm
    // https://en.wikipedia.org/wiki/Midpoint_circle_algorithm
    // Quadrants:
    //   1    |   0
    //        |
    // ...............
    //        |
    //   2    |   3
    let mut x = cx + radius;
    let mut y = cy;

    let mut dx = x - cx;
    let mut dy = y - cy;
    canvas.set_draw_color(color);

    let qtd = match quadrants {
        Some(list) => list,
        None => vec![0, 1, 2, 3],
    };

    while dx.abs() > dy.abs() {
        dx = x - cx;
        dy = y - cy;
        let dx2 = dx * 2;
        let dy2 = dy * 2;

        // Every quadrant is composed of the two octants
        if qtd.contains(&0) {
            canvas.draw_point((x, y)).unwrap();
            canvas.draw_point((y - dy2, x - dx2)).unwrap();
        }
        if qtd.contains(&1) {
            canvas.draw_point((x - dx2, y)).unwrap();
            canvas.draw_point((y, x - dx2)).unwrap();
        }

        if qtd.contains(&2) {
            canvas.draw_point((x - dx2, y - dy2)).unwrap();
            canvas.draw_point((y, x)).unwrap();
        }
        if qtd.contains(&3) {
            canvas.draw_point((x, y - dy2)).unwrap();
            canvas.draw_point((y - dy2, x)).unwrap();
        }

        // Use error radius to decide if x should move or not
        // Keep in mind we are using the (0, r) going clockwise octant as reference so the y always
        // decreases and maybe the x decreases. Error radius help us decide which is better
        if error_radius(cx, cy, x, y - 1, radius) < error_radius(cx, cy, x - 1, y - 1, radius) {
            x = x;
        } else {
            x = x - 1;
        }
        y -= 1;
    }
}
