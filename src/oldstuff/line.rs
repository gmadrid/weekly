// use crate::point_pair;
// use printpdf::Line;
//
// pub struct PLine {
//     x: f64,
//     y: f64,
//     x_offset: f64,
//     y_offset: f64,
// }
//
// impl PLine {
//     pub fn horiz(x: f64, y: f64, width: f64) -> PLine {
//         PLine {
//             x,
//             y,
//             x_offset: width,
//             y_offset: 0.0,
//         }
//     }
//
//     pub fn shape(&self) -> Line {
//         let points = vec![
//             point_pair(self.x, self.y),
//             point_pair(self.x + self.x_offset, self.y + self.y_offset),
//         ];
//         Line {
//             points,
//             is_closed: false,
//             has_fill: false,
//             has_stroke: true,
//             is_clipping_path: false,
//         }
//     }
// }
