# kendalls-py

To run the test, run the following commands inside a virtual environment (`python3 -m venv venv && source ./venv/bin/activate`).

```shell
pip install --upgrade pip
pip install maturin
pip install -r ./requirements.txt
maturin develop -b pyo3
pytest ./tests.py
```