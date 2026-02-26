use ratatui::{
    style::{Color, Style, Stylize as _},
    widgets::Block,
};

#[derive(Debug)]
pub(crate) struct Theme {
    background_color: Color,
    background_highlight_color: Color,
    /// Accent color (titles, modal borders, links)
    accent_color: Color,
    accent_light_color: Color,
    /// Borders
    border_active_color: Color,
    border_inactive_color: Color,
    /// Params list selected item colors
    list_highlight_text_color: Color,
    list_highlight_bg_color: Color,
    /// Common text color
    text_color: Color,
    /// Input text color
    text_input_color: Color,
    /// Text color for disabled actions
    text_muted_color: Color,
    /// Text color for parameter values
    text_param_color: Color,
    text_param_disabled_color: Color,
    /// Key bindings color
    keys_color: Color,
    /// Alert type and validation colors
    info_color: Color,
    info_text_color: Color,
    warning_color: Color,
    warning_text_color: Color,
    error_color: Color,
    error_text_color: Color,
    /// Triplet for info pane
    color1: Color,
    color2: Color,
    color3: Color,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            background_color: Color::Black,
            background_highlight_color: Color::DarkGray,
            accent_color: Color::Blue,
            accent_light_color: Color::LightBlue,
            border_active_color: Color::White,
            border_inactive_color: Color::DarkGray,
            list_highlight_text_color: Color::Black,
            list_highlight_bg_color: Color::White,
            text_color: Color::White,
            text_input_color: Color::White,
            text_muted_color: Color::Gray,
            text_param_color: Color::Yellow,
            text_param_disabled_color: Color::DarkGray,
            keys_color: Color::Green,
            info_color: Color::Blue,
            info_text_color: Color::White,
            warning_color: Color::Yellow,
            warning_text_color: Color::White,
            error_color: Color::Red,
            error_text_color: Color::White,
            color1: Color::LightYellow,
            color2: Color::LightCyan,
            color3: Color::LightMagenta,
        }
    }

    // ----- Getters -----

    #[inline]
    pub fn background_color(&self) -> Color {
        self.background_color
    }

    #[inline]
    pub fn background_highlight_color(&self) -> Color {
        self.background_highlight_color
    }

    #[inline]
    pub fn accent_color(&self) -> Color {
        self.accent_color
    }

    #[inline]
    pub fn accent_light_color(&self) -> Color {
        self.accent_light_color
    }

    #[inline]
    pub fn border_active_color(&self) -> Color {
        self.border_active_color
    }

    #[inline]
    pub fn border_inactive_color(&self) -> Color {
        self.border_inactive_color
    }

    #[inline]
    pub fn list_highlight_text_color(&self) -> Color {
        self.list_highlight_text_color
    }

    #[inline]
    pub fn list_highlight_bg_color(&self) -> Color {
        self.list_highlight_bg_color
    }

    #[inline]
    pub fn text_color(&self) -> Color {
        self.text_color
    }

    #[inline]
    pub fn text_input_color(&self) -> Color {
        self.text_input_color
    }

    #[inline]
    pub fn text_muted_color(&self) -> Color {
        self.text_muted_color
    }

    #[inline]
    pub fn text_param_color(&self) -> Color {
        self.text_param_color
    }

    #[inline]
    pub fn text_param_disabled_color(&self) -> Color {
        self.text_param_disabled_color
    }

    #[inline]
    pub fn keys_color(&self) -> Color {
        self.keys_color
    }

    #[inline]
    pub fn modal_title_color(&self) -> Color {
        self.accent_light_color
    }

    #[inline]
    pub fn info_text_color(&self) -> Color {
        self.info_text_color
    }

    #[inline]
    pub fn warning_color(&self) -> Color {
        self.warning_color
    }

    #[inline]
    pub fn warning_text_color(&self) -> Color {
        self.warning_text_color
    }

    #[inline]
    pub fn error_color(&self) -> Color {
        self.error_color
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
            .bg(self.list_highlight_bg_color())
    }

    #[inline]
    pub fn tab_bg_active(&self) -> Color {
        self.accent_color()
    }

    #[inline]
    pub fn tab_bg_inactive(&self) -> Color {
        self.border_inactive_color()
    }

    #[inline]
    pub fn key_style(&self) -> Style {
        Style::new().fg(self.keys_color())
    }

    #[inline]
    pub fn info_style(&self) -> Style {
        Style::new().fg(self.info_color)
    }

    #[inline]
    pub fn warning_style(&self) -> Style {
        Style::new().fg(self.warning_color)
    }

    #[inline]
    pub fn error_style(&self) -> Style {
        Style::new().fg(self.error_color)
    }

    #[inline]
    pub fn error_bg_style(&self) -> Style {
        Style::new().fg(self.error_text_color).bg(self.error_color)
    }

    /// effy logo in Help modal
    #[inline]
    pub fn logo_color(&self) -> Color {
        self.keys_color
    }

    /// ratatui logo in Help modal
    #[inline]
    pub fn ratatui_logo_color(&self) -> Color {
        self.list_highlight_bg_color()
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
            .fg(self.accent_color)
            .underlined()
            .underline_color(self.accent_light_color)
    }

    /// Info pane format/streams keys
    #[inline]
    pub fn color_triplet(&self) -> [Color; 3] {
        [self.color1, self.color2, self.color3]
    }
}
