# circulant

| id | value | detail |
| --- | --- | --- |
| `circulant.q_from_ac` | `5.000000000000000e-1` | q_from_ac(1, 1/2) = 1/2 |
| `circulant.q_identity_residual` | `0.000000000000000e0` | \|Q_signed - Q(a,c)\| |
| `circulant.ac_delta_roundtrip` | `2.775557561562891e-16` | max inverse-DFT parameter error |
| `circulant.q_koide` | `0.000000000000000e0` | Q(m_e, m_mu, m_tau) - 2/3 |
| `circulant.tau_prediction` | `1.776969027083014e3` | predicted m_tau (MeV) |
| `circulant.fit_r` | `7.071067811865478e-1` | c/a of fitted charged leptons |
| `circulant.fit_max_residual` | `1.354472090042691e-14` | max \|fitted - sqrt(m)\| |
| `circulant.fit_q_from_ac` | `3.330669073875470e-16` | Q_from_ac - Q of fitted masses |
| `circulant.fit_all_positive` | `1.000000000000000e0` | all fitted roots >= 0 |
| `circulant.q_physical_signed` | `3.750000000000000e-1` | Q from \|roots\| of test circulant |

## Provenance

- Generator: `fluid-universe-rust` (Rust edition 2024, no external crates).
- In-process determinism fingerprint (not G-001 evidence): run1=00ba849ebff85be4, run2=00ba849ebff85be4, agreement=PASS.
- Check-results FNV-1a 64-bit fingerprint: `078de436e785c02e` (not a configuration hash or cryptographic digest).
