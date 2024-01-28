use std::process::Command;

use reqwest::header::{self, HeaderMap};

use crate::sources::actions::{Action, RunAction, TmuxAction};

use super::{Source, SourceItem};

pub struct Notion {
    calculated_items: Vec<SourceItem>,
}

impl Notion {
    pub fn new() -> Notion {
        Notion {
            calculated_items: Vec::new(),
        }
    }
}
impl Source for Notion {
    fn calculate_items(&mut self) {
        // let mut res: Vec<SourceItem> = Vec::new();

        // let output = Command::new("sh").args(["-c", "tmux ls"]).output();
        // let ot = String::from_utf8(output.unwrap().stdout).unwrap();

        // for line in ot.lines() {
        //     let session_name = line.split(":").next().unwrap();
        //     res.push(SourceItem {
        //         title: line.to_string(),
        //         icon: None,
        //         action: Action::Notion(NotionAction {
        //             session: session_name.to_string(),
        //         }),
        //     });
        //     dbg!(line);
        // }
        // self.calculated_items = res;
        let token = "secret_ckkyjJQzMtH9tIl2edJeBqfNSf1prVfhStWeM2NF0b9";
        let mut headers = HeaderMap::new();
        headers.insert("Notion-Version", "2022-06-28".parse().unwrap());
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let client = reqwest::blocking::Client::new();
        let req = client
            .get("https://api.notion.com/v1/pages/6af37c241f464866bb3cadbc076dd326")
            .headers(headers);

        let res = req.send().unwrap();
        dbg!(res.text().unwrap());
    }

    fn items(&self) -> &Vec<SourceItem> {
        return &self.calculated_items;
    }
}
