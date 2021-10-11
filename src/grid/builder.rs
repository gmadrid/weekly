use crate::grid::TableGrid;
use crate::units::Unit;
use crate::{Instructions, NumericUnit, WRect};
use printpdf::IndirectFontRef;

#[derive(Default)]
pub struct Builder<'a> {
    doc_title: Option<String>,
    row_labels: Option<&'a [String]>,
    col_labels: Option<&'a [String]>,
    num_rows: Option<usize>,
    num_cols: Option<usize>,
    bounds: Option<WRect>,
    top_label_height: Option<Unit>,
    left_label_width: Option<Unit>,
    font: Option<&'a IndirectFontRef>,
    width_func: Option<Box<dyn Fn(usize) -> f64>>,
}

fn set_to_default_if_none<T>(opt: &mut Option<T>)
where
    T: Default,
{
    if opt.is_none() {
        opt.replace(T::default());
    }
}

fn set_if_none<T>(opt: &mut Option<T>, value: T) {
    if opt.is_none() {
        opt.replace(value);
    }
}

fn set_if_none_with<T>(opt: &mut Option<T>, f: impl Fn() -> T) {
    if opt.is_none() {
        opt.replace(f());
    }
}

impl<'a> Builder<'a> {
    pub fn new<'b>() -> Builder<'b> {
        Builder::default()
    }

    pub fn generate_instructions(&mut self) -> Instructions {
        self.build().instructions()
    }

    fn build(&mut self) -> TableGrid<'a> {
        // TODO: should this really be mutable self?
        self.fill_missing();

        TableGrid::new(
            self.doc_title.as_ref().unwrap(),
            self.row_labels.unwrap(),
            self.col_labels.unwrap(),
            self.num_rows.unwrap(),
            self.num_cols.unwrap(),
            self.bounds.clone().unwrap(),
            self.top_label_height.unwrap(),
            self.left_label_width.unwrap(),
            self.font.unwrap(),
            self.width_func.take().unwrap(),
        )
    }

    fn fill_missing(&mut self) {
        set_to_default_if_none(&mut self.doc_title);
        set_to_default_if_none(&mut self.row_labels);
        set_to_default_if_none(&mut self.col_labels);

        // unwrap, we just set this to a valid value.
        set_if_none(&mut self.num_rows, self.row_labels.unwrap().len());
        // unwrap, we just set this to a valid value.
        set_if_none(&mut self.num_cols, self.col_labels.unwrap().len());

        set_if_none_with(&mut self.bounds, || {
            WRect::with_dimensions(1.0.inches(), 1.0.inches())
        });
        set_if_none_with(&mut self.top_label_height, || 1.0.inches());
        set_if_none_with(&mut self.left_label_width, || 1.0.inches());

        set_if_none_with(&mut self.width_func, || Box::new(|_| 0.0));

        if self.font.is_none() {
            panic!("A font must be set to create a grid.");
        }
    }

    pub fn width_func(mut self, f: impl Fn(usize) -> f64 + 'static) -> Builder<'a> {
        let bf = Box::new(f);
        self.width_func = Some(bf);
        self
    }

    pub fn doc_title(mut self, title: impl Into<String>) -> Builder<'a> {
        self.doc_title = Some(title.into());
        self
    }

    pub fn row_labels(mut self, row_labels: &'a [String]) -> Builder<'a> {
        self.row_labels = Some(row_labels);
        self
    }

    pub fn col_labels(mut self, col_labels: &'a [String]) -> Builder<'a> {
        self.col_labels = Some(col_labels);
        self
    }

    pub fn num_rows(mut self, num_rows: usize) -> Builder<'a> {
        self.num_rows = Some(num_rows);
        self
    }

    pub fn num_cols(mut self, num_cols: usize) -> Builder<'a> {
        self.num_cols = Some(num_cols);
        self
    }

    pub fn bounds(mut self, bounds: WRect) -> Builder<'a> {
        self.bounds = Some(bounds);
        self
    }

    pub fn top_label_height(mut self, top_label_height: Unit) -> Builder<'a> {
        self.top_label_height = Some(top_label_height);
        self
    }

    pub fn left_label_width(mut self, left_label_width: Unit) -> Builder<'a> {
        self.left_label_width = Some(left_label_width);
        self
    }

    pub fn font(mut self, font: &'a IndirectFontRef) -> Builder<'a> {
        self.font = Some(font);
        self
    }
}
