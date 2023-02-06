//! cargo run -p rustinaction --example  other

use std::error::Error;

use rustinaction::other::notifier::{Discord, Notifier, Slack, Sms};
use rustinaction::other::{
    calendar::{Calendar, GoogleGalendar, Outlook},
    run,
};

pub fn main() -> Result<(), Box<dyn Error>> {
    let calendars: Vec<Box<dyn Calendar>> = vec![Box::new(Outlook), Box::new(GoogleGalendar)];
    let notifiers: Vec<Box<dyn Notifier>> = vec![Box::new(Sms), Box::new(Slack), Box::new(Discord)];
    run(calendars, notifiers)
}
