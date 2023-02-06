use std::error::Error;

use super::event::EventSummary;

pub trait Notifier {
    fn notify(&self, summ: &EventSummary) -> Result<(), Box<dyn Error>>;
}

pub struct Slack;
impl Notifier for Slack {
    fn notify(&self, summ: &EventSummary) -> Result<(), Box<dyn Error>> {
        summ.show();
        Ok(())
    }
}

pub struct Sms;
impl Notifier for Sms {
    fn notify(&self, summ: &EventSummary) -> Result<(), Box<dyn Error>> {
        summ.show();
        Ok(())
    }
}

pub struct Discord;
impl Notifier for Discord {
    fn notify(&self, summ: &EventSummary) -> Result<(), Box<dyn Error>> {
        summ.show();
        Ok(())
    }
}
