use std::collections::HashMap;

use sdl2::rect::Rect;

use crate::components::traits::UIComponent;

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
    items: HashMap<String, LayoutItem2>,
}

#[derive(Debug)]
pub struct LayoutItem2 {
    pub rect: Rect,
    pub layout_idx: LayoutIndex,
}

impl LayoutBuilder {
    pub fn new() -> Self {
        LayoutBuilder {
            root: None,
            cur_split_idx: 0,
            arena: vec![],

            gap: 0,
            items: HashMap::new(),
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

    fn generate_recur(
        &self,
        node: LayoutIndex,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
    ) -> HashMap<String, LayoutItem2> {
        let mut hm: HashMap<String, LayoutItem2> = HashMap::new();
        let container = self.arena.get(node).unwrap();
        match &container.container_type {
            ContainerType2::Leaf(leaf) => {
                let m = self.gap;
                hm.insert(
                    leaf.component.id().clone(),
                    LayoutItem2 {
                        rect: Rect::new(
                            (x + m) as i32,
                            (y + m) as i32,
                            (w - 2 * m) as u32,
                            (h - 2 * m) as u32,
                        ),
                        layout_idx: node,
                    },
                );
            }
            ContainerType2::HSplit(split) => {
                let mut accum_x = x;
                let accum_y = y;

                let mut sum_fixed_size: usize = 0;
                for child_idx in split.children.clone() {
                    let container = self.arena.get(child_idx).unwrap();
                    match container.size {
                        ContainerSize2::Fixed(size) => sum_fixed_size += size,
                        _ => (),
                    };
                }
                let remaining_size = w - sum_fixed_size;

                for child_idx in split.children.clone() {
                    let container = self.arena.get(child_idx).unwrap();
                    let w_step = match container.size {
                        ContainerSize2::Fixed(size) => size.clone(),
                        ContainerSize2::Percent(size) => {
                            (remaining_size as f64 * (size.clone() as f64 / 100.0)) as usize
                        }
                    };

                    hm.extend(self.generate_recur(child_idx, accum_x, accum_y, w_step, h));
                    accum_x += w_step;
                }
            }
            ContainerType2::VSplit(split) => {
                let accum_x = x;
                let mut accum_y = y;

                let mut sum_fixed_size: usize = 0;
                for child_idx in split.children.clone() {
                    let container = self.arena.get(child_idx).unwrap();
                    match container.size {
                        ContainerSize2::Fixed(size) => sum_fixed_size += size,
                        _ => (),
                    };
                }
                let remaining_size = h - sum_fixed_size;

                for child_idx in split.children.clone() {
                    let container = self.arena.get(child_idx).unwrap();
                    let h_step = match container.size {
                        ContainerSize2::Fixed(size) => size.clone(),
                        ContainerSize2::Percent(size) => {
                            (remaining_size as f64 * (size.clone() as f64 / 100.0)) as usize
                        }
                    };

                    hm.extend(self.generate_recur(child_idx, accum_x, accum_y, w, h_step));
                    accum_y += h_step;
                }
            }
        };
        hm
    }
    pub fn by_name(&mut self, name: String) -> &mut Box<dyn UIComponent> {
        for cell in self.arena.iter_mut() {
            match cell {
                Container2 {
                    container_type: ContainerType2::Leaf(Leaf2 { component: comp }),
                    ..
                } => {
                    if comp.id() == name {
                        return comp;
                    }
                }
                _ => (),
            }
        }
        panic!("Component not found")
    }

    pub fn generate(&self, w: usize, h: usize) {
        self.items = self.generate_recur(*self.root.as_ref().unwrap(), 0, 0, w, h)
    }

    pub fn components_with_rect(
        &mut self,
    ) -> std::collections::hash_map::ValuesMut<'_, String, LayoutItem2> {
        self.items.values_mut()
    }

    // pub fn components(&mut self) -> Vec<&mut Box<dyn UIComponent>> {
    //     self.items.values_mut().map(|i| &mut i.component).collect()
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
            ContainerSize2::Percent(50),
        );
        builder.add(
            Box::new(Spinner {
                id: "spin2".to_string(),
                period_millis: 1000,
                running: true,
            }),
            ContainerSize2::Percent(50),
        );
        builder.get_split(1);

        builder.generate(1000, 1000);

        dbg!(&builder);
        dbg!(&builder.items);
        dbg!(builder.by_name("spin1".to_string()));
    }
}
