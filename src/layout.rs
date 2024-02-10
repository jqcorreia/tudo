use std::{any::Any, collections::HashMap};

use sdl2::rect::Rect;

use crate::components::traits::UIComponent;

#[derive(Debug)]
pub enum SizeTypeEnum {
    Percent,
    Fixed,
}

pub struct Split {
    pub children: Vec<Container>,
}

pub struct Leaf {
    pub size: usize,
    pub size_type: SizeTypeEnum,
    pub key: String,
}

pub enum Container {
    Leaf(Leaf),
    HSplit(Split),
    VSplit(Split),
}

pub struct Layout {
    pub items: HashMap<String, LayoutItem>,
    pub gap: usize,
    pub width: usize,
    pub height: usize,
}

pub struct LayoutItem {
    pub rect: Rect,
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

    fn generate_recur(
        gap: usize,
        node: Container,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
    ) -> HashMap<String, LayoutItem> {
        let mut hm: HashMap<String, LayoutItem> = HashMap::new();
        match node {
            Container::Leaf(leaf) => {
                let m = gap;
                hm.insert(
                    leaf.key,
                    LayoutItem {
                        rect: Rect::new(
                            (x + m) as i32,
                            (y + m) as i32,
                            (w - 2 * m) as u32,
                            (h - 2 * m) as u32,
                        ),
                    },
                );
            }
            Container::HSplit(split) => {
                let mut accum_x = x;
                let accum_y = y;

                let mut sum_fixed_size: usize = 0;
                for child in split.children.iter() {
                    match child {
                        Container::Leaf(Leaf {
                            size_type: SizeTypeEnum::Fixed,
                            size,
                            ..
                        }) => sum_fixed_size += size,
                        _ => (),
                    };
                }
                let remaining_size = w - sum_fixed_size;

                for n in split.children {
                    let w_step = match n {
                        Container::Leaf(Leaf {
                            size_type: SizeTypeEnum::Fixed,
                            size,
                            ..
                        }) => size.clone(),
                        Container::Leaf(Leaf {
                            size_type: SizeTypeEnum::Percent,
                            size,
                            ..
                        }) => (remaining_size as f64 * (size.clone() as f64 / 100.0)) as usize,
                        _ => 0,
                    };

                    hm.extend(Self::generate_recur(gap, n, accum_x, accum_y, w_step, h));
                    accum_x += w_step;
                }
            }
            Container::VSplit(split) => {
                let accum_x = x;
                let mut accum_y = y;

                let mut sum_fixed_size: usize = 0;
                for child in split.children.iter() {
                    match child {
                        Container::Leaf(Leaf {
                            size_type: SizeTypeEnum::Fixed,
                            size,
                            ..
                        }) => sum_fixed_size += size,
                        _ => (),
                    };
                }
                let remaining_size = h - sum_fixed_size;

                for n in split.children {
                    let h_step = match n {
                        Container::Leaf(Leaf {
                            size_type: SizeTypeEnum::Fixed,
                            size,
                            ..
                        }) => size.clone(),
                        Container::Leaf(Leaf {
                            size_type: SizeTypeEnum::Percent,
                            size,
                            ..
                        }) => (remaining_size as f64 * (size.clone() as f64 / 100.0)) as usize,
                        _ => 0,
                    };

                    hm.extend(Self::generate_recur(gap, n, accum_x, accum_y, w, h_step));
                    accum_y += h_step;
                }
            }
        };
        hm
    }
}
