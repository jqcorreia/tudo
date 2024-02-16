use sdl2::{
    pixels::Color,
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
