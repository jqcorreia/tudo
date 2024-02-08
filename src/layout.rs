use std::collections::HashMap;

use sdl2::rect::Rect;

use crate::components::{enums::Component, traits::UIComponent};

#[derive(Debug)]
pub enum SizeTypeEnum {
    Percent,
    Fixed,
}

pub struct Split<'a> {
    pub children: Vec<Container<'a>>,
}

pub struct Leaf<'a> {
    pub size: usize,
    pub size_type: SizeTypeEnum,
    pub component: &'a mut dyn UIComponent,
}

pub enum Container<'a> {
    Leaf(Leaf<'a>),
    HSplit(Split<'a>),
    VSplit(Split<'a>),
}

pub struct Layout<'a> {
    pub items: HashMap<String, LayoutItem<'a>>,
    pub gap: usize,
    // pub root: Container<'a>,
    pub width: usize,
    pub height: usize,
}

pub struct LayoutItem<'a>(pub Rect, pub &'a mut dyn UIComponent);

impl<'a> Layout<'a> {
    pub fn new(gap: usize, mut root: Container<'a>, width: usize, height: usize) -> Self {
        let items = Layout::generate_recur(gap.clone(), &mut root, 0, 0, width, height);

        let layout = Layout {
            gap,
            width,
            height,
            // root,
            items,
        };
        layout
    }

    fn generate_recur(
        gap: usize,
        node: &'a mut Container,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
    ) -> HashMap<String, LayoutItem<'a>> {
        let mut hm: HashMap<String, LayoutItem<'a>> = HashMap::new();
        match node {
            Container::Leaf(leaf) => {
                let m = gap;
                hm.insert(
                    leaf.component.id(),
                    LayoutItem(
                        Rect::new(
                            (x + m) as i32,
                            (y + m) as i32,
                            (w - 2 * m) as u32,
                            (h - 2 * m) as u32,
                        ),
                        leaf.component,
                    ),
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

                for n in split.children.iter_mut() {
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

                for n in split.children.iter_mut() {
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
    // pub fn generate(&'a mut self) -> HashMap<String, LayoutItem<'a>> {
    //     return Layout::generate_recur(
    //         self.gap.clone(),
    //         &mut self.root,
    //         0,
    //         0,
    //         self.width,
    //         self.height,
    //     );
    // }
}
