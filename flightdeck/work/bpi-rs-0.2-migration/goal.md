# bpi-rs 0.2 API Migration Goal

## Objective

Upgrade bpi-rs APIs in high-quality, maintainable module batches.

The migration must use real Probe results to validate request contracts, typed params, response models, error behavior, tests, and future bpi-go reference material.

## Required Protocol

Before doing implementation work, read and follow:

```text
flightdeck/work/bpi-rs-0.2-migration/api-upgrade-protocol.md
```

That protocol is the source of truth for:

- batch granularity
- Probe account profiles
- request draft policy
- raw artifact locations
- Probe failure handling
- contract promotion
- Rust model and params upgrade rules
- test requirements
- commit policy
- done definition

## Current Contract Mode

The accepted contract shape is:

```text
tests/contracts/<domain>/<endpoint>/contract.json
tests/contracts/<domain>/<endpoint>/responses/<case>.json
```

`contract.json` owns the stable endpoint request and profile-specific cases. Each case records the expected API code, semantic error when applicable, response fixture path, fixture kind, and declared Rust model for success responses.

Successful response fixtures must parse through the declared Rust model. Error fixtures must validate the observed semantic error, such as `requires_login`.

The previous request-only shape, `tests/contracts/<domain>/<endpoint>/<profile>.request.json`, is deprecated for promoted endpoint contracts. The legacy `ApiContract` format remains available only for Probe flow steps and draft execution where needed.

## Latest Accepted Batch

Commit `6383119` migrated the current pilot contracts to the accepted endpoint fixture shape:

- `clientinfo/ip`
- `login/vip-info`
- `login/read-info` endpoints: `account-info`, `coin`, `nav`, `stat`, `today-coin-exp`
- `login/qr` generate, poll, and flow contracts

Verification for that batch:

```powershell
cargo fmt --check
cargo check --all-features
cargo test --all-features --lib
```

`cargo test --all-features --lib` passed with 358 passed, 0 failed, 302 ignored.

## Execution Guardrails

- Do not migrate endpoint-by-endpoint by default.
- Do not create hundreds of tiny commits.
- Do not write final contracts as guessed JSON.
- Do not store raw Probe output in `flightdeck/work`.
- Do not commit raw Probe output or account-sensitive data.
- Do not proceed with a module batch until the batch scope, Probe plan, and split/failure strategy are clear.
- If protocol and current instinct conflict, follow the protocol or stop and ask.
- Continue future batches using the endpoint fixture shape above.

