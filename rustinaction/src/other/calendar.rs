use std::error::Error;

use super::Event;

pub trait Calendar {
    fn get_events_today(&self) -> Result<Vec<Event>, Box<dyn Error>>;
}

pub struct Outlook;

impl Calendar for Outlook {
    fn get_events_today(&self) -> Result<Vec<Event>, Box<dyn Error>> {
        Ok(vec![
            Event::new("Outlook Calc1".into(), "outlook".into()),
            Event::new("Outlook calc2".into(), "outlook".into()),
            Event::new("Outlook calc3".into(), "outlook".into()),
        ])
    }
}

pub struct GoogleGalendar;

impl Calendar for GoogleGalendar {
    fn get_events_today(&self) -> Result<Vec<Event>, Box<dyn Error>> {
        Ok(vec![
            Event::new("Google Calc1".into(), "google".into()),
            Event::new("Google calc2".into(), "google".into()),
            Event::new("Google calc3".into(), "google".into()),
        ])
    }
}
