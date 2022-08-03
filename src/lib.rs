use pyo3::exceptions::PyNotImplementedError;
use pyo3::prelude::*;
use pyo3::ToPyObject;

#[derive(Debug)]
pub struct EventA {
    pub field_a1: i64,
    pub field_a2: bool,
}

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

impl ToPyObject for Event {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match *self {
            Event::EventA(EventA { field_a1, field_a2 }) => {
                let py_event = PyEventA { field_a1, field_a2 };
                py_event.into_py(py)
            }
            Event::EventB(EventB { field_b1, field_b2 }) => {
                let py_event = PyEventB { field_b1, field_b2 };
                py_event.into_py(py)
            }
        }
    }
}

#[pyclass(name = "EventA")]
struct PyEventA {
    #[pyo3(get, set)]
    pub field_a1: i64,
    #[pyo3(get, set)]
    pub field_a2: bool,
}

#[pyclass(name = "EventB")]
struct PyEventB {
    #[pyo3(get, set)]
    pub field_b1: f64,
    #[pyo3(get, set)]
    pub field_b2: i32,
}

#[pyclass(name = "Client", subclass)]
struct PyClient {}

#[pymethods]
impl PyClient {
    #[new]
    fn new() -> Self {
        Self {}
    }

    fn handle_event(&mut self, _event: &PyAny) -> PyResult<()> {
        Err(PyNotImplementedError::new_err("not implemented"))
    }
}

impl Client for Py<PyClient> {
    fn handle_event(&mut self, event: &Event) {
        Python::with_gil(|py| {
            let py_event = event.to_object(py);
            // TODO: error handling
            self.call_method1(py, "handle_event", (py_event.as_ref(py),)).unwrap();
        });
    }
}

#[pyclass(name = "Driver", unsendable)]
struct PyDriver {
    inner: Driver,
}

#[pymethods]
impl PyDriver {
    #[new]
    fn new(_py: Python<'_>, clients: Vec<Py<PyClient>>) -> PyResult<Self> {
        let clients: Vec<_> = clients
            .into_iter()
            .map(|c| Box::new(c) as Box<dyn Client>)
            .collect();
        Ok(Self {
            inner: Driver::new(clients),
        })
    }

    fn emit_event_a(&mut self, field_a1: i64, field_a2: bool) {
        self.inner.emit_event_a(field_a1, field_a2)
    }

    fn emit_event_b(&mut self, field_b1: f64, field_b2: i32) {
        self.inner.emit_event_b(field_b1, field_b2)
    }
}

#[pymodule]
fn pyo3_question(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyClient>()?;
    m.add_class::<PyDriver>()?;
    m.add_class::<PyEventA>()?;
    m.add_class::<PyEventB>()?;
    Ok(())
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