use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{self, UnboundedSender};

use crate::{
    action::TuiAction,
    components::{fps::FpsCounter, home::Home, Component},
    config::Config,
    mode::Mode,
    tui::{self, Tui},
};

pub struct App {
    pub config: Config,
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub components: Vec<Box<dyn Component>>,
    pub should_quit: bool,
    pub should_suspend: bool,
    pub mode: Mode,
    pub last_tick_key_events: Vec<KeyEvent>,
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Result<Self> {
        let home = Home::new();
        let fps = FpsCounter::default();
        let config = Config::new()?;
        let mode = Mode::Home;
        Ok(Self {
            tick_rate,
            frame_rate,
            components: vec![Box::new(home), Box::new(fps)],
            should_quit: false,
            should_suspend: false,
            config,
            mode,
            last_tick_key_events: Vec::new(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let (action_tx, mut action_rx) = mpsc::unbounded_channel();

        let mut tui = tui::Tui::new()?
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate);
        // tui.mouse(true);
        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(action_tx.clone())?;
        }

        for component in self.components.iter_mut() {
            component.register_config_handler(self.config.clone())?;
        }

        for component in self.components.iter_mut() {
            component.init(tui.size()?)?;
        }

        loop {
            if let Some(e) = tui.next().await {
                match e {
                    tui::Event::Quit => action_tx.send(TuiAction::Quit)?,
                    tui::Event::Tick => action_tx.send(TuiAction::Tick)?,
                    tui::Event::Render => action_tx.send(TuiAction::Render)?,
                    tui::Event::Resize(x, y) => action_tx.send(TuiAction::Resize(x, y))?,
                    tui::Event::Key(key) => {
                        if let Some(keymap) = self.config.keybindings.get(&self.mode) {
                            if let Some(action) = keymap.get(&vec![key]) {
                                log::info!("Got action: {action:?}");
                                action_tx.send(action.clone())?;
                            } else {
                                // If the key was not handled as a single key action,
                                // then consider it for multi-key combinations.
                                self.last_tick_key_events.push(key);

                                // Check for multi-key combinations
                                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                                    log::info!("Got action: {action:?}");
                                    action_tx.send(action.clone())?;
                                }
                            }
                        };
                    }
                    _ => {}
                }
                for component in self.components.iter_mut() {
                    if let Some(action) = component.handle_events(Some(e.clone()))? {
                        action_tx.send(action)?;
                    }
                }
            }

            while let Ok(action) = action_rx.try_recv() {
                if action != TuiAction::Tick && action != TuiAction::Render {
                    log::debug!("{action:?}");
                }
                match action {
                    TuiAction::Tick => {
                        self.last_tick_key_events.drain(..);
                    }
                    TuiAction::Quit => self.should_quit = true,
                    TuiAction::Suspend => self.should_suspend = true,
                    TuiAction::Resume => self.should_suspend = false,
                    TuiAction::Resize(w, h) => {
                        tui.resize(Rect::new(0, 0, w, h))?;
                        self.draw(&mut tui, &action_tx)?;
                    }
                    TuiAction::Render => {
                        self.draw(&mut tui, &action_tx)?;
                    }
                    _ => {}
                }
                for component in self.components.iter_mut() {
                    if let Some(action) = component.update(action.clone())? {
                        action_tx.send(action)?
                    };
                }
            }
            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(TuiAction::Resume)?;
                tui = tui::Tui::new()?
                    .tick_rate(self.tick_rate)
                    .frame_rate(self.frame_rate);
                // tui.mouse(true);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }

    fn draw(&mut self, tui: &mut Tui, action_tx: &UnboundedSender<TuiAction>) -> Result<()> {
        tui.draw(|f| {
            for component in self.components.iter_mut() {
                let r = component.draw(f, f.size());
                if let Err(e) = r {
                    action_tx
                        .send(TuiAction::Error(format!("Failed to draw: {:?}", e)))
                        .unwrap();
                }
            }
        })?;
        Ok(())
    }
}
