PYTHON ?= python

.PHONY: help test results verify n64 scale stability production convergence temporal clean

help:
	@echo "make test     - run the regression suite"
	@echo "make results  - regenerate every number in results/"
	@echo "make verify   - re-derive every verified claim independently"
	@echo "make n64      - N=64 diagnostic (slow)"
	@echo "make scale    - scale-iteration candidate (slow)"
	@echo "make stability- validated N=64 tangent Hessian (slow)"
	@echo "make production - N=64 claim-15 production scan (<15 min)"
	@echo "make convergence - N=64 vs N=96 resolution check (<15 min)"
	@echo "make temporal - N=64 dt check at the same physical time (<15 min)"
	@echo "make clean    - remove caches and generated results"

test:
	$(PYTHON) -m pytest -q

results:
	$(PYTHON) -m phenomenology.run_all

verify:
	$(PYTHON) -m verification.verify

n64:
	$(PYTHON) -m phenomenology.hopfion3d_n64_report

scale:
	$(PYTHON) -m phenomenology.hopfion_scale_report

stability:
	$(PYTHON) -m phenomenology.stability_report

production:
	$(PYTHON) -m phenomenology.production_report

convergence:
	$(PYTHON) -m phenomenology.convergence_report

temporal:
	$(PYTHON) -m phenomenology.temporal_report

clean:
	$(PYTHON) -c "import pathlib, shutil; [shutil.rmtree(p, ignore_errors=True) for p in pathlib.Path('.').rglob('__pycache__')]"
	$(PYTHON) -c "import pathlib; [p.unlink() for p in pathlib.Path('results').glob('*.json') if p.name not in {'hopfion3d_n64.json', 'hopfion_scale.json', 'hopfion_stability.json', 'hopfion3d_production.json', 'hopfion3d_convergence.json', 'hopfion3d_temporal.json'}]"
	$(PYTHON) -c "import pathlib; [p.unlink() for p in pathlib.Path('results').glob('*.md') if p.name not in {'README.md', 'hopfion3d_n64.md', 'hopfion_scale.md', 'hopfion_stability.md', 'hopfion3d_production.md', 'hopfion3d_convergence.md', 'hopfion3d_temporal.md'}]"
