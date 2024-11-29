use sdl2::rect::Rect;

use crate::ui::components::traits::UIComponent;

type LayoutIndex = usize;

#[derive(Debug)]
pub enum SplitType {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub enum ContainerSize {
    Percent(usize),
    Fixed(usize),
}

#[derive(Debug)]
pub struct Container {
    pub size: ContainerSize,
    pub container_type: ContainerType,
}

#[derive(Debug)]
pub struct Split {
    pub children: Vec<usize>,
}

#[derive(Debug)]
pub struct Leaf {
    pub component: Box<dyn UIComponent>,
    pub rect: Option<Rect>,
}

#[derive(Debug)]
pub enum ContainerType {
    Leaf(Leaf),
    HSplit(Split),
    VSplit(Split),
}

#[derive(Debug)]
pub struct LayoutBuilder {
    gap: usize,
    root: Option<LayoutIndex>,
    cur_split_idx: LayoutIndex,
    arena: Vec<Container>,
}

impl Default for LayoutBuilder {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn get_container(&mut self, id: usize) -> Option<&mut Container> {
        self.arena.get_mut(id)
    }

    pub fn set_cur_split(&mut self, split_idx: usize) {
        self.cur_split_idx = split_idx;
    }

    pub fn add(&mut self, comp: Box<dyn UIComponent>, size: ContainerSize) -> LayoutIndex {
        let idx = self.arena.len();
        let split_idx = self.cur_split_idx;

        match &mut self.root {
            None => {
                self.root = Some(idx);
                self.arena.push(Container {
                    size: ContainerSize::Percent(100),
                    container_type: ContainerType::Leaf(Leaf {
                        component: comp,
                        rect: None,
                    }),
                });
            }
            Some(_) => {
                let target_split = self.get_container(split_idx);
                let container = Container {
                    size,
                    container_type: ContainerType::Leaf(Leaf {
                        component: comp,
                        rect: None,
                    }),
                };
                match target_split.unwrap() {
                    Container {
                        container_type:
                            ContainerType::HSplit(ref mut split) | ContainerType::VSplit(ref mut split),
                        ..
                    } => {
                        split.children.push(idx);
                        self.arena.push(container)
                    }
                    _ => panic!("Container not found"),
                }
            }
        };
        idx
    }

    pub fn add_split(&mut self, split_type: SplitType, size: ContainerSize) -> LayoutIndex {
        let idx = self.arena.len();

        // Create new split container
        let split = Split { children: vec![] };

        let container = Container {
            size,
            container_type: match split_type {
                SplitType::Horizontal => ContainerType::HSplit(split),
                SplitType::Vertical => ContainerType::VSplit(split),
            },
        };

        match &mut self.root {
            None => self.root = Some(idx),
            Some(_) => {
                let target_split = self.get_container(self.cur_split_idx);
                match target_split.unwrap() {
                    Container {
                        container_type:
                            ContainerType::HSplit(ref mut split) | ContainerType::VSplit(ref mut split),
                        ..
                    } => {
                        split.children.push(idx);
                    }
                    _ => panic!("Container not found"),
                }
            }
        };
        self.arena.push(container);
        self.cur_split_idx = idx;
        idx
    }

    fn generate_recur(
        &self,
        node: LayoutIndex,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
    ) -> Vec<(LayoutIndex, Rect)> {
        let mut vec: Vec<(LayoutIndex, Rect)> = Vec::new();
        let container = self.arena.get(node).unwrap();
        match &container.container_type {
            ContainerType::Leaf(_) => {
                let m = self.gap;
                // In case some of these values are zero, for instance while hiding a container
                if w == 0 || h == 0 {
                    return vec;
                }
                let rect = Rect::new(
                    (x + m) as i32,
                    (y + m) as i32,
                    (w - 2 * m) as u32,
                    (h - 2 * m) as u32,
                );
                vec.push((node, rect));
            }
            ContainerType::HSplit(split) => {
                let mut accum_x = x;
                let accum_y = y;

                let mut sum_fixed_size: usize = 0;
                for child_idx in split.children.clone() {
                    let container = self.arena.get(child_idx).unwrap();
                    if let ContainerSize::Fixed(size) = container.size { sum_fixed_size += size };
                }
                let remaining_size = w as i32 - sum_fixed_size as i32;

                for child_idx in split.children.clone() {
                    let container = self.arena.get(child_idx).unwrap();
                    let w_step = match container.size {
                        ContainerSize::Fixed(size) => size,
                        ContainerSize::Percent(size) => {
                            (remaining_size as f64 * (size as f64 / 100.0)) as usize
                        }
                    };

                    vec.extend(self.generate_recur(child_idx, accum_x, accum_y, w_step, h));
                    accum_x += w_step;
                }
            }
            ContainerType::VSplit(split) => {
                let accum_x = x;
                let mut accum_y = y;

                let mut sum_fixed_size: usize = 0;
                for child_idx in split.children.clone() {
                    let container = self.arena.get(child_idx).unwrap();
                    if let ContainerSize::Fixed(size) = container.size { sum_fixed_size += size };
                }
                let remaining_size = h as i32 - sum_fixed_size as i32;

                for child_idx in split.children.clone() {
                    let container = self.arena.get(child_idx).unwrap();
                    let h_step = match container.size {
                        ContainerSize::Fixed(size) => size,
                        ContainerSize::Percent(size) => {
                            (remaining_size as f64 * (size as f64 / 100.0)) as usize
                        }
                    };

                    vec.extend(self.generate_recur(child_idx, accum_x, accum_y, w, h_step));
                    accum_y += h_step;
                }
            }
        };
        vec
    }
    pub fn by_name(&mut self, name: String) -> &mut Box<dyn UIComponent> {
        for cell in self.arena.iter_mut() {
            if let Container {
                    container_type:
                        ContainerType::Leaf(Leaf {
                            component: comp, ..
                        }),
                    ..
                } = cell {
                if comp.id() == name {
                    return comp;
                }
            }
        }
        panic!("Component not found")
    }

    pub fn generate(&mut self, w: usize, h: usize) {
        let rects = self.generate_recur(*self.root.as_ref().unwrap(), 0, 0, w, h);

        for (idx, r) in rects {
            let container = self.arena.get_mut(idx).unwrap();

            if let Container {
                    container_type: ContainerType::Leaf(leaf),
                    ..
                } = container { leaf.rect = Some(r) }
        }
    }

    pub fn components_with_rect(&mut self) -> Vec<(Rect, &mut Box<dyn UIComponent>)> {
        self.arena
            .iter_mut()
            .filter_map(|container| match container {
                Container {
                    container_type: ContainerType::Leaf(Leaf { component, rect }),
                    ..
                } => Some((rect.unwrap(), component)),
                _ => None,
            })
            .collect()
    }

    pub fn components(&mut self) -> Vec<&mut Box<dyn UIComponent>> {
        self.arena
            .iter_mut()
            .filter_map(|container| match container {
                Container {
                    container_type: ContainerType::Leaf(Leaf { component, .. }),
                    ..
                } => Some(component),
                _ => None,
            })
            .collect()
    }

    pub fn by_coordinates(&mut self, x: i32, y: i32) -> &mut Box<dyn UIComponent> {
        self.arena
            .iter_mut()
            .filter_map(|container| match container {
                Container {
                    container_type: ContainerType::Leaf(Leaf { component, rect }),
                    ..
                } => Some((rect.unwrap(), component)),
                _ => None,
            })
            .find(|(r, _c)| x > r.x && x < r.x + r.w && y > r.y && y < r.y + r.h)
            .unwrap()
            .1
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ui::components::spinner::Spinner, ui::layout::ContainerSize, ui::layout::LayoutBuilder,
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
            ContainerSize::Percent(50),
        );
        builder.add(
            Box::new(Spinner {
                id: "spin2".to_string(),
                period_millis: 1000,
                running: true,
            }),
            ContainerSize::Percent(50),
        );
        builder.get_container(1);
        dbg!(&builder);
        builder.generate(1000, 1000);
        dbg!(&builder);
        dbg!(builder.by_name("spin1".to_string()));
        dbg!(builder.components());
        dbg!(builder.components_with_rect());
        assert!(builder.components().len() == 2);

        dbg!(builder.by_coordinates(500, 10));
    }
}
