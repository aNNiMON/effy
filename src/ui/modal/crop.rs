use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Margin, Offset, Position, Rect},
    style::Stylize as _,
    symbols,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget as _},
};
use tui_input::{Input, backend::crossterm::EventHandler as _};

use crate::{
    model::CropData,
    ui::{
        Theme, input_value_and_pos, is_portrait,
        modal::{KeyboardHandler, ModalResult, UiModal},
        widget::BgClear,
    },
};

#[derive(Debug)]
pub(crate) struct CropModal {
    active_input: usize,
    x: Input,
    y: Input,
    w: Input,
    h: Input,
    dimensions: (u32, u32),
    error: Option<String>,
}

impl UiModal for CropModal {
    fn render(&mut self, frame: &mut Frame, theme: &Theme) {
        let area = frame.area();
        let portrait = is_portrait(area);
        let [modal_area] = Layout::vertical([Constraint::Length(10)])
            .horizontal_margin(if portrait { 1 } else { area.width / 4 })
            .flex(Flex::Center)
            .areas(area);
        let [xy_area, wh_area, hints_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .areas(modal_area.inner(Margin::new(2, 1)));

        let [x_area, y_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(xy_area);
        let [w_area, h_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(wh_area);

        frame.render_widget(BgClear::new(theme.background_color()), modal_area);
        Block::bordered()
            .title("Crop".fg(theme.modal_title_color()))
            .border_set(symbols::border::THICK)
            .border_style(theme.border_modal_style())
            .render(modal_area, frame.buffer_mut());

        let active_border_style = theme.border_input_color();
        let inactive_border_style = theme.border_input_inactive_color();
        let mut border_styles = [inactive_border_style; 4];
        border_styles[self.active_input] = active_border_style;

        // Inputs
        let (dw, dh) = self.dimensions;
        for (i, (input, area, label)) in [
            (&self.x, x_area, "X".to_owned()),
            (&self.y, y_area, "Y".to_owned()),
            (&self.w, w_area, format!("Width (max {})", dw)),
            (&self.h, h_area, format!("Height (max {})", dh)),
        ]
        .into_iter()
        .enumerate()
        {
            let (val, pos) = input_value_and_pos(input, area.width);
            Paragraph::new(val)
                .block(
                    Block::bordered()
                        .border_style(border_styles[i])
                        .style(if self.active_input == i {
                            theme.text_input_color()
                        } else {
                            theme.text_muted_color()
                        })
                        .title(Span::styled(label, border_styles[i])),
                )
                .render(area, frame.buffer_mut());

            if self.active_input == i {
                frame.set_cursor_position(Position {
                    x: area.x + pos,
                    y: area.y + 1,
                });
            }
        }
        self.render_status(hints_area.offset(Offset::new(0, 1)), frame, theme);
    }
}

impl KeyboardHandler for CropModal {
    fn handle_key(&mut self, key: KeyEvent) -> ModalResult {
        self.error.take();
        match key.code {
            KeyCode::Esc => return ModalResult::Close,
            KeyCode::BackTab => self.active_input = (self.active_input + 3) % 4,
            KeyCode::Tab => self.active_input = (self.active_input + 1) % 4,
            KeyCode::Char(s) => match (self.active_input, s) {
                (0, '0'..='9') if Self::prevalidate_value(self.x.value()) => {
                    self.x.handle_event(&Event::Key(key));
                }
                (1, '0'..='9') if Self::prevalidate_value(self.y.value()) => {
                    self.y.handle_event(&Event::Key(key));
                }
                (2, '0'..='9') if Self::prevalidate_value(self.w.value()) => {
                    self.w.handle_event(&Event::Key(key));
                }
                (3, '0'..='9') if Self::prevalidate_value(self.h.value()) => {
                    self.h.handle_event(&Event::Key(key));
                }
                _ => return ModalResult::None,
            },
            KeyCode::Backspace | KeyCode::Delete => {
                [&mut self.x, &mut self.y, &mut self.w, &mut self.h]
                    .get_mut(self.active_input)
                    .expect("active_input is invalid")
                    .handle_event(&Event::Key(key));
            }
            KeyCode::Enter => {
                if let Err(msg) = CropData::validate(
                    self.x.value(),
                    self.y.value(),
                    self.w.value(),
                    self.h.value(),
                    self.dimensions,
                ) {
                    self.error = Some(msg);
                    return ModalResult::None;
                }
                return ModalResult::Crop;
            }
            _ => {}
        }
        ModalResult::None
    }
}

impl From<&CropModal> for CropData {
    fn from(model: &CropModal) -> CropData {
        CropData {
            x: Some(model.x.value().to_owned()).filter(|x| !x.is_empty()),
            y: Some(model.y.value().to_owned()).filter(|x| !x.is_empty()),
            w: Some(model.w.value().to_owned()).filter(|x| !x.is_empty()),
            h: Some(model.h.value().to_owned()).filter(|x| !x.is_empty()),
        }
    }
}

impl CropModal {
    pub(crate) fn new(data: CropData, dimensions: (u32, u32)) -> Self {
        let (dw, dh) = dimensions;
        Self {
            active_input: 0,
            x: Input::new(data.x.unwrap_or_default()),
            y: Input::new(data.y.unwrap_or_default()),
            w: Input::new(data.w.unwrap_or(dw.to_string())),
            h: Input::new(data.h.unwrap_or(dh.to_string())),
            dimensions,
            error: None,
        }
    }

    fn render_status(&self, area: Rect, frame: &mut Frame, theme: &Theme) {
        let line = if let Some(error) = &self.error {
            Line::from(Span::styled(error, theme.error_style().bold())).centered()
        } else {
            let key_style = theme.key_style();
            let text_style = theme.text_color();
            Line::from(vec![
                Span::styled("Enter", key_style),
                Span::styled(": confirm  ", text_style),
                Span::styled("Esc", key_style),
                Span::styled(": close  ", text_style),
                Span::styled("Tab", key_style),
                Span::styled(": switch focus", text_style),
            ])
        };
        frame.render_widget(Paragraph::new(line), area);
    }

    fn prevalidate_value(value: &str) -> bool {
        value.len() < 5
    }
}
