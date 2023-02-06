#[cfg(test)]
mod tests {
    use std::error::Error;

    use rustinaction::other::{
        calendar::Calendar,
        event::{Event, EventSummary},
        notifier::Notifier,
        run,
    };

    struct MockCalendar;
    impl Calendar for MockCalendar {
        fn get_events_today(&self) -> Result<Vec<Event>, Box<dyn Error>> {
            Ok(vec![
                Event::new("Mock Calc1".into(), "mock".into()),
                Event::new("Mock calc2".into(), "mock".into()),
                Event::new("Mock calc3".into(), "mock".into()),
            ])
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
