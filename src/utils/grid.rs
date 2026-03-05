#[derive(Clone, Debug, PartialEq, Default)]
pub enum GridAutoFlow { #[default] Row, Column, Dense, RowDense, ColumnDense }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum JustifyItems { #[default] Start, Center, End, Stretch }

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Grid {
    pub template_columns: Vec<String>,
    pub template_rows: Vec<String>,
    pub auto_columns: Vec<String>,
    pub auto_rows: Vec<String>,
    pub auto_flow: GridAutoFlow,
    pub column_span: Option<String>,
    pub row_span: Option<String>,
    pub gap: (f32, f32), // (row, col)
    pub justify_items: JustifyItems,
}
