## Python async bindings

Python bindings for the Rust library.  
Built with pyo3 and pyo3_asyncio.

Usage:  
```bash
# setup enviroment
python -m venv .env
source .env/bin/activate
pip install maturin
maturin develop

# run examples
cd ./examples
python <example.py>
```