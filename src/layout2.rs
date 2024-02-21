use crate::{
    components::traits::UIComponent,
    layout::{Container, ContainerSize, ContainerType, Layout, Leaf, Split},
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

    pub fn get_split(node: &mut Container, id: u32) -> Option<&mut Container> {
        match node {
            Container {
                container_type:
                    ContainerType::HSplit(ref mut split) | ContainerType::VSplit(ref mut split),
                ..
            } => {
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

    pub fn add(&mut self, comp: Box<dyn UIComponent>, size: ContainerSize) {
        match &mut self.root {
            None => {
                self.root = Some(Container {
                    size: ContainerSize::Percent(100),
                    container_type: ContainerType::Leaf(Leaf { component: comp }),
                })
            }
            Some(root) => {
                let target_split = LayoutBuilder::get_split(root, self.cur_split_id);
                let container = Container {
                    size,
                    container_type: ContainerType::Leaf(Leaf { component: comp }),
                };
                match target_split.unwrap() {
                    Container {
                        container_type:
                            ContainerType::HSplit(ref mut split) | ContainerType::VSplit(ref mut split),
                        ..
                    } => split.children.push(container),
                    _ => panic!("Container not found"),
                }
            }
        };
    }

    pub fn add_split(&mut self, split_type: SplitType, size: ContainerSize) {
        let next_split = self.cur_split_id + 1;

        // Create new split container
        let split = Split {
            id: next_split,
            children: vec![],
        };

        let container = Container {
            size,
            container_type: match split_type {
                SplitType::Horizontal => ContainerType::HSplit(split),
                SplitType::Vertical => ContainerType::VSplit(split),
            },
        };

        match &mut self.root {
            None => self.root = Some(container),
            Some(root) => {
                let target_split = LayoutBuilder::get_split(root, self.cur_split_id);
                match target_split.unwrap() {
                    Container {
                        container_type:
                            ContainerType::HSplit(ref mut split) | ContainerType::VSplit(ref mut split),
                        ..
                    } => split.children.push(container),
                    _ => panic!("Container not found"),
                }
            }
        };
        self.cur_split_id += 1;
    }

    pub fn build(self, width: usize, height: usize) -> Layout {
        Layout::new(self.gap, self.root.unwrap(), width, height)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        components::spinner::Spinner,
        layout::{Container, ContainerSize, ContainerType},
        layout2::LayoutBuilder,
    };

    use super::SplitType;

    #[test]
    fn test_layout_builder() {
        let mut builder = LayoutBuilder::new();

        builder.add_split(SplitType::Horizontal, ContainerSize::Percent(100));
        builder.add(
            Box::new(Spinner {
                id: "spin1".to_string(),
                period_millis: 1000,
                running: true,
            }),
            ContainerSize::Percent(100),
        );
        LayoutBuilder::get_split(&mut builder.root.unwrap(), 1);
        // LayoutBuilder::get_split(&mut builder.root.unwrap(), 2);
        // builder.add_split(SplitType::Vertical, ContainerSize::Percent(100));
        // builder.add(
        //     Box::new(Spinner {
        //         id: "spin2".to_string(),
        //         period_millis: 1000,
        //         running: true,
        //     }),
        //     ContainerSize::Fixed(100),
        // );
        // builder.add(
        //     Box::new(Spinner {
        //         id: "spin3".to_string(),
        //         period_millis: 1000,
        //         running: true,
        //     }),
        //     ContainerSize::Fixed(100),
        // );

        // if let ContainerType::VSplit(split) = builder.root.unwrap().container_type {
        //     dbg!(split.id);
        //     dbg!(split.children.len());
        // }
    }
}
