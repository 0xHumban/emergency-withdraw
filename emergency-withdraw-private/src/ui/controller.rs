use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use eyre::Result;

use crate::ui::tui::restore;

use super::{
    model::Model,
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
        self.run()?;
        restore()?;
        println!(
            "Wallets selected before to quit:\n{:#?}",
            self.model.wallets_selected
        );
        self.model.start_transfer_wallet_selected().await;
        Ok(())
    }

    /// main loop    
    fn run(&mut self) -> Result<()> {
        while self.model.running {
            self.tui
                .draw(|frame| self.view.render_frame(frame, &mut self.model))?;
            // add handle events
            match event::read()? {
                // check if the event is a key press event
                // crossterm also emits release and repeat
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }

                _ => self.tick(),
            };
        }

        Ok(())
    }

    /// handle key events
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Down => self.model.next(),
            KeyCode::Up => self.model.previous(),
            KeyCode::Enter => self.model.toggle_wallet(),
            KeyCode::Char('a') => self.model.toggle_all_wallets(),
            _ => {}
        };
    }

    /// change the boolean to quit the app
    pub fn exit(&mut self) {
        self.model.running = false;
    }

    /// use to define refresh time in the terminal (sleep)
    pub fn tick(&self) {
        eprintln!("log: 'tick'function called");
    }
}
