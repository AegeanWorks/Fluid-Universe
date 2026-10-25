PYTHON ?= python

.PHONY: help test results verify clean

help:
	@echo "make test     - run the regression suite"
	@echo "make results  - regenerate every number in results/"
	@echo "make verify   - re-derive every verified claim independently"
	@echo "make clean    - remove caches and generated results"

test:
	$(PYTHON) -m pytest -q

results:
	$(PYTHON) -m phenomenology.run_all

verify:
	$(PYTHON) -m verification.verify

clean:
	$(PYTHON) -c "import pathlib, shutil; [shutil.rmtree(p, ignore_errors=True) for p in pathlib.Path('.').rglob('__pycache__')]"
	$(PYTHON) -c "import pathlib; [p.unlink() for p in pathlib.Path('results').glob('*.json')]"
	$(PYTHON) -c "import pathlib; [p.unlink() for p in pathlib.Path('results').glob('*.md') if p.name != 'README.md']"
