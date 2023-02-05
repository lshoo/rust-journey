use std::error::Error;

mod calendar;
mod notifier;

use calendar::{Calendar, Outlook, GoogleGalendar, event::{Event, EventSummary}};
use notifier::{Notifier, Sms, Slack, Discord};


pub fn main() -> Result<(), Box<dyn Error>> {
    let calendars: Vec<Box<dyn Calendar>> = vec![Box::new(Outlook), Box::new(GoogleGalendar)];
    let notifiers: Vec<Box<dyn Notifier>> = vec![Box::new(Sms), Box::new(Slack), Box::new(Discord)];
    run(calendars, notifiers)
}

pub fn run(
    calenders: Vec<Box<dyn Calendar>>,
    notifiers: Vec<Box<dyn Notifier>>,
) -> Result<(), Box<dyn Error>> {
    let events = calenders.into_iter()
        .map(|c| c.get_events_today())
        .collect::<Result<Vec<Vec<Event>>, Box<dyn Error>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<Event>>();

    let summary = EventSummary::from(events);

    notifiers.into_iter()
        .try_for_each(|n| n.notify(&summary))
        // .map(|n| n.notify(&summary))
        // .collect::<Result<(), Box<dyn Error>>>()
}

#[cfg(test)] 
mod tests {
    use super::*;
    struct MockCalendar;
    impl Calendar for MockCalendar {
        fn get_events_today(&self) -> Result<Vec<Event>, Box<dyn Error>> {
            Ok(vec![Event::new("Mock Calc1".into(), "mock".into()), Event::new("Mock calc2".into(), "mock".into()), Event::new("Mock calc3".into(), "mock".into())])
        }
    }

    struct MockNotifier;
    impl Notifier for MockNotifier {
        fn notify(&self, _summ: &EventSummary) -> Result<(), Box<dyn Error>> {
            Ok(())
        }
    }

    #[test]
    fn it_works() {
        let result = run(vec![Box::new(MockCalendar)], vec![Box::new(MockNotifier)]);
        assert!(result.is_ok());
        // assert_eq!(result, Ok(()));
        // match result {
        //     Ok(_) => (),
        //     err => panic!("Expected Ok, got {:#?}", err),
        // }
    }
}