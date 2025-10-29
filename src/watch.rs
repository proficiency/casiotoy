use anyhow::Result;
use std::time::Instant;

use crate::{settings::WatchSettings, time::TimeManager};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WatchMode {
    Home,
    WorldTime,
    Alarm,
    Timer,
    Stopwatch,
}

pub struct Watch {
    pub mode: WatchMode,
    pub settings: WatchSettings,
    pub time_manager: TimeManager,
    pub stopwatch_time: u64, // milliseconds
    pub stopwatch_running: bool,
    pub stopwatch_start_time: Option<Instant>,
    pub timer_time: u64, // milliseconds
    pub timer_running: bool,
    pub timer_start_time: Option<Instant>,
    pub light_on: bool,
    pub light_start_time: Option<Instant>,
}

impl Watch {
    pub fn new() -> Result<Self> {
        let settings = WatchSettings::load()?;
        let time_manager = TimeManager::new();
        
        Ok(Self {
            mode: WatchMode::Home,
            settings,
            time_manager,
            stopwatch_time: 0,
            stopwatch_running: false,
            stopwatch_start_time: None,
            timer_time: 0,
            timer_running: false,
            timer_start_time: None,
            light_on: false,
            light_start_time: None,
        })
    }

    pub fn update(&mut self) -> Result<()> {
        self.time_manager.update();

        // update stopwatch if running
        if self.stopwatch_running {
            if let Some(start_time) = self.stopwatch_start_time {
                let elapsed = start_time.elapsed().as_millis() as u64;
                self.stopwatch_time = elapsed;
            }
        }

        // update timer if running
        if self.timer_running {
            if let Some(start_time) = self.timer_start_time {
                let elapsed = start_time.elapsed().as_millis() as u64;
                self.timer_time = elapsed;
            }
        }

        // turn the light off
        if self.light_on {
            if let Some(start_time) = self.light_start_time {
                let elapsed = start_time.elapsed().as_secs();
                if elapsed >= self.settings.auto_light_duration {
                    self.light_on = false;
                    self.light_start_time = None;
                }
            }
        }

        // todo: handle alarm, beeping sound etc
        if self.settings.alarm_enabled && self.time_manager.check_alarm(&self.settings.alarm_time) {
            todo!()
        }
        
        Ok(())
    }

    pub fn toggle_mode(&mut self) -> Result<()> {
        self.mode = match self.mode {
            WatchMode::Home => WatchMode::WorldTime,
            WatchMode::WorldTime => WatchMode::Alarm,
            WatchMode::Alarm => WatchMode::Timer,
            WatchMode::Timer => WatchMode::Stopwatch,
            WatchMode::Stopwatch => WatchMode::Home,
        };
        Ok(())
    }

    pub fn toggle_start_stop(&mut self) -> Result<()> {
        match self.mode {
            WatchMode::Stopwatch => {
                if self.stopwatch_running {
                    // Stop the stopwatch
                    self.stopwatch_running = false;
                    if let Some(start_time) = self.stopwatch_start_time {
                        let elapsed = start_time.elapsed().as_millis() as u64;
                        self.stopwatch_time = elapsed;
                    }
                    self.stopwatch_start_time = None;
                } else {
                    // Start the stopwatch
                    self.stopwatch_running = true;
                    self.stopwatch_start_time = Some(Instant::now());
                }
            }
            WatchMode::Timer => {
                if self.timer_running {
                    // Stop the timer
                    self.timer_running = false;
                    if let Some(start_time) = self.timer_start_time {
                        let elapsed = start_time.elapsed().as_millis() as u64;
                        self.timer_time = elapsed;
                    }
                    self.timer_start_time = None;
                } else {
                    // Start the timer
                    self.timer_running = true;
                    self.timer_start_time = Some(Instant::now());
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn reset(&mut self) -> Result<()> {
        match self.mode {
            WatchMode::Stopwatch => {
                self.stopwatch_time = 0;
                self.stopwatch_running = false;
                self.stopwatch_start_time = None;
            }
            WatchMode::Timer => {
                self.timer_time = 0;
                self.timer_running = false;
                self.timer_start_time = None;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn toggle_light(&mut self) -> Result<()> {
        self.light_on = !self.light_on;
        if self.light_on {
            self.light_start_time = Some(Instant::now());
        } else {
            self.light_start_time = None;
        }
        Ok(())
    }

    pub fn set_alarm(&mut self) -> Result<()> {
        self.settings.alarm_enabled = !self.settings.alarm_enabled;
        if self.settings.alarm_enabled {
            // Set alarm for 1 minute from now for testing
            let now = chrono::Local::now();
            let alarm_time = now + chrono::Duration::minutes(1);
            self.settings.alarm_time = Some(alarm_time.format("%H:%M").to_string());
        } else {
            self.settings.alarm_time = None;
        }
        self.settings.save()?;
        Ok(())
    }
}
