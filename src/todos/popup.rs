use tui::{
    text::{Span, Spans},
    widgets::{
        Block,
        Borders, 
        canvas::Rectangle,
    }, layout::{
        Direction,
        Layout, Constraint, Rect
    },
};

pub struct Popup {
    pub show_popup: bool
}

impl Popup {
    pub fn new() -> Self {
        Self {
            show_popup: false
        }
    }
    pub fn render(&self, size: Rect) -> Option<(Block, Rect)> {
        if self.show_popup {
            let block = Block::default()
                .title("Popup")
                .borders(Borders::ALL);
            let area = centered_rect(60, 20, size);


            Some((block, area))
        // let area = centered_rect(60, 20, size);
        // f.render_widget(Clear, area); //this clears out the background
        // f.render_widget(block, area);
        } else {
            None
        }
    }

    pub fn add_popup(&self, size: Rect) -> ( Block, Rect ) {
        // if app.show_popup {
        let block = Block::default()
            .title("Popup")
            .borders(Borders::ALL);
        let area = centered_rect(60, 20, size);
        // f.render_widget(Clear, area); //this clears out the background
        // f.render_widget(block, area);
        // }
        (block, area)
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
