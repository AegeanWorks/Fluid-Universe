PYTHON ?= python

.PHONY: help test results verify n64 scale stability production convergence temporal extend purefn escape lengthscan vortex cascade rotorpotential madelung nelson gaussianfluid coherentfluid excitednode hydrogen hydrogenvortex wateranalogue watervalidity waterdispersive waterrays waterscattering waterelectron waterwell clean

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
	@echo "make extend   - one N=96 checkpointed chunk (<15 min)"
	@echo "make purefn   - pure-FN N=64 candidate Hessian (<15 min)"
	@echo "make escape   - saddle-escape search from the pure-FN candidate (<15 min)"
	@echo "make lengthscan - pure-FN Hessian at lengths 8,10,12,14 (<15 min)"
	@echo "make vortex   - vortex-knot mass spectrum test (claim 17)"
	@echo "make cascade  - one-parameter vortex hierarchy test (claim 18)"
	@echo "make rotorpotential - real Z3 rotor potential scan (claim 19)"
	@echo "make madelung - Madelung fluid dictionary validation (claim 20)"
	@echo "make nelson   - Nelson stochastic kinematics test (claim 21)"
	@echo "make gaussianfluid - Gaussian Madelung reduction (claim 22)"
	@echo "make coherentfluid - moving-center Gaussian reduction (claim 23)"
	@echo "make excitednode - first-excited-state node singularity (claim 24)"
	@echo "make hydrogen - hydrogen 1s Madelung reduction (claim 25)"
	@echo "make hydrogenvortex - hydrogen 2p vortex identification (claim 26)"
	@echo "make wateranalogue - shallow-water acoustic horizon (claim 27)"
	@echo "make watervalidity - capillary validity condition (claim 28)"
	@echo "make waterdispersive - dispersive water horizon (claim 29)"
	@echo "make waterrays - radial ray separatrix (claim 30)"
	@echo "make waterscattering - 1D wave scattering at the horizon (claim 31)"
	@echo "make waterelectron - hydrated-electron cavity scale (claim 32)"
	@echo "make waterwell - finite-well test of the 1.7 eV absorption (claim 33)"
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

extend:
	$(PYTHON) -m phenomenology.extension_report

purefn:
	$(PYTHON) -m phenomenology.purefn_stability_report

escape:
	$(PYTHON) -m phenomenology.saddle_escape_report

lengthscan:
	$(PYTHON) -m phenomenology.length_scan_report

vortex:
	$(PYTHON) -m phenomenology.vortex_spectrum_report

cascade:
	$(PYTHON) -m phenomenology.vortex_cascade_report

rotorpotential:
	$(PYTHON) -m phenomenology.rotor_potential_report

madelung:
	$(PYTHON) -m phenomenology.madelung_report

nelson:
	$(PYTHON) -m phenomenology.nelson_report

gaussianfluid:
	$(PYTHON) -m phenomenology.gaussian_fluid_report

coherentfluid:
	$(PYTHON) -m phenomenology.coherent_fluid_report

excitednode:
	$(PYTHON) -m phenomenology.excited_fluid_report

hydrogen:
	$(PYTHON) -m phenomenology.hydrogen_fluid_report

hydrogenvortex:
	$(PYTHON) -m phenomenology.hydrogen_vortex_report

wateranalogue:
	$(PYTHON) -m phenomenology.water_analogue_report

watervalidity:
	$(PYTHON) -m phenomenology.water_validity_report

waterdispersive:
	$(PYTHON) -m phenomenology.water_dispersive_report

waterrays:
	$(PYTHON) -m phenomenology.water_rays_report

waterscattering:
	$(PYTHON) -m phenomenology.water_scattering_report

waterelectron:
	$(PYTHON) -m phenomenology.water_electron_report

waterwell:
	$(PYTHON) -m phenomenology.water_well_report

clean:
	$(PYTHON) -c "import pathlib, shutil; [shutil.rmtree(p, ignore_errors=True) for p in pathlib.Path('.').rglob('__pycache__')]"
	$(PYTHON) -c "import pathlib; [p.unlink() for p in pathlib.Path('results').glob('*.json') if p.name not in {'hopfion3d_n64.json', 'hopfion_scale.json', 'hopfion_stability.json', 'hopfion3d_production.json', 'hopfion3d_convergence.json', 'hopfion3d_temporal.json', 'hopfion3d_extension.json', 'hopfion_purefn_stability.json', 'hopfion_saddle_escape.json', 'hopfion_length_scan.json', 'vortex_knot_spectrum.json', 'vortex_cascade.json', 'rotor_potential.json', 'madelung.json', 'nelson.json', 'gaussian_fluid.json', 'coherent_fluid.json', 'excited_fluid.json', 'hydrogen_fluid.json', 'hydrogen_vortex.json', 'water_analogue.json', 'water_validity.json', 'water_dispersive.json', 'water_rays.json', 'water_scattering.json', 'water_electron.json', 'water_well.json'}]"
	$(PYTHON) -c "import pathlib; [p.unlink() for p in pathlib.Path('results').glob('*.md') if p.name not in {'README.md', 'hopfion3d_n64.md', 'hopfion_scale.md', 'hopfion_stability.md', 'hopfion3d_production.md', 'hopfion3d_convergence.md', 'hopfion3d_temporal.md', 'hopfion3d_extension.md', 'hopfion_purefn_stability.md', 'hopfion_saddle_escape.md', 'hopfion_length_scan.md', 'vortex_knot_spectrum.md', 'vortex_cascade.md', 'rotor_potential.md', 'madelung.md', 'nelson.md', 'gaussian_fluid.md', 'coherent_fluid.md', 'excited_fluid.md', 'hydrogen_fluid.md', 'hydrogen_vortex.md', 'water_analogue.md', 'water_validity.md', 'water_dispersive.md', 'water_rays.md', 'water_scattering.md', 'water_electron.md', 'water_well.md'}]"
