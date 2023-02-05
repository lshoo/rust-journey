use std::error::Error;

use event::Event;

pub trait Calendar {
    fn get_events_today(&self) -> Result<Vec<Event>, Box<dyn Error>>;
}

pub struct Outlook;

impl Calendar for Outlook {
    fn get_events_today(&self) -> Result<Vec<Event>, Box<dyn Error>> {
        Ok(vec![Event::new("Outlook Calc1".into(), "outlook".into()), Event::new("Outlook calc2".into(), "outlook".into()), Event::new("Outlook calc3".into(), "outlook".into())])
    }
}

pub struct GoogleGalendar;

impl Calendar for GoogleGalendar {
    fn get_events_today(&self) -> Result<Vec<Event>, Box<dyn Error>> {
        Ok(vec![Event::new("Google Calc1".into(), "google".into()), Event::new("Google calc2".into(), "google".into()), Event::new("Google calc3".into(), "google".into())])
    }
}

pub mod event {

    #[derive(Debug)]
    pub struct Event {
        name: String,
        kind: String,
    }

    impl Event {
        pub fn new(name: String, kind: String) -> Self {
            Self { name, kind }
        }
    }

    pub struct EventSummary(Vec<Event>);

    impl EventSummary {
        pub fn show(&self) {
            self.0.iter().for_each(|e| println!("{:?} name: {:?}, kind: {:?}", e, e.name, e.kind));
        }
    }

    impl From<Vec<Event>> for EventSummary {
        fn from(events: Vec<Event>) -> Self {
            Self(events)
        }
    }
}