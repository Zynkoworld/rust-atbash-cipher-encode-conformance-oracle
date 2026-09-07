# zynko-oracle · `rust-atbash-cipher-encode-conformance-oracle`

**A deterministic, re-checkable conformance oracle for `atbash-cipher` (rust).**

## Proven
Measured on the canonical Exercism corpus — **8 input/output pairs, 7 distinct outputs** — produced by *running* the reference in a sealed sandbox, not asserted.

## Scope (declared)
The corpus is the canonical Exercism test data for `atbash-cipher`. Inputs outside that set are **not covered**; this oracle decides agreement on the published corpus only and makes no claim of general correctness.

## Provenance
Reference: the Exercism reference solution for `atbash-cipher` (rust; MIT, Exercism), body unchanged. Proven by the exercism testsuite (pin=1d3f38ddb462a36c), re-executed by harvest in a sealed sandbox (unshare -rn) before this bundle was generated.

## License
Apache-2.0 for the scaffolding; the reference body retains its upstream MIT (Exercism) license.
