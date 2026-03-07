#[derive(Clone, Debug, PartialEq, Default)]
pub enum GridAutoFlow { #[default] Row, Col, RowDense, ColDense }

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Grid {
    pub grid_template_columns: Vec<String>,
    pub grid_template_rows: Vec<String>,
    pub grid_auto_flow: GridAutoFlow,
    pub column_gap: Option<f32>,
    pub row_gap: Option<f32>,
}
