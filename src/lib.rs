use pyo3::prelude::*;

#[pyclass]
#[derive(Debug)]
pub struct EventA {
    pub field_a1: i64,
    pub field_a2: bool,
}

#[pyclass]
#[derive(Debug)]
pub struct EventB {
    pub field_b1: f64,
    pub field_b2: i32,
}

#[derive(Debug)]
pub enum Event {
    EventA(EventA),
    EventB(EventB),
}

pub trait Client {
    fn handle_event(&mut self, event: &Event);
}

pub struct Driver {
    clients: Vec<Box<dyn Client>>,
}

impl Driver {
    pub fn new(clients: Vec<Box<dyn Client>>) -> Self {
        Self { clients }
    }

    fn emit_event(&mut self, event: Event) {
        for client in self.clients.iter_mut() {
            client.handle_event(&event)
        }
    }

    pub fn emit_event_a(&mut self, field_a1: i64, field_a2: bool) {
        let event = Event::EventA(EventA { field_a1, field_a2 });
        self.emit_event(event)
    }

    pub fn emit_event_b(&mut self, field_b1: f64, field_b2: i32) {
        let event = Event::EventB(EventB { field_b1, field_b2 });
        self.emit_event(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client_1() -> impl Client {
        struct C {}
        impl Client for C {
            fn handle_event(&mut self, event: &Event) {
                println!("client 1 received {:?}", event)
            }
        }
        C {}
    }
    fn client_2() -> impl Client {
        struct C {}
        impl Client for C {
            fn handle_event(&mut self, event: &Event) {
                println!("client 2 received {:?}", event)
            }
        }
        C {}
    }


    #[test]
    fn it_works() {
        let mut driver = Driver::new(vec![Box::new(client_1()), Box::new(client_2())]);

        driver.emit_event_a(100, false);
        driver.emit_event_b(3.4, 42);
    }
}
