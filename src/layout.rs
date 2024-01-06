use sdl2::rect::Rect;

use crate::components::traits::Component;

#[derive(Debug)]
pub enum SizeTypeEnum {
    Percent,
    Fixed,
}

#[derive(Debug)]
pub enum ContainerType {
    Leaf,
    HSplit,
    VSplit,
}

// #[derive(Debug)]
pub struct Container {
    pub container_type: ContainerType,
    pub children: Option<Vec<Container>>,
    pub key: Option<String>,
    pub size: usize,
    pub size_type: SizeTypeEnum,
    pub component: Option<Box<dyn Component>>,
}

// #[derive(Debug)]
pub struct Layout {
    pub gap: usize,
    pub root: Container,
}

impl Layout {
    pub fn new(root: Container) -> Self {
        Layout { gap: 1, root }
    }
    // fn generate_recur(
    //     &mut self,
    //     num: usize,
    //     vec: &mut Vec<(Rect, String)>,
    //     node: &Container,
    //     x: usize,
    //     y: usize,
    //     w: usize,
    //     h: usize,
    // ) {
    //     match node.container_type {
    //         ContainerType::Leaf => {
    //             let m = self.gap;
    //             vec.push((
    //                 Rect::new(
    //                     (x + m) as i32,
    //                     (y + m) as i32,
    //                     (w - 2 * m) as u32,
    //                     (h - 2 * m) as u32,
    //                 ),
    //                 node.key.clone().unwrap(),
    //             ));
    //         }
    //         ContainerType::HSplit => {
    //             let nodes = node.children.as_ref().unwrap();
    //             let mut accum_x = x;
    //             let accum_y = y;

    //             let sum_fixed_size: usize = nodes
    //                 .iter()
    //                 .filter(|n| matches!(n.size_type, SizeTypeEnum::Fixed))
    //                 .map(|n| n.size)
    //                 .sum();
    //             let remaining_size = w - sum_fixed_size;

    //             for n in nodes {
    //                 let w_step = match n.size_type {
    //                     SizeTypeEnum::Fixed => n.size,
    //                     SizeTypeEnum::Percent => {
    //                         (remaining_size as f64 * (n.size as f64 / 100.0)) as usize
    //                     }
    //                 };

    //                 self.generate_recur(num + 1, vec, n, accum_x, accum_y, w_step, h);
    //                 accum_x += w_step;
    //             }
    //         }
    //         ContainerType::VSplit => {
    //             let nodes = node.children.as_ref().unwrap();
    //             let accum_x = x;
    //             let mut accum_y = y;
    //             let sum_fixed_size: usize = nodes
    //                 .iter()
    //                 .filter(|n| matches!(n.size_type, SizeTypeEnum::Fixed))
    //                 .map(|n| n.size)
    //                 .sum();
    //             let remaining_size = h - sum_fixed_size;

    //             for n in nodes {
    //                 let h_step = match n.size_type {
    //                     SizeTypeEnum::Fixed => n.size,
    //                     SizeTypeEnum::Percent => {
    //                         (remaining_size as f64 * (n.size as f64 / 100.0)) as usize
    //                     }
    //                 };
    //                 // let h_step = h / count;
    //                 self.generate_recur(num + 1, vec, n, accum_x, accum_y, w, h_step);
    //                 accum_y += h_step;
    //             }
    //         }
    //     };
    // }
    // pub fn generate(&mut self, w: usize, h: usize) -> Vec<(Rect, String)> {
    //     let mut vec: Vec<(Rect, String)> = Vec::new();

    //     self.generate_recur(0, &mut vec, &self.root, 0, 0, w, h);
    //     vec
    // }

    fn generate_recur2<'a>(
        num: usize,
        vec: &mut Vec<(Rect, String, Box<&'a mut dyn Component>)>,
        node: &'a mut Container,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
    ) {
        match node.container_type {
            ContainerType::Leaf => {
                let m = 10;
                vec.push((
                    Rect::new(
                        (x + m) as i32,
                        (y + m) as i32,
                        (w - 2 * m) as u32,
                        (h - 2 * m) as u32,
                    ),
                    node.key.clone().unwrap(),
                    Box::new(node.component.as_deref_mut().unwrap()),
                ));
            }
            ContainerType::HSplit => {
                let nodes = node.children.as_mut().unwrap();
                let mut accum_x = x;
                let accum_y = y;

                let sum_fixed_size: usize = nodes
                    .iter()
                    .filter(|n| matches!(n.size_type, SizeTypeEnum::Fixed))
                    .map(|n| n.size)
                    .sum();
                let remaining_size = w - sum_fixed_size;

                for n in nodes.iter_mut() {
                    let w_step = match n.size_type {
                        SizeTypeEnum::Fixed => n.size,
                        SizeTypeEnum::Percent => {
                            (remaining_size as f64 * (n.size as f64 / 100.0)) as usize
                        }
                    };

                    Layout::generate_recur2(num + 1, vec, n, accum_x, accum_y, w_step, h);
                    accum_x += w_step;
                }
            }
            ContainerType::VSplit => {
                let nodes = node.children.as_mut().unwrap();
                let accum_x = x;
                let mut accum_y = y;
                let sum_fixed_size: usize = nodes
                    .iter()
                    .filter(|n| matches!(n.size_type, SizeTypeEnum::Fixed))
                    .map(|n| n.size)
                    .sum();
                let remaining_size = h - sum_fixed_size;

                for n in nodes.iter_mut() {
                    let h_step = match n.size_type {
                        SizeTypeEnum::Fixed => n.size,
                        SizeTypeEnum::Percent => {
                            (remaining_size as f64 * (n.size as f64 / 100.0)) as usize
                        }
                    };
                    // let h_step = h / count;
                    Layout::generate_recur2(num + 1, vec, n, accum_x, accum_y, w, h_step);
                    accum_y += h_step;
                }
            }
        };
    }
    pub fn generate2(
        &mut self,
        w: usize,
        h: usize,
    ) -> Vec<(Rect, String, Box<&mut dyn Component>)> {
        let mut vec: Vec<(Rect, String, Box<&mut dyn Component>)> = Vec::new();

        Layout::generate_recur2(0, &mut vec, &mut self.root, 0, 0, w, h);
        vec
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_layout() {
//         let l = Layout {
//             gap: 10,
//             root: Container {
//                 container_type: ContainerType::HSplit,
//                 size: 10,
//                 size_type: SizeTypeEnum::Percent,
//                 children: Some(Vec::from([
//                     Container {
//                         size: 10,
//                         size_type: SizeTypeEnum::Percent,
//                         container_type: ContainerType::Leaf,
//                         children: None,
//                         key: Some(String::from("t1")),
//                     },
//                     Container {
//                         size: 10,
//                         size_type: SizeTypeEnum::Percent,
//                         container_type: ContainerType::VSplit,
//                         children: Some(Vec::from([
//                             Container {
//                                 size: 10,
//                                 size_type: SizeTypeEnum::Percent,
//                                 container_type: ContainerType::Leaf,
//                                 children: None,
//                                 key: Some(String::from("t2")),
//                             },
//                             Container {
//                                 size: 10,
//                                 size_type: SizeTypeEnum::Percent,
//                                 container_type: ContainerType::Leaf,
//                                 children: None,
//                                 key: Some(String::from("t3")),
//                             },
//                         ])),
//                         key: None,
//                     },
//                 ])),
//                 key: None,
//             },
//         };
//         dbg!(&l);
//         l.generate(100, 100);
//     }
// }
