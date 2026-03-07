use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Grid {
    pub grid_template_columns: Vec<String>,
    pub grid_template_rows: Vec<String>,
    pub column_gap: f32,
    pub row_gap: f32,
}
