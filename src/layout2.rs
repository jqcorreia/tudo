use std::collections::HashMap;

use sdl2::rect::Rect;

use crate::{
    components::traits::UIComponent,
    layout::{Container, Layout, LayoutItem, Leaf, SizeTypeEnum, Split},
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

    pub fn add(&mut self, comp: Container) {
        match self.root {
            None => self.root = Some(comp),
            _ => (),
        };
    }

    pub fn add_split(&mut self, split_type: SplitType) {
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

        self.cur_split_id += 1;

        self.add(container)
    }

    pub fn build(self, width: usize, height: usize) -> Layout {
        Layout::new(self.gap, self.root.unwrap(), width, height)
    }
    // fn generate_recur(
    //     gap: usize,
    //     node: Container,
    //     x: usize,
    //     y: usize,
    //     w: usize,
    //     h: usize,
    // ) -> HashMap<String, LayoutItem> {
    //     let mut hm: HashMap<String, LayoutItem> = HashMap::new();
    //     match node {
    //         Container::Leaf(leaf) => {
    //             let m = gap;
    //             hm.insert(
    //                 leaf.component.id().clone(),
    //                 LayoutItem {
    //                     rect: Rect::new(
    //                         (x + m) as i32,
    //                         (y + m) as i32,
    //                         (w - 2 * m) as u32,
    //                         (h - 2 * m) as u32,
    //                     ),
    //                     component: leaf.component,
    //                 },
    //             );
    //         }
    //         Container::HSplit(split) => {
    //             let mut accum_x = x;
    //             let accum_y = y;

    //             let mut sum_fixed_size: usize = 0;
    //             for child in split.children.iter() {
    //                 match child {
    //                     Container::Leaf(Leaf {
    //                         size_type: SizeTypeEnum::Fixed,
    //                         size,
    //                         ..
    //                     }) => sum_fixed_size += size,
    //                     _ => (),
    //                 };
    //             }
    //             let remaining_size = w - sum_fixed_size;

    //             for n in split.children {
    //                 let w_step = match n {
    //                     Container::Leaf(Leaf {
    //                         size_type: SizeTypeEnum::Fixed,
    //                         size,
    //                         ..
    //                     }) => size.clone(),
    //                     Container::Leaf(Leaf {
    //                         size_type: SizeTypeEnum::Percent,
    //                         size,
    //                         ..
    //                     }) => (remaining_size as f64 * (size.clone() as f64 / 100.0)) as usize,
    //                     _ => 0,
    //                 };

    //                 hm.extend(Self::generate_recur(gap, n, accum_x, accum_y, w_step, h));
    //                 accum_x += w_step;
    //             }
    //         }
    //         Container::VSplit(split) => {
    //             let accum_x = x;
    //             let mut accum_y = y;

    //             let mut sum_fixed_size: usize = 0;
    //             for child in split.children.iter() {
    //                 match child {
    //                     Container::Leaf(Leaf {
    //                         size_type: SizeTypeEnum::Fixed,
    //                         size,
    //                         ..
    //                     }) => sum_fixed_size += size,
    //                     _ => (),
    //                 };
    //             }
    //             let remaining_size = h - sum_fixed_size;

    //             for n in split.children {
    //                 let h_step = match n {
    //                     Container::Leaf(Leaf {
    //                         size_type: SizeTypeEnum::Fixed,
    //                         size,
    //                         ..
    //                     }) => size.clone(),
    //                     Container::Leaf(Leaf {
    //                         size_type: SizeTypeEnum::Percent,
    //                         size,
    //                         ..
    //                     }) => (remaining_size as f64 * (size.clone() as f64 / 100.0)) as usize,
    //                     _ => 0,
    //                 };

    //                 hm.extend(Self::generate_recur(gap, n, accum_x, accum_y, w, h_step));
    //                 accum_y += h_step;
    //             }
    //         }
    //     };
    //     hm
    // }
}

#[cfg(test)]
mod tests {
    use crate::layout2::LayoutBuilder;

    use super::SplitType;

    #[test]
    fn test_basic_search() {
        let mut builder = LayoutBuilder::new();

        builder.add_split(SplitType::Vertical);
    }
}
