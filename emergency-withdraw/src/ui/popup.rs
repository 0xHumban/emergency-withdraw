use ratatui::prelude::*;
use ratatui::widgets::block::Title;
use ratatui::widgets::{Clear, Paragraph};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};

use crate::constants::INFO_TEXT_CONFIRM_TRANSFER;

use super::model::{CurrentScreen, Model};
use super::{model::CurrentlyConfirming, view::View};

impl Model {
    // --- confirm transfer
    /// passed the model state to the next in confirm transfer popup
    pub fn next_confirm_transfer(&mut self) {
        if self.currently_transfering == Some(CurrentlyConfirming::Yes) {
            self.currently_transfering = Some(CurrentlyConfirming::No);
        } else {
            self.currently_transfering = Some(CurrentlyConfirming::Yes);
        }
    }

    /// clear the confirm transfer state (quit)
    pub fn exit_confirm_transfer(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.currently_transfering = None;
    }

    /// run the right action in function of the current state
    pub async fn perform_action_confirm_transfer(&mut self) {
        if self.currently_transfering == Some(CurrentlyConfirming::Yes) {
            let _ = self.start_transfer_wallet_selected().await;
            self.exit_confirm_transfer();
        } else {
            self.exit_confirm_transfer();
        }
    }

    /// enter / show in confirm transfer popup
    pub fn enter_confirm_transfer(&mut self) {
        self.current_screen = CurrentScreen::Transfering;
        self.currently_transfering = Some(CurrentlyConfirming::Yes);
    }

    // ---
}

impl View {
    /// render the right popup if currently selected
    pub fn render_popup(&self, frame: &mut Frame, model: &mut Model) {
        // check if in the confirm transfer popup
        if let Some(transfering_state) = model.currently_transfering.clone() {
            self.render_confirm_transfer_popup(frame, transfering_state);
        }
    }

    /// render the confirm transfer popup
    fn render_confirm_transfer_popup(&self, frame: &mut Frame, confirming: CurrentlyConfirming) {
        let popup_block = Block::default()
            .title(" Confirm the transfer ")
            .title_alignment(Alignment::Center)
            .title(
                Title::from(INFO_TEXT_CONFIRM_TRANSFER)
                    .alignment(Alignment::Center)
                    .position(ratatui::widgets::block::Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(Style::new().bg(Color::Blue));
        let area = centered_rect(60, 25, frame.size());
        // clear the area, to overwrite
        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let mut yes_block = Block::default().borders(Borders::ALL);
        let mut no_block = Block::default().borders(Borders::ALL);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);
        // check wich one is selected
        match confirming {
            CurrentlyConfirming::Yes => yes_block = yes_block.set_style(active_style),
            CurrentlyConfirming::No => no_block = no_block.set_style(active_style),
        }

        let yes_text = Paragraph::new(Text::from("YES").centered())
            .block(yes_block)
            .centered();
        frame.render_widget(yes_text, popup_chunks[0]);

        let no_text = Paragraph::new(Text::from("NO")).block(no_block).centered();
        frame.render_widget(no_text, popup_chunks[1]);
    }
}

// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let new_percent_y = (100 - percent_y) / 2;
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(new_percent_y),
            Constraint::Percentage(percent_y),
            Constraint::Percentage(new_percent_y),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
