pub mod calendar;
pub mod event;
pub mod notifier;

use std::error::Error;

pub use calendar::*;
pub use event::*;
pub use notifier::*;

pub fn run(
    calenders: Vec<Box<dyn Calendar>>,
    notifiers: Vec<Box<dyn Notifier>>,
) -> Result<(), Box<dyn Error>> {
    let events = calenders
        .into_iter()
        .map(|c| c.get_events_today())
        .collect::<Result<Vec<Vec<Event>>, Box<dyn Error>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<Event>>();

    let summary = EventSummary::from(events);

    notifiers.into_iter().try_for_each(|n| n.notify(&summary))
    // .map(|n| n.notify(&summary))
    // .collect::<Result<(), Box<dyn Error>>>()
}
