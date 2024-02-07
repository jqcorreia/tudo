use sdl2::rect::Rect;

use crate::components::enums::Component;

#[derive(Debug)]
pub enum SizeTypeEnum {
    Percent,
    Fixed,
}

pub struct Split {
    pub children: Vec<Container>,
}

pub struct Leaf {
    pub key: String,
    pub size: usize,
    pub size_type: SizeTypeEnum,
    pub component: Component,
}

pub enum Container {
    Leaf(Leaf),
    HSplit(Split),
    VSplit(Split),
}

pub struct Layout {
    pub gap: usize,
    pub root: Container,
    pub width: usize,
    pub height: usize,
}

pub struct LayoutItem<'a>(pub Rect, pub String, pub &'a mut Component);

impl Layout {
    fn generate_recur<'a>(
        gap: usize,
        num: usize,
        node: &'a mut Container,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
    ) -> Vec<LayoutItem> {
        let mut vec: Vec<LayoutItem> = Vec::new();
        match node {
            Container::Leaf(leaf) => {
                let m = gap;
                vec.push(LayoutItem(
                    Rect::new(
                        (x + m) as i32,
                        (y + m) as i32,
                        (w - 2 * m) as u32,
                        (h - 2 * m) as u32,
                    ),
                    leaf.key.clone(),
                    &mut leaf.component,
                ));
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

                    vec.extend(Self::generate_recur(
                        gap,
                        num + 1,
                        n,
                        accum_x,
                        accum_y,
                        w_step,
                        h,
                    ));
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

                    vec.extend(Self::generate_recur(
                        gap,
                        num + 1,
                        n,
                        accum_x,
                        accum_y,
                        w,
                        h_step,
                    ));
                    accum_y += h_step;
                }
            }
        };
        vec
    }
    pub fn generate(&mut self) -> Vec<LayoutItem> {
        return Layout::generate_recur(
            self.gap.clone(),
            0,
            &mut self.root,
            0,
            0,
            self.width,
            self.height,
        );
    }
}
