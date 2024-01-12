use sdl2::{
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{BlendMode, Canvas, Texture, TextureCreator},
    ttf::Font,
    video::{Window, WindowContext},
};

use std::collections::HashMap;

use crate::BLEND;
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct TextureInfo {
    pub font_name: String,
    pub font_size: i32,
    pub fg: Color,
    pub ch: char,
}

pub struct FontAtlas<'fa> {
    pub atlas: HashMap<TextureInfo, Texture<'fa>>,
    tc: &'fa TextureCreator<WindowContext>,
}

impl<'fa> FontAtlas<'fa> {
    pub fn new(tc: &'fa TextureCreator<WindowContext>) -> Self {
        FontAtlas {
            atlas: HashMap::new(),
            tc,
        }
    }
    pub fn generate_new_texture(&mut self, font: &Font, te: TextureInfo) -> &Texture {
        let surf = font.render_char(te.ch as char).blended(te.fg).unwrap();

        let tex: Texture<'fa> = self.tc.create_texture_from_surface(surf).unwrap();

        self.atlas.insert(te.clone(), tex);
        self.atlas.get(&te).unwrap()
    }

    pub fn draw_char(&mut self, font: &Font, ch: char, fg: Color) -> &Texture {
        let font_name = font.face_family_name().unwrap();
        let te = TextureInfo {
            font_name,
            fg,
            ch,
            font_size: font.height(),
        };

        if self.atlas.get(&te).is_none() {
            self.generate_new_texture(font, te)
        } else {
            self.atlas.get(&te).unwrap()
        }
    }

    pub fn draw_string(
        &mut self,
        s: String,
        _canvas: &mut Canvas<Window>,
        font: &Font,
        fg: Color,
    ) -> Texture {
        let surf = font.render(&s).blended(fg).unwrap();
        let final_tex = self.tc.create_texture_from_surface(surf).unwrap();
        return final_tex;
    }

    pub fn draw_string_atlas(
        &mut self,
        s: String,
        canvas: &mut Canvas<Window>,
        font: &Font,
        fg: Color,
    ) -> Texture {
        let mut x = 0;
        // let mut y = 0;

        let mut tw = 0;
        let mut th = 0;

        //FIXME this is stupid has we need to traverse the string twice FIXME
        for c in s.chars() {
            let ch = c as char;
            let t = self.draw_char(font, ch, fg);
            tw += t.query().width;
            th = t.query().height;
        }

        let mut final_tex = self
            .tc
            .create_texture_target(PixelFormatEnum::RGBA8888, tw, th)
            .unwrap();

        canvas
            .with_texture_canvas(&mut final_tex, |texture_canvas| {
                // if blend {
                //     texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 0));
                //     texture_canvas.clear();
                // }
                for c in s.chars() {
                    let ch = c as char;
                    let t = self.draw_char(font, ch, fg);

                    texture_canvas
                        .copy(&t, None, Rect::new(x, 0, t.query().width, t.query().height))
                        .unwrap();
                    x += t.query().width as i32;
                }
            })
            .unwrap();

        final_tex
    }
}
