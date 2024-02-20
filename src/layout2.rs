use crate::{
    components::traits::UIComponent,
    layout::{Container, Layout, Leaf, SizeTypeEnum, Split},
};

pub enum SplitType {
    Horizontal,
    Vertical,
}
pub struct LayoutBuilder {
    gap: usize,
    root: Option<Container>,
    cur_split_id: u32,
}

impl LayoutBuilder {
    pub fn new() -> Self {
        LayoutBuilder {
            root: None,
            gap: 0,
            cur_split_id: 0,
        }
    }

    pub fn with_gap(mut self, gap: usize) -> Self {
        self.gap = gap;
        self
    }

    fn get_split(node: &mut Container, id: u32) -> Option<&mut Container> {
        match node {
            Container::HSplit(split) => {
                if split.id == id {
                    return Some(node);
                } else {
                    for child in split.children.iter_mut() {
                        LayoutBuilder::get_split(child, id);
                    }
                }
            }
            Container::VSplit(split) => {
                if split.id == id {
                    return Some(node);
                } else {
                    for child in split.children.iter_mut() {
                        LayoutBuilder::get_split(child, id);
                    }
                }
            }
            _ => (),
        }
        None
    }

    pub fn add(&mut self, comp: Box<dyn UIComponent>) {
        match &mut self.root {
            None => {
                self.root = Some(Container::Leaf(Leaf {
                    size: 100,
                    size_type: SizeTypeEnum::Percent,
                    component: comp,
                }))
            }
            Some(root) => {
                let target_split = LayoutBuilder::get_split(root, self.cur_split_id);
                let container = Container::Leaf(Leaf {
                    size: 200,
                    size_type: SizeTypeEnum::Percent,
                    component: comp,
                });
                match target_split {
                    Some(Container::HSplit(split)) => split.children.push(container),
                    Some(Container::VSplit(split)) => split.children.push(container),
                    _ => panic!("Container not found"),
                }
            }
        };
    }

    pub fn add_split(&mut self, split_type: SplitType) {
        let prev_split = self.cur_split_id;
        self.cur_split_id += 1;
        let container = match split_type {
            SplitType::Horizontal => Container::HSplit(Split {
                id: self.cur_split_id,
                children: vec![],
            }),
            SplitType::Vertical => Container::VSplit(Split {
                id: self.cur_split_id,
                children: vec![],
            }),
        };
        match &mut self.root {
            None => self.root = Some(container),
            Some(root) => {
                let target_split = LayoutBuilder::get_split(root, prev_split);
                match target_split {
                    Some(Container::HSplit(split)) => split.children.push(container),
                    Some(Container::VSplit(split)) => split.children.push(container),
                    _ => panic!("Container not found"),
                }
            }
        };
    }

    pub fn build(self, width: usize, height: usize) -> Layout {
        Layout::new(self.gap, self.root.unwrap(), width, height)
    }
}

#[cfg(test)]
mod tests {
    use crate::{components::spinner::Spinner, layout::Container, layout2::LayoutBuilder};

    use super::SplitType;

    #[test]
    fn test_layout_builder() {
        let mut builder = LayoutBuilder::new();

        builder.add_split(SplitType::Vertical);
        builder.add(Box::new(Spinner {
            id: "spin1".to_string(),
            period_millis: 1000,
            running: true,
        }));
        builder.add(Box::new(Spinner {
            id: "spin2".to_string(),
            period_millis: 1000,
            running: true,
        }));
        builder.add_split(SplitType::Horizontal);
        builder.add(Box::new(Spinner {
            id: "spin3".to_string(),
            period_millis: 1000,
            running: true,
        }));

        if let Container::VSplit(split) = builder.root.unwrap() {
            dbg!(split.id);
            dbg!(split.children.len());
        }
    }
}
