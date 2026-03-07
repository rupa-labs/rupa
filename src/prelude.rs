pub use crate::core::component::Component;
pub use crate::platform::{App, PlatformEvent, request_redraw};

// Layout Elements
pub use crate::elements::layout::{Section, Container, VStack, HStack};
pub use crate::elements::viewport::Viewport;

// Base Elements
pub use crate::elements::{
    Text, 
    Brand, 
    Svg, 
    ThemeSwitcher,
    button::Button
};

// Forms
pub use crate::elements::forms::{
    Label, 
    Input, 
    Select, 
    Checkbox, 
    Radio, 
    Switch
};

// Feedback
pub use crate::elements::feedback::{
    Alert, 
    Badge, 
    Progress, 
    Spinner, 
    Skeleton
};

// Navigation & Overlays
pub use crate::elements::navigation::{Navbar, Tabs, Breadcrumb};
pub use crate::elements::overlay::{Modal, Tooltip};

// Control Flow
pub use crate::elements::control_flow::{Show, ForEach};

// Primitives
pub use crate::primitives::{div::Div, flex::Flex, grid::Grid};

// Core Support (Infrastructure)
pub use crate::support::{
    Vec2, 
    Signal, 
    Memo,
    Readable,
    generate_id
};

// DNA Visual & Styling (Ecosystem - L9)
pub use crate::style::utilities::style::Style;
pub use crate::style::utilities::color::Color;
pub use crate::style::utilities::typography::{TextAlign, FontWeight, TypographyStyle};
pub use crate::style::utilities::scale::Scale;
pub use crate::style::modifiers::base::{StyleModifier, Stylable};
pub use crate::style::modifiers::visual::ChainedVisual;
pub use crate::style::modifiers::spacing::ChainedSpacing;
pub use crate::style::modifiers::layout::ChainedLayout;
pub use crate::style::modifiers::sizing::ChainedSizing;
pub use crate::style::modifiers::responsive::{
    Breakpoint, ChainedResponsive, 
    xs, sm, md, lg, xl, xl2, xl3, xl4, xl5, xl6
};
pub use crate::style::modifiers::state::{
    ChainedState, hover, active, focus, group_hover, is_group
};
pub use crate::style::modifiers::animation::{ChainedMotion, motion, Easing};
pub use std::time::Duration;
pub use crate::style::modifiers::theme::{Theme, Variant};
pub use crate::style::modifiers::{
    p, px, py, m, mx, my, 
    bg, text_color, rounded, rounded_full,
    w, h, w_full, h_full,
    flex, col, row, gap,
    items_center, justify_center
};
