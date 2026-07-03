# Cockpit — bpi-rs

Focus: `work/bpi-rs-0.2-migration/` — staged 0.2 redesign for a high-quality Rust SDK.

## In flight

- `work/bpi-rs-0.2-migration/` — staged 0.2 migration is active; goal-mode continuation now defaults to Probe-backed endpoint contract batches unless an explicit non-Probe bridge batch is selected.

## Next

- Choose the next incomplete module or cohesive submodule from `flightdeck/work/bpi-rs-0.2-migration/migration-status.md`. For normal endpoint migration, follow `api-upgrade-protocol.md`: draft under `target`, run Probe, promote reviewed `contract.json` plus sanitized fixtures, then update Rust/tests. A remaining endpoint-candidate audit found no new safe read batch; `manga/download-read` is Probe-blocked by repeated API `code = 99` results. Before continuing, identify a new safe read endpoint, a valid manga download flow/chapter/handshake, an explicitly enabled gated/mutating batch, or explicitly record non-Probe bridge work.

## Open questions
