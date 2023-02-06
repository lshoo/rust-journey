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
        self.0
            .iter()
            .for_each(|e| println!("{:?} name: {:?}, kind: {:?}", e, e.name, e.kind));
    }
}

impl From<Vec<Event>> for EventSummary {
    fn from(events: Vec<Event>) -> Self {
        Self(events)
    }
}
