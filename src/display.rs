use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Clear},
};
use chrono::Timelike;

use crate::watch::{Watch, WatchMode, WatchModel, F91WMode};

pub fn ui(f: &mut Frame, watch: &Watch) {
    let size = f.area();
    
    // different sizes for different watches
    let (watch_width, watch_height) = match watch.model {
        WatchModel::AE1200 => (50, 15),
        WatchModel::F91W => (30, 10),
    };

    let watch_area = Rect {
        x: (size.width.saturating_sub(watch_width)) / 2,
        y: (size.height.saturating_sub(watch_height)) / 2,
        width: watch_width.min(size.width),
        height: watch_height.min(size.height),
    };
    
    // draw the frame with model-specific title
    let title = match watch.model {
        WatchModel::AE1200 => "Casio AE-1200",
        WatchModel::F91W => "Casio F-91W",
    };
    
    let watch_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));
    
    f.render_widget(Clear, watch_area);
    let watch_inner = watch_block.inner(watch_area);
    f.render_widget(watch_block, watch_area);
    
    match watch.model {
        WatchModel::AE1200 => {
            match watch.mode {
                WatchMode::Home => {
                    render_time_display(f, watch_inner, watch);
                }
                WatchMode::WorldTime => {
                    render_world_time_display(f, watch_inner, watch);
                }
                WatchMode::Alarm => {
                    render_alarm_display(f, watch_inner, watch);
                }
                WatchMode::Timer => {
                    render_timer_display(f, watch_inner, watch);
                }
                WatchMode::Stopwatch => {
                    render_stopwatch_display(f, watch_inner, watch);
                }
            }
        }
        WatchModel::F91W => {
            match watch.f91w_mode {
                F91WMode::Time => {
                    render_f91w_time_display(f, watch_inner, watch);
                }
                F91WMode::Alarm => {
                    render_f91w_alarm_display(f, watch_inner, watch);
                }
                F91WMode::Stopwatch => {
                    render_f91w_stopwatch_display(f, watch_inner, watch);
                }
            }
        }
    }
    
    render_status_indicators(f, watch_area, watch);
}

fn render_time_display(f: &mut Frame, area: Rect, watch: &Watch) {
    let time_text = watch.time_manager.format_time(watch.settings.time_format_24h);
    let date_text = watch.time_manager.format_date(watch.settings.date_format_us);
    let day_text = watch.time_manager.format_day_of_week();
    
    let time_display = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                time_text,
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("{} {}", day_text, date_text),
                Style::default().fg(Color::Cyan),
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from(""),
        Line::from("press 'M' for mode, 'L' for backlight"),
    ])
    .block(Block::default());
    
    f.render_widget(time_display, area);
    
    render_analog_display(f, area, watch);
}

fn render_analog_display(f: &mut Frame, area: Rect, watch: &Watch) {
    use std::f64::consts::PI;
    
    // create a small area for the analog clock in the top-left corner
    let analog_area = Rect {
        x: area.x + 2,
        y: area.y + 1,
        width: 10.min(area.width),
        height: 5.min(area.height),
    };
    
    let time = &watch.time_manager.current_time;
    let hour = time.hour() as f64;
    let minute = time.minute() as f64;
    let second = time.second() as f64;
    
    // hour hand: 360 degrees / 12 hours = 30 degrees per hour + minute adjustment
    let hour_angle = (hour % 12.0) * 30.0 + minute * 0.5;
    let _hour_angle_rad = hour_angle * PI / 180.0;
    
    // minute hand: 360 degrees / 60 minutes = 6 degrees per minute + second adjustment
    let minute_angle = minute * 6.0 + second * 0.1;
    let _minute_angle_rad = minute_angle * PI / 180.0;
    
    // second hand: 360 degrees / 60 seconds = 6 degrees per second
    let second_angle = second * 6.0;
    let _second_angle_rad = second_angle * PI / 180.0;
    
    let analog_display = Paragraph::new(vec![
        Line::from(format!("┌────────┐")),
        Line::from(format!("│  {}{}  │", 
            if hour_angle >= 270.0 || hour_angle <= 90.0 { "▲" } else { " " },
            if minute_angle >= 270.0 || minute_angle <= 90.0 { "●" } else { " " }
        )),
        Line::from(format!("│{}{} {}{}{}│",
            if hour_angle > 180.0 && hour_angle <= 360.0 { "◀" } else { " " },
            if minute_angle > 180.0 && minute_angle <= 360.0 { "●" } else { " " },
            if second_angle > 180.0 && second_angle <= 360.0 { "·" } else { " " },
            if minute_angle > 0.0 && minute_angle <= 180.0 { "●" } else { " " },
            if hour_angle > 0.0 && hour_angle <= 180.0 { "▶" } else { " " }
        )),
        Line::from(format!("│  {}{}  │",
            if minute_angle > 90.0 && minute_angle <= 270.0 { "●" } else { " " },
            if hour_angle > 90.0 && hour_angle <= 270.0 { "▼" } else { " " }
        )),
        Line::from(format!("└────────┘")),
    ])
    .style(Style::default().fg(Color::Yellow));
    
    f.render_widget(analog_display, analog_area);
}

fn render_date_display(f: &mut Frame, area: Rect, watch: &Watch) {
    let date_text = watch.time_manager.format_date(watch.settings.date_format_us);
    let day_text = watch.time_manager.format_day_of_week();
    let year_text = watch.time_manager.current_time.format("%Y").to_string();
    
    let date_display = Paragraph::new(vec![
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("{} {}", day_text, date_text),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                year_text,
                Style::default().fg(Color::Cyan),
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from(""),
        Line::from(""),
        Line::from("Press 'M' for mode, 'L' for light"),
    ])
    .block(Block::default());
    
    f.render_widget(date_display, area);
}

fn render_stopwatch_display(f: &mut Frame, area: Rect, watch: &Watch) {
    let time_text = format_stopwatch_time(watch.stopwatch_time);
    let status = if watch.stopwatch_running { "RUNNING" } else { "STOPPED" };
    
    let stopwatch_display = Paragraph::new(vec![
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "STOPWATCH",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                time_text,
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                status,
                if watch.stopwatch_running {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default().fg(Color::Blue)
                },
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from(""),
        Line::from("press 'S' start/stop, 'R' reset"),
        Line::from("press 'M' for mode, 'L' for backlight"),
    ])
    .block(Block::default());
    
    f.render_widget(stopwatch_display, area);
}

fn format_stopwatch_time(milliseconds: u64) -> String {
    let total_seconds = milliseconds / 1000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    let millis = (milliseconds % 1000) / 10;
    
    format!("{:02}:{:02}.{:02}", minutes, seconds, millis)
}

fn render_status_indicators(f: &mut Frame, area: Rect, watch: &Watch) {
    if watch.light_on {
        let light_indicator = Paragraph::new("LGT")
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Right);
        
        let light_area = Rect {
            x: area.x + area.width.saturating_sub(8),
            y: area.y + 1,
            width: 7.min(area.width),
            height: 1,
        };
        
        f.render_widget(light_indicator, light_area);
    }
    
    if watch.settings.alarm_enabled {
        let alarm_indicator = Paragraph::new("ALM")
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Left);
        
        let alarm_area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: 8.min(area.width),
            height: 1,
        };
        
        f.render_widget(alarm_indicator, alarm_area);
    }
}

fn render_world_time_display(f: &mut Frame, area: Rect, watch: &Watch) {
    let time_text = watch.time_manager.format_time(watch.settings.time_format_24h);
    
    let world_time_display = Paragraph::new(vec![
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "WT",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                time_text,
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from(""),
        Line::from("Press 'M' for mode, 'L' for backlight"),
    ])
    .block(Block::default());
    
    f.render_widget(world_time_display, area);
}

fn render_alarm_display(f: &mut Frame, area: Rect, watch: &Watch) {
    let alarm_status = if watch.settings.alarm_enabled { 
        watch.settings.alarm_time.clone().unwrap_or_else(|| "Not set".to_string())
    } else {
        "Disabled".to_string()
    };
    
    let alarm_display = Paragraph::new(vec![
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "ALM",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                alarm_status,
                Style::default().fg(Color::Cyan),
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from(""),
        Line::from("Press 'A' to toggle, 'M' for mode"),
    ])
    .block(Block::default());
    
    f.render_widget(alarm_display, area);
}

fn render_timer_display(f: &mut Frame, area: Rect, watch: &Watch) {
    let time_text = format_timer_time(watch.timer_time);
    let status = if watch.timer_running { "RUNNING" } else { "STOPPED" };
    
    let timer_display = Paragraph::new(vec![
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "TMR",
                Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                time_text,
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                status,
                if watch.timer_running {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default().fg(Color::Blue)
                },
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from(""),
        Line::from("Press 'S' start/stop, 'R' reset"),
        Line::from("Press 'M' for mode, 'L' for backlight"),
    ])
    .block(Block::default());
    
    f.render_widget(timer_display, area);
}

fn format_timer_time(milliseconds: u64) -> String {
    let total_seconds = milliseconds / 1000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    let millis = (milliseconds % 1000) / 10;
    
    format!("{:02}:{:02}.{:02}", minutes, seconds, millis)
}

fn render_f91w_time_display(f: &mut Frame, area: Rect, watch: &Watch) {
    let time_text = watch.time_manager.format_time(watch.settings.time_format_24h);
    let date_text = watch.time_manager.format_date(watch.settings.date_format_us);
    
    let time_display = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                time_text,
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                date_text,
                Style::default().fg(Color::Cyan),
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from("press 'M' for mode"),
    ])
    .block(Block::default());
    
    f.render_widget(time_display, area);
}

fn render_f91w_alarm_display(f: &mut Frame, area: Rect, watch: &Watch) {
    let alarm_status = if watch.settings.alarm_enabled { 
        watch.settings.alarm_time.clone().unwrap_or_else(|| "--:--".to_string())
    } else {
        "OFF".to_string()
    };
    
    let alarm_display = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "ALARM",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                alarm_status,
                Style::default().fg(Color::Cyan),
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from("press 'A' toggle, 'M' mode"),
    ])
    .block(Block::default());
    
    f.render_widget(alarm_display, area);
}

fn render_f91w_stopwatch_display(f: &mut Frame, area: Rect, watch: &Watch) {
    let time_text = format_stopwatch_time(watch.stopwatch_time);
    let status = if watch.stopwatch_running { "RUN" } else { "STOP" };
    
    let stopwatch_display = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "STOPWATCH",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                time_text,
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                status,
                if watch.stopwatch_running {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default().fg(Color::Blue)
                },
            )
        ]).alignment(Alignment::Center),
        Line::from(""),
        Line::from("press 'S' start/stop, 'R' reset"),
        Line::from("press 'M' for mode"),
    ])
    .block(Block::default());
    
    f.render_widget(stopwatch_display, area);
}
