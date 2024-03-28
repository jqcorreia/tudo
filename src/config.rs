use std::collections::HashMap;

use mlua::{Lua, LuaSerdeExt};
use sdl2::pixels::Color;

use serde::{
    de::{Error, Visitor},
    ser::SerializeMap,
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub font_file: String,
    pub cursor_blink: bool,
    #[serde(serialize_with = "serialize_color")]
    #[serde(deserialize_with = "deserialize_color")]
    pub prompt_color: Color,
    pub frame_lock: bool,
}

fn serialize_color<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(4))?;
    map.serialize_entry("r", &color.r)?;
    map.serialize_entry("g", &color.g)?;
    map.serialize_entry("b", &color.r)?;
    map.serialize_entry("a", &color.a)?;
    map.end()
}

fn deserialize_color<'de, D>(des: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    struct ColorVisitor;
    impl<'de> Visitor<'de> for ColorVisitor {
        type Value = Color;

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let mut r: u8 = 0;
            let mut g: u8 = 0;
            let mut b: u8 = 0;
            let mut a: u8 = 0;
            while let Some((key, value)) = map.next_entry::<String, u8>().unwrap() {
                match key.as_str() {
                    "r" => r = value,
                    "g" => g = value,
                    "b" => b = value,
                    "a" => a = value,
                    _ => return Err(Error::custom("Unknown key")),
                }
            }
            Ok(Color::RGBA(r, g, b, a))
        }
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("Expects a map with 4 keys 'r', 'g', 'b', 'a'")
        }
    }

    Ok(des.deserialize_map(ColorVisitor))?
}

impl Config {
    fn new() -> Config {
        Default::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            font_file: "/usr/share/fonts/noto/NotoSans-Regular.ttf".to_string(),
            cursor_blink: true,
            prompt_color: Color::RGBA(255, 255, 255, 255),
            frame_lock: false,
        }
    }
}

pub fn set_globals(lua: &Lua) {
    let globals = lua.globals();
    let config = Config::new();

    globals.set("tudo", lua.to_value(&config).unwrap()).unwrap();
    let color_func = lua.create_function(|_ctx, (r, g, b, a): (u8, u8, u8, u8)| {
        let mut hm: HashMap<String, u8> = HashMap::new();
        hm.insert("r".to_string(), r);
        hm.insert("g".to_string(), g);
        hm.insert("b".to_string(), b);
        hm.insert("a".to_string(), a);

        Ok(hm)
    });

    globals.set("color", color_func.unwrap()).unwrap();
}

pub fn load_config(path: impl AsRef<str>) -> Config {
    let lua = Lua::new();
    let contents = std::fs::read(path.as_ref()).unwrap_or_else(|_| "".into());

    let globals = lua.globals();

    set_globals(&lua);

    match lua.load(&contents).set_name("config").eval() {
        Ok(r) => dbg!(r),
        Err(err) => {
            panic!("{}", err)
        }
    };
    let c = lua.from_value(globals.get("tudo").unwrap()).unwrap();
    c
}
