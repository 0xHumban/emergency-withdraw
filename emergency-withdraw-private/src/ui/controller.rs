use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use eyre::Result;

use crate::ui::tui::restore;

use super::{
    model::{CurrentScreen, Model},
    tui::{init, Tui},
    view::View,
};

pub struct Controller {
    model: Model,
    view: View,
    tui: Tui,
}

impl Controller {
    /// create an instance of the Controller
    pub async fn new() -> Result<Controller> {
        Ok(Controller {
            model: Model::new().await,
            view: View,

            tui: init()?,
        })
    }

    /// start the application
    pub async fn start(&mut self) -> Result<()> {
        self.run().await?;
        restore()?;
        println!(
            "Wallets selected before to quit:\n{:#?}",
            self.model.wallets_selected
        );
        Ok(())
    }

    /// main loop    
    async fn run(&mut self) -> Result<()> {
        while self.model.running {
            self.tui
                .draw(|frame| self.view.render_frame(frame, &mut self.model))?;
            // add handle events
            match event::read()? {
                // check if the event is a key press event
                // crossterm also emits release and repeat
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event).await
                }

                _ => self.tick(),
            };
        }

        Ok(())
    }

    // ---- HANDLE EVENTS

    /// handle key events
    async fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.model.current_screen {
            CurrentScreen::Main => self.handle_key_event_main(key_event),
            CurrentScreen::Transfering => {
                self.handle_key_event_confirm_transfer_popup(key_event)
                    .await
            }
        }
    }

    /// handle key events on main screen
    fn handle_key_event_main(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Down => self.model.next(),
            KeyCode::Up => self.model.previous(),
            KeyCode::Enter => self.model.toggle_wallet(),
            KeyCode::Char('a') => self.model.toggle_all_wallets(),
            KeyCode::Char('t') => self.model.enter_confirm_transfer(),
            _ => {}
        };
    }

    /// handle key events on confirm transfer popup

    async fn handle_key_event_confirm_transfer_popup(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.model.exit_confirm_transfer(),
            KeyCode::Left => self.model.next_confirm_transfer(),
            KeyCode::Right => self.model.next_confirm_transfer(),
            KeyCode::Enter => self.model.perform_action_confirm_transfer().await,
            KeyCode::Char('y') => self.model.perform_action_confirm_transfer().await,
            KeyCode::Char('Y') => self.model.perform_action_confirm_transfer().await,
            KeyCode::Char('n') => self.model.exit_confirm_transfer(),
            KeyCode::Char('N') => self.model.exit_confirm_transfer(),
            _ => {}
        };
    }

    // ----

    /// change the boolean to quit the app
    pub fn exit(&mut self) {
        self.model.running = false;
    }

    /// use to define refresh time in the terminal (sleep)
    pub fn tick(&self) {
        eprintln!("log: 'tick'function called");
    }
}
