use crate::{components::traits::UIComponent, layout::Layout};

type LayoutIndex = usize;

#[derive(Debug)]
pub enum SplitType {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub enum ContainerSize2 {
    Percent(usize),
    Fixed(usize),
}

#[derive(Debug)]
pub struct Container2 {
    pub size: ContainerSize2,
    pub container_type: ContainerType2,
}

#[derive(Debug)]
pub struct Split2 {
    pub children: Vec<usize>,
}

#[derive(Debug)]
pub struct Leaf2 {
    pub component: Box<dyn UIComponent>,
}

#[derive(Debug)]
pub enum ContainerType2 {
    Leaf(Leaf2),
    HSplit(Split2),
    VSplit(Split2),
}

#[derive(Debug)]
pub struct LayoutBuilder {
    gap: usize,
    root: Option<LayoutIndex>,
    cur_split_idx: LayoutIndex,
    arena: Vec<Container2>,
}

impl LayoutBuilder {
    pub fn new() -> Self {
        LayoutBuilder {
            root: None,
            cur_split_idx: 0,
            arena: vec![],

            gap: 0,
        }
    }

    pub fn with_gap(mut self, gap: usize) -> Self {
        self.gap = gap;
        self
    }

    pub fn get_split(&mut self, id: usize) -> Option<&mut Container2> {
        self.arena.get_mut(id)
    }

    pub fn add(&mut self, comp: Box<dyn UIComponent>, size: ContainerSize2) {
        let idx = self.arena.len();
        let split_idx = self.cur_split_idx;

        match &mut self.root {
            None => {
                self.root = Some(idx);
                self.arena.push(Container2 {
                    size: ContainerSize2::Percent(100),
                    container_type: ContainerType2::Leaf(Leaf2 { component: comp }),
                });
            }
            Some(root) => {
                let target_split = self.get_split(split_idx);
                let container = Container2 {
                    size,
                    container_type: ContainerType2::Leaf(Leaf2 { component: comp }),
                };
                match target_split.unwrap() {
                    Container2 {
                        container_type:
                            ContainerType2::HSplit(ref mut split)
                            | ContainerType2::VSplit(ref mut split),
                        ..
                    } => {
                        split.children.push(idx);
                        self.arena.push(container)
                    }
                    _ => panic!("Container not found"),
                }
            }
        };
    }

    pub fn add_split(&mut self, split_type: SplitType, size: ContainerSize2) {
        let idx = self.arena.len();
        self.cur_split_idx = idx;

        // Create new split container
        let split = Split2 { children: vec![] };

        let container = Container2 {
            size,
            container_type: match split_type {
                SplitType::Horizontal => ContainerType2::HSplit(split),
                SplitType::Vertical => ContainerType2::VSplit(split),
            },
        };

        match &mut self.root {
            None => self.root = Some(idx),
            Some(root) => {
                let target_split = self.get_split(self.cur_split_idx);
                match target_split.unwrap() {
                    Container2 {
                        container_type:
                            ContainerType2::HSplit(ref mut split)
                            | ContainerType2::VSplit(ref mut split),
                        ..
                    } => {
                        split.children.push(idx);
                    }
                    _ => panic!("Container not found"),
                }
            }
        };
        self.arena.push(container);
    }

    // pub fn build(self, width: usize, height: usize) -> Layout {
    //     Layout::new(
    //         self.gap,
    //         Container {
    //             container_type: ContainerType::Leaf(arena.get_mut(0)),
    //             size: ContainerSize::Percent(100),
    //         },
    //         width,
    //         height,
    //     )
    // }
}

#[cfg(test)]
mod tests {
    use crate::{
        components::spinner::Spinner,
        layout2::LayoutBuilder,
        layout2::{Container2, ContainerSize2, ContainerType2},
    };

    use super::SplitType;

    #[test]
    fn test_layout_builder() {
        let mut builder = LayoutBuilder::new();

        builder.add_split(SplitType::Horizontal, ContainerSize2::Percent(100));
        builder.add(
            Box::new(Spinner {
                id: "spin1".to_string(),
                period_millis: 1000,
                running: true,
            }),
            ContainerSize2::Percent(100),
        );
        builder.add(
            Box::new(Spinner {
                id: "spin2".to_string(),
                period_millis: 1000,
                running: true,
            }),
            ContainerSize2::Percent(100),
        );
        builder.get_split(1);
        dbg!(builder);
    }
}
