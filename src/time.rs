use chrono::{DateTime, Local, NaiveTime, Timelike};

#[derive(Debug, Clone)]
pub struct TimeManager {
    pub current_time: DateTime<Local>,
}

impl TimeManager {
    pub fn new() -> Self {
        Self {
            current_time: Local::now(),
        }
    }

    pub fn update(&mut self) {
        self.current_time = Local::now();
    }

    pub fn format_time(&self, format_24h: bool) -> String {
        if format_24h {
            self.current_time.format("%H:%M:%S").to_string()
        } else {
            self.current_time.format("%I:%M:%S %p").to_string()
        }
    }

    pub fn format_date(&self, format_us: bool) -> String {
        if format_us {
            self.current_time.format("%m/%d").to_string()
        } else {
            self.current_time.format("%d/%m").to_string()
        }
    }

    pub fn format_day_of_week(&self) -> String {
        self.current_time.format("%a").to_string()
    }

    pub fn check_alarm(&self, alarm_time: &Option<String>) -> bool {
        if let Some(alarm) = alarm_time {
            if let Ok(time) = NaiveTime::parse_from_str(alarm, "%H:%M") {
                let current = self.current_time.time();

                let diff = (current.hour() as i32 - time.hour() as i32) * 60
                    + (current.minute() as i32 - time.minute() as i32);
                diff >= 0 && diff < 1
            } else {
                false
            }
        } else {
            false
        }
    }
}
