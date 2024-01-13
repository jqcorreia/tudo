use rlua::{Lua, Table};

use crate::sources::Action;

use super::{RunAction, Source, SourceItem};

pub struct LuaSource {
    pub items: Vec<SourceItem>,
    pub source: String,
}

impl LuaSource {
    pub fn new(source: String) -> LuaSource {
        LuaSource {
            items: Vec::new(),
            source,
        }
    }
}
impl Source for LuaSource {
    fn items(&self) -> &Vec<SourceItem> {
        &self.items
    }

    fn calculate_items(&mut self) {
        let mut items = Vec::<SourceItem>::new();
        let lua = Lua::new();

        lua.context(|ctx| {
            let script = std::fs::read(&self.source).unwrap();
            let res: Vec<Table> = ctx.load(&script).set_name("teste").unwrap().eval().unwrap();

            for v in res.iter() {
                let title: String = v.get("title".to_string()).unwrap();
                let icon: Option<String> = v.get("icon").unwrap();
                let action: Table = v.get("action").unwrap();
                let action_type: String = action.get("type").unwrap();

                match action_type.as_str() {
                    "run" => items.push(SourceItem {
                        title: title.clone(),
                        icon: icon.clone(),
                        action: Action::Run(RunAction {
                            path: action.get("path").unwrap(),
                            clip_output: false,
                            exit_after: true,
                        }),
                    }),
                    _ => (),
                };
                dbg!(&title, &icon, action);
            }
        });
        self.items = items;
    }
}

#[cfg(test)]
mod tests {
    use crate::sources::Source;

    use super::LuaSource;
    #[test]
    fn test_basic_lua() {
        let mut source = LuaSource::new("plugins/pass.lua".to_string());
        source.calculate_items();
    }
}
