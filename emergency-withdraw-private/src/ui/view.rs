use std::rc::Rc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{block::Title, Block, Borders, Cell, Padding, Paragraph, Row, Table},
    Frame,
};

use crate::{
    constants::{APP_TITLE, INFO_TEXT},
    wallet::Wallet,
};

use super::model::Model;

pub struct View;

impl View {
    /// render the frame
    pub fn render_frame(&self, frame: &mut Frame, model: &mut Model) {
        // calculate all layouts
        let main_layout = self.calculate_main_layout_constraints(frame);
        let footer_layout = self.calculate_footer_layout_constraints(main_layout[1]);

        // render differents blocks in layouts
        self.render_main_block(frame, frame.size());
        self.render_table(frame, main_layout[0], model);
        self.render_address_selected_counter(frame, footer_layout[0], model);
        self.render_total_eth_balance(frame, footer_layout[1], model);
        self.render_to_address(frame, main_layout[2], model);

        // render popup
        self.render_popup(frame, model);
    }

    /// returns the layout with constraints in app
    fn calculate_main_layout_constraints(&self, frame: &Frame) -> Rc<[Rect]> {
        // used to define the dimension inside the main block
        let rect = Rect::new(
            frame.size().x + 1,
            frame.size().y + 1,
            frame.size().width - 2,
            frame.size().height - 2,
        );
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(rect);

        layout
    }

    /// returns the layouts with constraints for footer
    fn calculate_footer_layout_constraints(&self, area: Rect) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Ratio(1, 2); 2])
            .split(area)
    }

    // ----- RENDER

    /// render the main block
    fn render_main_block(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(Title::from(APP_TITLE).alignment(Alignment::Center))
            .title(
                Title::from(INFO_TEXT)
                    .alignment(Alignment::Center)
                    .position(ratatui::widgets::block::Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded);
        frame.render_widget(block, area);
    }

    /// render the block with the table
    fn render_table(&self, frame: &mut Frame, area: Rect, model: &mut Model) {
        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let header_names = vec!["N°", "Address", "$ETH balance"];
        // transform name in cells and in a row
        let header = header_names
            .iter()
            .copied()
            .map(|content| Cell::from(Text::from(content.to_string()).centered()))
            .collect::<Row>()
            .style(Style::new().cyan())
            .height(1);

        // create rows
        let rows = model
            .app_data
            .wallets()
            .into_iter()
            .enumerate()
            .map(|(index, wallet)| {
                let wallet_content = vec![
                    index.to_string(),
                    wallet.address_to_string(),
                    wallet.eth_balance_to_string(),
                ];

                let mut row = wallet_content
                    .iter()
                    .map(|content| Cell::from(Text::from(content.to_string()).centered()))
                    .collect::<Row>();

                // if the user is already selected in the list, change his style
                if wallet.is_wallet_in_list(&model.wallets_selected) {
                    row = row.style(Style::new().red().italic());
                }
                row
            });

        let bar = " █ ";

        // block where the table will be
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded);

        // create table

        let table = Table::new(
            rows,
            [
                // +1 for padding
                Constraint::Length(model.longuest_item_lens.0 + 0),
                Constraint::Min(model.longuest_item_lens.1 + 0),
                Constraint::Min(model.longuest_item_lens.2 + 0),
            ],
        )
        .header(header)
        .highlight_symbol(Text::from(vec![
            "".into(),
            bar.into(),
            bar.into(),
            "".into(),
        ]))
        .highlight_style(selected_style)
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
        .block(block);

        frame.render_stateful_widget(table, area, &mut model.table_state);
    }

    /// render the block with counter of address selected
    fn render_address_selected_counter(&self, frame: &mut Frame, area: Rect, model: &mut Model) {
        let block = Block::default().padding(Padding::vertical(1));

        let counter_text = Text::from(vec![Line::from(vec![
            "Address(es) selected: ".into(),
            model.wallets_selected.len().to_string().yellow(),
        ])]);

        let paragraph = Paragraph::new(counter_text)
            .alignment(Alignment::Center)
            .block(block);

        frame.render_widget(paragraph, area);
    }

    /// render the block with total ETH balance
    fn render_total_eth_balance(&self, frame: &mut Frame, area: Rect, model: &mut Model) {
        let total_number = Wallet::calculate_total_eth_balance_in_list(&model.wallets_selected)
            .unwrap_or("0".to_string());
        let block = Block::default().padding(Padding::vertical(1));

        let counter_text = Text::from(vec![Line::from(vec![
            "Total ether: ".into(),
            total_number.yellow(),
            " $ETH".into(),
        ])]);

        let paragraph = Paragraph::new(counter_text)
            .alignment(Alignment::Center)
            .block(block);

        frame.render_widget(paragraph, area);
    }

    /// render block with destination address
    fn render_to_address(&self, frame: &mut Frame, area: Rect, model: &Model) {
        let block = Block::default().padding(Padding::vertical(1));
        let counter_text = Text::from(vec![Line::from(vec![
            "To address: ".into(),
            model.app_data.to_address_to_string().yellow(),
        ])]);

        let paragraph = Paragraph::new(counter_text)
            .alignment(Alignment::Center)
            .block(block);

        frame.render_widget(paragraph, area);
    }

    // -----
}
