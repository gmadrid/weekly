use weekly::{Colors, HasRenderAttrs, Instructions, NumericUnit, Result, sizes, WLine};

fn main() -> Result<()> {
    let page_bounds = sizes::letter();
    weekly::save_one_page_document("foo", "playtime.pdf", &page_bounds, |_doc, bounds| {
        let mut instructions = Instructions::default();

        instructions.set_stroke_color(Colors::black());

        let whole_page_rect = bounds.clone();
        instructions.push_shape(whole_page_rect.stroke());

        let one_mm_rect = bounds.inset_all_q1(1.0.mm(),1.0.mm(),1.0.mm(),1.0.mm(),);
        instructions.push_shape(one_mm_rect.stroke());

        let five_mm_rect = bounds.inset_all_q1(5.0.mm(),5.0.mm(),5.0.mm(),5.0.mm(),);
        instructions.push_shape(five_mm_rect.stroke());

        let ten_mm_rect = bounds.inset_all_q1(10.0.mm(),10.0.mm(),10.0.mm(),10.0.mm(),);
        instructions.push_shape(ten_mm_rect.stroke());

        instructions.set_stroke_color(Colors::gray(0.75));
        instructions.set_dash(7, 3);
        let half_line = WLine::line(page_bounds.left(), page_bounds.height() / 2.0, page_bounds.right(), page_bounds.height() / 2.0);
        instructions.push_shape(half_line.stroke());

        Ok(instructions)
    })
}
