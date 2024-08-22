//! A simple observer pattern implementation.
//!

use std::collections::HashMap;
use std::hash::Hash;

/// Subscriber function type
pub type Subscriber<T> = fn(event: &T);

/// Publisher
#[derive(Default, Debug)]
pub struct Publisher<T> {
    events: HashMap<T, Vec<Subscriber<T>>>,
}

impl<T> Publisher<T>
where
    T: PartialEq + Eq + Hash + Copy,
{
    /// Subscribe to an event
    pub fn subscribe(&mut self, event: T, listener: Subscriber<T>) {
        self.events.entry(event).or_default().push(listener);
        self.events.get_mut(&event.clone()).unwrap().push(listener);
    }

    /// Unsubscribe from an event
    pub fn unsubscribe(&mut self, event: T, listener: Subscriber<T>) {
        self.events
            .get_mut(&event)
            .unwrap()
            .retain(|l| *l != listener);
    }

    /// Notify subscribers of an event
    pub fn notify(&self, event: T) {
        if let Some(listeners) = self.events.get(&event) {
            for listener in listeners {
                listener(&event);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Eq, Hash, Copy, Clone, Default, Debug)]
    enum Event {
        #[default]
        Event1,
        Event2,
    }

    #[test]
    fn test_observer() {
        let mut publisher = Publisher::default();

        publisher.subscribe(Event::Event1, event_1_listener);
        publisher.subscribe(Event::Event2, event_2_listener);

        publisher.notify(Event::Event1);

        publisher.notify(Event::Event2);

        publisher.unsubscribe(Event::Event1, event_1_listener);

        // Nothing should print
        publisher.notify(Event::Event1);
    }

    fn event_1_listener(event: &Event) {
        println!("Event1 listener");
        assert_eq!(*event, Event::Event1);
    }

    fn event_2_listener(event: &Event) {
        println!("Event1 listener");
        assert_eq!(*event, Event::Event2);
    }
}
