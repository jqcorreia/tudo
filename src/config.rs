use std::collections::HashMap;

use mlua::{IntoLua, Lua, LuaSerdeExt, Value};
use sdl2::pixels::Color;

use serde::{de::Visitor, ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub font_file: String,
    pub cursor_blink: bool,
    #[serde(serialize_with = "serialize_color")]
    #[serde(deserialize_with = "deserialize_color")]
    pub prompt_color: Color,
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
            // println!("I'm here");
            // let _ = map;
            Ok(Color::RGBA(255, 0, 0, 255))
        }
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("Expects a map with 4 keys 'r', 'g', 'b', 'a'")
        }
    }

    des.deserialize_map(ColorVisitor)?;
    Ok(Color::RGBA(255, 1, 1, 255))
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
        }
    }
}

// // impl<'lua> ToLua<'lua> for Config {
// //     fn to_lua(self, lua: rlua::prelude::LuaContext<'lua>) -> rlua::prelude::LuaResult<Value<'lua>> {
// //         let table = lua.create_table()?;

// //         let color = self.prompt_color;
// //         let lcolor = LuaColor(color);
// //         table.set("prompt_color", lcolor)?;
// //         table.set("font_file", self.font_file)?;

// //         Ok(Value::Table(table))
// //     }
// // }

// // Used to be able to have a SDL2 color as a lua value
// #[derive(Debug)]
// struct LuaColor(Color);

// impl LuaColor {
//     fn to_color(self) -> Color {
//         self.0
//     }
// }

// impl<'lua> IntoLua<'lua> for LuaColor {
//     fn into_lua(self, lua: &'lua Lua) -> mlua::prelude::LuaResult<mlua::prelude::LuaValue<'lua>> {
//         let table = lua.create_table()?;
//         table.set("r", self.0.r)?;
//         table.set("g", self.0.g)?;
//         table.set("b", self.0.b)?;
//         table.set("a", self.0.a)?;
//         Ok(Value::Table(table))
//     }
// }

// impl<'lua> FromLua<'lua> for LuaColor {
//     fn from_lua(
//         lua_value: rlua::prelude::LuaValue<'lua>,
//         _lua: rlua::prelude::LuaContext<'lua>,
//     ) -> rlua::prelude::LuaResult<Self> {
//         let (r, g, b, a);
//         match lua_value {
//             Value::Table(table) => {
//                 r = table.get::<_, u8>("r")?;
//                 g = table.get::<_, u8>("g")?;
//                 b = table.get::<_, u8>("b")?;
//                 a = table.get::<_, u8>("a")?;
//                 Ok(LuaColor(Color::RGBA(r, g, b, a)))
//             }
//             _ => panic!("Bad color format"),
//         }
//     }
// }

pub fn load_config(path: impl AsRef<str>) -> Config {
    let lua = Lua::new();
    let contents = std::fs::read(path.as_ref()).unwrap_or_else(|_| "".into());
    let config = Config::new();

    let globals = lua.globals();

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

    lua.load(&contents).set_name("test").exec().unwrap();
    match lua.load(&contents).set_name("config").eval() {
        Ok(r) => dbg!(r),
        Err(err) => {
            panic!("{}", err)
        }
    };

    // let c = lua.from_value(globals.get("tudo").unwrap()).unwrap();
    // let c = globals
    //     .get::<_, HashMap<String, u8>>("tudo.prompt_color")
    //     .unwrap();
    config
}
