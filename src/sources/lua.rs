use std::collections::HashMap;

use mlua::{Error, IntoLua, Lua, LuaSerdeExt, Table, UserData};
use ureq::serde_json;

use crate::sources::Action;

use super::{actions::PassSecretAction, actions::RunAction, Source, SourceItem};

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

// Struct that wraps a serde_jso::Value and implements the IntoLua trait
// FIXME(quadrado) only supports root objects
struct LuaJSON {
    value: serde_json::Value,
}
impl<'lua> IntoLua<'lua> for LuaJSON {
    fn into_lua(self, lua: &'lua Lua) -> mlua::prelude::LuaResult<mlua::prelude::LuaValue<'lua>> {
        match self.value {
            serde_json::Value::Object(obj) => Ok(lua.to_value(&obj)?),
            _ => panic!("Unsupported JSON return type"),
        }
    }
}

fn http_get(
    _lua: &Lua,
    req_args: (String, HashMap<String, String>),
) -> Result<impl IntoLua, Error> {
    let mut req = ureq::get(&req_args.0);

    for (k, v) in req_args.1 {
        req = req.set(&k, &v);
    }

    let contents: serde_json::Value;
    match req.call() {
        Ok(response) => contents = response.into_json().unwrap(),
        Err(err) => return Err(mlua::Error::RuntimeError(err.to_string())),
    }
    Ok(LuaJSON { value: contents })
}

fn load_json_file(_lua: &Lua, path: String) -> Result<impl IntoLua, Error> {
    let res = std::fs::read(path).unwrap();

    let contents = serde_json::from_slice(res.as_slice()).unwrap();
    Ok(LuaJSON { value: contents })
}

// Set some utility functions
fn setup(lua: &Lua) {
    let http_get = lua.create_function(http_get).unwrap();
    let load_json_file = lua.create_function(load_json_file).unwrap();

    lua.globals().set("http_get", http_get).unwrap();
    lua.globals().set("open_json", load_json_file).unwrap();
}

impl Source for LuaSource {
    fn is_async(&self) -> bool {
        false
    }

    fn generate_items(&self) -> Vec<SourceItem> {
        let mut items = Vec::<SourceItem>::new();

        // Rewrite this with serde!!
        let lua = Lua::new();
        let res: Vec<Table>;

        let _script = std::fs::read(&self.source);
        let script;
        if _script.is_err() {
            // Return empty string if error reading script
            return vec![];
        } else {
            script = _script.unwrap();
        }

        setup(&lua);

        res = match lua.load(&script).set_name("teste").eval() {
            Ok(r) => r,
            Err(err) => {
                println!("{}", err);
                return vec![];
            }
        };

        dbg!(&res);
        for v in res.iter() {
            let title: String = v.get("title".to_string()).unwrap();
            let icon: Option<String> = v.get("icon").unwrap();
            let action_table: Table = v.get("action").unwrap();
            let action_type: String = action_table.get("type").unwrap();

            let action: Box<dyn Action + Send> = match action_type.as_str() {
                "run" => Box::new(RunAction {
                    path: action_table.get("path").unwrap(),
                    clip_output: false,
                    exit_after: true,
                }),
                "secret" => Box::new(PassSecretAction {
                    secret_name: action_table.get("secret_name").unwrap(),
                }),
                _ => panic!("Unsupported lua action type"),
            };
            items.push(SourceItem {
                title: title.clone(),
                icon: icon.clone(),
                action: action.clone(),
            });
        }
        items
    }
}
