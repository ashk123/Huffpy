all: huffpy

setup:
	python -m venv .env
	source .env/bin/activate
	pip install maturin
	maturin init

huffpy:
	maturin develop