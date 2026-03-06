use ratatui::{
    style::{Color, Style, Stylize as _},
    widgets::Block,
};

#[derive(Debug)]
pub(crate) struct Theme {
    background: Color,
    background_highlight: Color,
    /// Accent color (titles, modal borders, links)
    accent: Color,
    accent_light: Color,
    /// Borders
    border_active: Color,
    border_inactive: Color,
    /// Params list selected item colors
    list_highlight: Color,
    list_highlight_text: Color,
    /// Common text color
    text: Color,
    /// Input text color
    text_input: Color,
    /// Text color for disabled actions
    text_muted: Color,
    /// Text color for parameter values
    text_param: Color,
    text_param_disabled: Color,
    /// Key bindings color
    keys: Color,
    /// Alert type and validation colors
    info: Color,
    info_text: Color,
    warning: Color,
    warning_text: Color,
    error: Color,
    error_text: Color,
    /// Triplet for info pane
    color1: Color,
    color2: Color,
    color3: Color,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            background: Color::Reset,
            background_highlight: Color::DarkGray,
            accent: Color::Blue,
            accent_light: Color::LightBlue,
            border_active: Color::White,
            border_inactive: Color::DarkGray,
            list_highlight: Color::White,
            list_highlight_text: Color::Black,
            text: Color::White,
            text_input: Color::White,
            text_muted: Color::Gray,
            text_param: Color::Yellow,
            text_param_disabled: Color::DarkGray,
            keys: Color::Green,
            info: Color::Blue,
            info_text: Color::White,
            warning: Color::Yellow,
            warning_text: Color::White,
            error: Color::Red,
            error_text: Color::White,
            color1: Color::LightYellow,
            color2: Color::LightCyan,
            color3: Color::LightMagenta,
        }
    }

    // ----- Getters -----

    #[inline]
    pub fn background_color(&self) -> Color {
        self.background
    }

    #[inline]
    pub fn background_highlight_color(&self) -> Color {
        self.background_highlight
    }

    #[inline]
    pub fn accent_color(&self) -> Color {
        self.accent
    }

    #[inline]
    pub fn accent_light_color(&self) -> Color {
        self.accent_light
    }

    #[inline]
    pub fn border_active_color(&self) -> Color {
        self.border_active
    }

    #[inline]
    pub fn border_inactive_color(&self) -> Color {
        self.border_inactive
    }

    #[inline]
    pub fn list_highlight_text_color(&self) -> Color {
        self.list_highlight_text
    }

    #[inline]
    pub fn list_highlight_color(&self) -> Color {
        self.list_highlight
    }

    #[inline]
    pub fn text_color(&self) -> Color {
        self.text
    }

    #[inline]
    pub fn text_input_color(&self) -> Color {
        self.text_input
    }

    #[inline]
    pub fn text_muted_color(&self) -> Color {
        self.text_muted
    }

    #[inline]
    pub fn text_param_color(&self) -> Color {
        self.text_param
    }

    #[inline]
    pub fn text_param_disabled_color(&self) -> Color {
        self.text_param_disabled
    }

    #[inline]
    pub fn keys_color(&self) -> Color {
        self.keys
    }

    #[inline]
    pub fn modal_title_color(&self) -> Color {
        self.accent_light
    }

    #[inline]
    pub fn info_text_color(&self) -> Color {
        self.info_text
    }

    #[inline]
    pub fn warning_color(&self) -> Color {
        self.warning
    }

    #[inline]
    pub fn warning_text_color(&self) -> Color {
        self.warning_text
    }

    #[inline]
    pub fn error_color(&self) -> Color {
        self.error
    }

    // ----- Other styles produced from getters -----

    #[inline]
    pub fn border_active_style(&self) -> Style {
        Style::new().fg(self.border_active_color())
    }

    #[inline]
    pub fn border_inactive_style(&self) -> Style {
        Style::new().fg(self.border_inactive_color())
    }

    #[inline]
    pub fn border_modal_style(&self) -> Style {
        Style::new().fg(self.accent_color())
    }

    #[inline]
    pub fn border_input_color(&self) -> Color {
        self.accent_light_color()
    }

    #[inline]
    pub fn border_input_inactive_color(&self) -> Color {
        self.accent_color()
    }

    #[inline]
    pub fn block_input(&self) -> Block<'_> {
        Block::bordered().fg(self.border_input_color())
    }

    #[inline]
    pub fn checkbox_checked_color(&self) -> Color {
        self.accent_light_color()
    }

    #[inline]
    pub fn checkbox_color(&self) -> Color {
        self.text_muted_color()
    }

    #[inline]
    pub fn checkbox_label_color(&self) -> Color {
        self.text_muted_color()
    }

    #[inline]
    pub fn checkbox_focused_style(&self) -> Style {
        Style::new().bg(self.background_highlight_color())
    }

    #[inline]
    pub fn pane_title_color(&self) -> Color {
        self.accent_color()
    }

    #[inline]
    pub fn list_highlight_style(&self) -> Style {
        Style::default()
            .fg(self.list_highlight_text_color())
            .bg(self.list_highlight_color())
    }

    #[inline]
    pub fn key_style(&self) -> Style {
        Style::new().fg(self.keys_color())
    }

    #[inline]
    pub fn info_style(&self) -> Style {
        Style::new().fg(self.info)
    }

    #[inline]
    pub fn warning_style(&self) -> Style {
        Style::new().fg(self.warning)
    }

    #[inline]
    pub fn error_style(&self) -> Style {
        Style::new().fg(self.error)
    }

    #[inline]
    pub fn error_bg_style(&self) -> Style {
        Style::new().fg(self.error_text).bg(self.error)
    }

    /// effy logo in Help modal
    #[inline]
    pub fn logo_color(&self) -> Color {
        self.keys
    }

    /// ratatui logo in Help modal
    #[inline]
    pub fn ratatui_logo_color(&self) -> Color {
        self.list_highlight_color()
    }

    /// ratatui logo in Help modal
    #[inline]
    pub fn about_caption_color(&self) -> Color {
        self.warning_color()
    }

    /// Links in Help modal
    #[inline]
    pub fn link_style(&self) -> Style {
        Style::new()
            .fg(self.accent)
            .underlined()
            .underline_color(self.accent_light)
    }

    /// Info pane format/streams keys
    #[inline]
    pub fn color_triplet(&self) -> [Color; 3] {
        [self.color1, self.color2, self.color3]
    }
}
