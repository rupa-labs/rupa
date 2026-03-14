use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Grid {
    pub template_columns: Vec<GridTrack>,
    pub template_rows: Vec<GridTrack>,
    pub auto_columns: Vec<GridTrack>,
    pub auto_rows: Vec<GridTrack>,
    pub column_gap: f32,
    pub row_gap: f32,
    pub auto_flow: GridAutoFlow,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct GridPlacement {
    pub column_start: GridLine,
    pub column_end: GridLine,
    pub row_start: GridLine,
    pub row_end: GridLine,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GridTrack {
    Fr(f32),
    Px(f32),
    Percent(f32),
    Auto,
    MinContent,
    MaxContent,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridAutoFlow {
    #[default]
    Row,
    Column,
    RowDense,
    ColumnDense,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridLine {
    #[default]
    Auto,
    Index(i16),
    Span(u16),
}
