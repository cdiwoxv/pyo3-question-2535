To run the test:

```
python3.10 -m venv venv
. ./venv/bin/activate
pip install maturin
maturin dev
python3.10 tests/test_basics.py
```