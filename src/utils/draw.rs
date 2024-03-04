use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{BlendMode, Canvas, Texture, TextureCreator},
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
    let rw = rect.w;
    let rh = rect.h;

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

    let tc = canvas.texture_creator();
    let mut tex = tc
        .create_texture_target(PixelFormatEnum::RGBA32, rw as u32, rh as u32)
        .unwrap();

    tex.set_blend_mode(BlendMode::Blend);
    canvas
        .with_texture_canvas(&mut tex, |c| {
            c.set_draw_color(Color::RGBA(0, 0, 0, 0));
            c.clear();
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
    //TODO(quadrado): Using draw_point for now which is slow. Change this to byte buffer and
    //texture
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

        // Every quadrant is composed of the two octants
        if qtd.contains(&0) {
            canvas.draw_point((cx + dx, cy + dy)).unwrap();
            canvas.draw_point((cx - dy, cy - dx)).unwrap();
        }
        if qtd.contains(&1) {
            canvas.draw_point((cx - dx, cy + dy)).unwrap();
            canvas.draw_point((cx + dy, cy - dx)).unwrap();
        }
        if qtd.contains(&2) {
            canvas.draw_point((cx - dx, cy - dy)).unwrap();
            canvas.draw_point((cx + dy, cy + dx)).unwrap();
        }
        if qtd.contains(&3) {
            canvas.draw_point((cx + dx, cy - dy)).unwrap();
            canvas.draw_point((cx - dy, cy + dx)).unwrap();
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

pub fn draw_filled_circle_quadrants(
    canvas: &mut Canvas<Window>,
    cx: i32,
    cy: i32,
    radius: i32,
    color: Color,
    quadrants: Option<Vec<usize>>,
) {
    let rw = radius * 2;
    let rh = radius * 2;
    let _cx = radius;
    let _cy = radius;
    let mut buf: Vec<u8> = vec![0; rw as usize * rh as usize * 4];

    let qtd = match quadrants {
        Some(list) => list,
        None => vec![0, 1, 2, 3],
    };

    for y in 0..rh {
        for x in 0..rw {
            let _x = x;
            let _y = y;
            let mut draw_point = false;
            // Every quadrant is composed of the two octants
            if qtd.contains(&1) && (_x < _cx && _y < _cy) {
                draw_point = true;
            } else if qtd.contains(&2) && (_x < _cx && _y >= _cy) {
                draw_point = true;
            } else if qtd.contains(&3) && (_x >= _cx && _y >= _cy) {
                draw_point = true;
            } else if qtd.contains(&0) && (_x >= _cx && _y < _cy) {
                draw_point = true;
            }

            if (_x - _cx).pow(2) + (_y - _cy).pow(2) <= radius.pow(2) && draw_point {
                buf[y as usize * (rw as usize * 4) + (x as usize * 4)] = color.r;
                buf[y as usize * (rw as usize * 4) + (x as usize * 4) + 1] = color.g;
                buf[y as usize * (rw as usize * 4) + (x as usize * 4) + 2] = color.b;
                buf[y as usize * (rw as usize * 4) + (x as usize * 4) + 3] = color.a;
            }
        }
    }

    let tc = canvas.texture_creator();
    let mut tex = tc
        .create_texture_target(PixelFormatEnum::RGBA32, rw as u32, rh as u32)
        .unwrap();
    tex.set_blend_mode(BlendMode::Blend);
    tex.update(None, buf.as_slice(), rw as usize * 4).unwrap();

    canvas
        .copy(
            &tex,
            None,
            Rect::new(cx - rw / 2, cy - rh / 2, rw as u32, rh as u32),
        )
        .unwrap();
}
pub fn draw_filled_rounded_rect(
    canvas: &mut Canvas<Window>,
    rect: Rect,
    radius: i32,
    color: Color,
) {
    let rw = rect.w;
    let rh = rect.h;

    let corners = [
        (radius, radius, 1),
        (radius, rh - radius - 1, 2),
        (rw - radius - 1, rh - radius - 1, 3),
        (rw - radius - 1, radius, 0),
    ];

    let tc = canvas.texture_creator();
    let mut tex = tc
        .create_texture_target(PixelFormatEnum::RGBA32, rw as u32, rh as u32)
        .unwrap();

    tex.set_blend_mode(BlendMode::Blend);
    canvas
        .with_texture_canvas(&mut tex, |c| {
            c.set_draw_color(Color::RGBA(0, 0, 0, 0));
            c.clear();
            c.set_draw_color(color);

            // Draw corner quadrants
            for (cx, cy, quadrant) in corners {
                draw_filled_circle_quadrants(c, cx, cy, radius, color, Some(vec![quadrant]));
            }

            // Draw filled rectangle filling the gaps
            c.fill_rect(Rect::new(
                0,
                radius,
                rw as u32,
                rh as u32 - 2 * radius as u32,
            ))
            .unwrap();
            c.fill_rect(Rect::new(
                radius,
                0,
                rw as u32 - 2 * radius as u32,
                rh as u32,
            ))
            .unwrap();
        })
        .unwrap();
    canvas.copy(&tex, None, rect).unwrap();
}

pub trait DrawExtensions {
    fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32);
    fn draw_circle_quadrants(
        &mut self,
        cx: i32,
        cy: i32,
        radius: i32,
        quadrants: Option<Vec<usize>>,
    );
    fn draw_rounded_rect(&mut self, rect: Rect, radius: i32);
    fn draw_filled_rounded_rect(&mut self, rect: Rect, radius: i32);
}

impl DrawExtensions for Canvas<Window> {
    fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32) {
        draw_circle(self, cx, cy, radius, self.draw_color())
    }

    fn draw_circle_quadrants(
        &mut self,
        cx: i32,
        cy: i32,
        radius: i32,
        quadrants: Option<Vec<usize>>,
    ) {
        draw_circle_quadrants(self, cx, cy, radius, self.draw_color(), quadrants)
    }

    fn draw_rounded_rect(&mut self, rect: Rect, radius: i32) {
        draw_rounded_rect(self, rect, radius, self.draw_color())
    }
    fn draw_filled_rounded_rect(&mut self, rect: Rect, radius: i32) {
        draw_filled_rounded_rect(self, rect, radius, self.draw_color())
    }
}
