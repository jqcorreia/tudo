use std::collections::HashMap;

use sdl2::rect::Rect;

use crate::components::traits::UIComponent;

#[derive(Debug)]
pub enum ContainerSize {
    Percent(usize),
    Fixed(usize),
}

pub struct Split {
    pub children: Vec<Container>,
    pub id: u32,
}

pub struct Leaf {
    pub component: Box<dyn UIComponent>,
}

pub enum ContainerType {
    Leaf(Leaf),
    HSplit(Split),
    VSplit(Split),
}

pub struct Container {
    pub size: ContainerSize,
    pub container_type: ContainerType,
}

pub struct Layout {
    pub items: HashMap<String, LayoutItem>,
    pub gap: usize,
    pub width: usize,
    pub height: usize,
}

pub struct LayoutItem {
    pub rect: Rect,
    pub component: Box<dyn UIComponent>,
}

impl Layout {
    pub fn new(gap: usize, root: Container, width: usize, height: usize) -> Self {
        let items = Layout::generate_recur(gap.clone(), root, 0, 0, width, height);

        let layout = Layout {
            gap,
            width,
            height,
            items,
        };
        layout
    }
    pub fn by_name(&mut self, name: String) -> &mut LayoutItem {
        self.items.get_mut(&name).unwrap()
    }

    pub fn components_with_rect(
        &mut self,
    ) -> std::collections::hash_map::ValuesMut<'_, String, LayoutItem> {
        self.items.values_mut()
    }

    pub fn components(&mut self) -> Vec<&mut Box<dyn UIComponent>> {
        self.items.values_mut().map(|i| &mut i.component).collect()
    }

    fn generate_recur(
        gap: usize,
        node: Container,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
    ) -> HashMap<String, LayoutItem> {
        let mut hm: HashMap<String, LayoutItem> = HashMap::new();
        match node.container_type {
            ContainerType::Leaf(leaf) => {
                let m = gap;
                hm.insert(
                    leaf.component.id().clone(),
                    LayoutItem {
                        rect: Rect::new(
                            (x + m) as i32,
                            (y + m) as i32,
                            (w - 2 * m) as u32,
                            (h - 2 * m) as u32,
                        ),
                        component: leaf.component,
                    },
                );
            }
            ContainerType::HSplit(split) => {
                let mut accum_x = x;
                let accum_y = y;

                let mut sum_fixed_size: usize = 0;
                for child in split.children.iter() {
                    match child.size {
                        ContainerSize::Fixed(size) => sum_fixed_size += size,
                        _ => (),
                    };
                }
                let remaining_size = w - sum_fixed_size;

                for n in split.children {
                    let w_step = match n.size {
                        ContainerSize::Fixed(size) => size.clone(),
                        ContainerSize::Percent(size) => {
                            (remaining_size as f64 * (size.clone() as f64 / 100.0)) as usize
                        }
                    };

                    hm.extend(Self::generate_recur(gap, n, accum_x, accum_y, w_step, h));
                    accum_x += w_step;
                }
            }
            ContainerType::VSplit(split) => {
                let accum_x = x;
                let mut accum_y = y;

                let mut sum_fixed_size: usize = 0;
                for child in split.children.iter() {
                    match child.size {
                        ContainerSize::Fixed(size) => sum_fixed_size += size,
                        _ => (),
                    };
                }
                let remaining_size = h - sum_fixed_size;

                for n in split.children {
                    let h_step = match n.size {
                        ContainerSize::Fixed(size) => size.clone(),
                        ContainerSize::Percent(size) => {
                            (remaining_size as f64 * (size.clone() as f64 / 100.0)) as usize
                        }
                    };

                    hm.extend(Self::generate_recur(gap, n, accum_x, accum_y, w, h_step));
                    accum_y += h_step;
                }
            }
        };
        hm
    }
}
