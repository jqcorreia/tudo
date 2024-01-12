use rlua::Lua;

use super::{Source, SourceItem};

pub struct LuaSource {
    pub items: Vec<SourceItem>,
}

impl LuaSource {
    fn new() -> LuaSource {
        LuaSource { items: Vec::new() }
    }
}
impl Source for LuaSource {
    fn items(&self) -> &Vec<SourceItem> {
        &self.items
    }

    fn calculate_items(&mut self) {
        let lua = Lua::new();

        lua.context(|ctx| {
            let globals = ctx.globals();

            globals.set("foo", "bar");

            let s = r#"
            print("from lua " .. foo)
            return 1000
            "#;
            let res: u32 = ctx.load(s).set_name("teste").unwrap().eval().unwrap();
            println!("from rust {:?}", res);
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::sources::Source;

    use super::LuaSource;
    #[test]
    fn test_basic_lua() {
        let mut source = LuaSource::new();
        source.calculate_items();
    }
}
