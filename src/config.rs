use rlua::{FromLua, Lua, Table, ToLua, UserData, Value};
use sdl2::pixels::Color;

#[derive(Debug)]
pub struct Config {
    pub prompt_color: Color,
}

impl Config {
    fn new() -> Config {
        Config {
            prompt_color: Color::RGBA(255, 255, 255, 255),
        }
    }
}

impl<'lua> FromLua<'lua> for Config {
    fn from_lua(
        lua_value: rlua::prelude::LuaValue<'lua>,
        lua: rlua::prelude::LuaContext<'lua>,
    ) -> rlua::prelude::LuaResult<Self> {
        match lua_value {
            Value::Table(table) => {
                let lcolor = table.get::<_, LuaColor>("prompt_color")?;
                Ok(Config {
                    prompt_color: lcolor.to_color(),
                })
            }
            _ => panic!("Error on lua return"),
        }
    }
}

impl<'lua> ToLua<'lua> for Config {
    fn to_lua(self, lua: rlua::prelude::LuaContext<'lua>) -> rlua::prelude::LuaResult<Value<'lua>> {
        let table = lua.create_table()?;
        let color = self.prompt_color;
        let lcolor = LuaColor(color);
        table.set("prompt_color", lcolor)?;
        Ok(Value::Table(table))
    }
}

// Used to be able to have a SDL2 color as a lua value
#[derive(Debug)]
struct LuaColor(Color);

impl<'lua> ToLua<'lua> for LuaColor {
    fn to_lua(self, lua: rlua::prelude::LuaContext<'lua>) -> rlua::prelude::LuaResult<Value<'lua>> {
        let table = lua.create_table()?;
        table.set("r", self.0.r)?;
        table.set("g", self.0.g)?;
        table.set("b", self.0.b)?;
        table.set("a", self.0.a)?;
        Ok(Value::Table(table))
    }
}

impl LuaColor {
    fn to_color(self) -> Color {
        self.0
    }
}

impl<'lua> FromLua<'lua> for LuaColor {
    fn from_lua(
        lua_value: rlua::prelude::LuaValue<'lua>,
        lua: rlua::prelude::LuaContext<'lua>,
    ) -> rlua::prelude::LuaResult<Self> {
        let (r, g, b, a);
        match lua_value {
            Value::Table(table) => {
                r = table.get::<_, u8>("r")?;
                g = table.get::<_, u8>("g")?;
                b = table.get::<_, u8>("b")?;
                a = table.get::<_, u8>("a")?;
                Ok(LuaColor(Color::RGBA(r, g, b, a)))
            }
            _ => panic!("Bad color format"),
        }
    }
}

pub fn load_config(path: impl AsRef<str>) -> Config {
    let lua = Lua::new();
    let contents = std::fs::read(path.as_ref()).unwrap_or_else(|_| "".into());
    let config = Config::new();

    let res = lua.context(|ctx| {
        let globals = ctx.globals();

        globals.set("config", config)?;
        let color_func =
            ctx.create_function(|ctx, (a, r, g, b)| Ok(LuaColor(Color::RGBA(r, g, b, a))))?;
        globals.set("color", color_func)?;

        match ctx.load(&contents).set_name("config").unwrap().eval() {
            Ok(r) => r,
            Err(err) => {
                panic!("{}", err)
            }
        };
        let c = globals.get::<_, Config>("config");
        c
    });
    dbg!(res.unwrap())
}
