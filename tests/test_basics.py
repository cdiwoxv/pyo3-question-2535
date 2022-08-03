from pyo3_question import Client, Driver, EventA, EventB

class MyClient(Client):
    def handle_event(event):
        match event:
            case EventA(field_a1=a1, field_a2=a2): print(f"got A(a1={a1}, a2={a2})")
            case EventB(field_b1=b1, field_b2=b2): print(f"got B(b1={b1}, b2={b2})")

def test_it_does_not_crash():
    driver = Driver([MyClient()])
    driver.emit_event_a(100, False)
    driver.emit_event_b(3.4, 20)

if __name__ == '__main__':
    test_it_does_not_crash()