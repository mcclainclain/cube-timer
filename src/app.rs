use self::State::*;
use crate::scramble::Scramble;
use crate::timer::Timer;
use crate::times::Times;

use figlet_rs::FIGfont;
use std::fmt::Display;
use std::{error, time::Duration};
use tui::style::Color;
use tui::widgets::TableState;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub state: State,      // State of the app
    pub running: bool,     // Is the app running?
    pub show_help: bool,   // Bool to determine whether to show help popup
    pub font: FIGfont,     // Font for the main timer
    pub timer: Timer,      // Timer object
    pub time: Duration,    // current time
    pub time_color: Color, // color of the main time
    pub scramble: Scramble,
    pub inspection_timer: Timer,   // timer for inspection
    pub inspection_time: Duration, // current inspection time
    pub times: Times,              // times list
    pub show_last_scramble: bool,  // bool to determine whether to show last scramble
    pub table_state: TableState,   // todo!()
    pub penalty: Penalty,          // penalty type
}

// App state
#[derive(Debug, PartialEq)]
pub enum State {
    Idle,
    Timing,
    Inspecting,
}

// Penalty types
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Penalty {
    None,
    PlusTwo,
    DNF,
}

impl Display for Penalty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Penalty::None => write!(f, ""),
            Penalty::PlusTwo => write!(f, "+2"),
            Penalty::DNF => write!(f, "DNF"),
        }
    }
}

// Default implementation for App
impl Default for App {
    fn default() -> Self {
        Self {
            state: Idle,
            running: true,
            show_help: false,
            font: FIGfont::standard().unwrap(),
            timer: Timer::new(),
            time: Duration::new(0, 0),
            time_color: Color::White,
            scramble: Scramble::new(20),
            inspection_timer: Timer::new(),
            inspection_time: Duration::new(0, 0),
            times: Times::new(),
            show_last_scramble: false,
            table_state: TableState::default(),
            penalty: Penalty::None,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        // Refresh the time and inspection time every tick.
        self.time = self.timer.get_time();
        self.inspection_time = self.inspection_timer.get_time();
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
        self.times.save_to_file(); // Save times to file times.json, so they can be loaded later.
    }

    /// Toggle help message.
    pub fn toggle_help(&mut self) {
        if self.timer.running {
            self.show_help = false;
        } else {
            self.show_help = !self.show_help;
        }
    }

    /// Handles and determines what happens when the user clicks space.
    /// Does the timer start?, stop?, reset?, etc.
    pub fn handle_space(&mut self) {
        match self.state {
            Idle => {
                // if idle, start inspection, and reset so there is no penalty
                self.start_inspection();
                self.penalty = Penalty::None;
            }
            Inspecting => {
                // if inspecting, stop inspecting, and start timing
                self.stop_inspection();
                self.state = Timing;
                self.timer.reset();
                self.timer.start();
            }
            Timing => {
                // if timing, stop timing, and add time to times list
                self.timer.stop();
                self.state = Idle;
                self.times.add_time(
                    self.time.as_secs_f64(),
                    self.scramble.to_string(),
                    self.penalty,
                );
                self.scramble = Scramble::new(20); // reset the scramble
            }
        }
    }

    /// Determines what time should be displayed on the main timer.
    pub fn time_string(&mut self) -> String {
        if self.state == Inspecting {
            if (15.00 - self.inspection_time.as_secs_f64()) < 0.00 {
                // if inspection goes longer than 15 seconds it is a penalty
                self.time_color = Color::Red;
                if 15.00 - self.inspection_time.as_secs_f64() < -2.00 {
                    // if inspection goes longer than 17 seconds it is a DNF
                    self.penalty = Penalty::DNF;
                    self.times.toggle_penalty(Penalty::DNF);
                    return self.font.convert("DNF").unwrap().to_string();
                }
                // if inspection goes longer than 15 seconds but less than 17 seconds it is a +2
                self.penalty = Penalty::PlusTwo;
                self.times.toggle_penalty(Penalty::PlusTwo);
                return self.font.convert("+2").unwrap().to_string();
            }

            // if inspection is less than 15 seconds, display the time
            return self
                .font
                .convert(&format!(
                    "{:.0}",
                    15.00 - self.inspection_time.as_secs_f64()
                ))
                .unwrap()
                .to_string();
        } else if self.state == Idle && self.time != Duration::new(0, 0) {
            // if idle, display the current time
            if self.times.currents()[0] == "NA" {
                return self
                    .font
                    .convert(&format!("{:.3}", 0.00))
                    .unwrap()
                    .to_string();
            }
            return self
                .font
                .convert(&format!("{:.5}", self.times.currents()[0]))
                .unwrap()
                .to_string();
        } else {
            return self
                .font
                .convert(&format!("{:.3}", self.time.as_secs_f64()))
                .unwrap()
                .to_string();
        }
    }

    /// Changes the color of the main timer.
    pub fn change_color(&mut self, color: Color) {
        self.time_color = color;
    }

    /// Refresh scramble with a new one.
    pub fn new_scramble(&mut self) {
        self.scramble = Scramble::new(20);
    }

    /// Start inspecting.
    pub fn start_inspection(&mut self) {
        self.state = Inspecting;
        self.inspection_timer.start();
    }

    /// Stop inspecting.
    pub fn stop_inspection(&mut self) {
        self.inspection_timer.stop();
        self.inspection_timer.reset();
    }

    /// Reset the timer.
    pub fn reset(&mut self) {
        self.times.reset();
        self.new_scramble();
    }

    /// Get the last scramble.
    pub fn last_scramble(&self) -> String {
        if (self.times.times["times"].as_array().unwrap().len()) == 0 {
            return String::from("No scrambles yet.");
        }
        return self.times.times["times"]
            .as_array()
            .unwrap()
            .last()
            .unwrap()["scramble"]
            .as_str()
            .unwrap()
            .to_string();
    }
}
