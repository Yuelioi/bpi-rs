# bpi-rs 0.2 API Migration Goal

## Objective

Upgrade bpi-rs APIs in high-quality, maintainable module batches.

The migration must use real Probe results to validate request contracts, typed params, response models, error behavior, tests, and future bpi-go reference material.

## Required Protocol

Before doing implementation work, read and follow:

```text
flightdeck/work/bpi-rs-0.2-migration/api-upgrade-protocol.md
flightdeck/work/bpi-rs-0.2-migration/plans/batch-todolist.md
```

`api-upgrade-protocol.md` is the source of truth for:

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

`plans/batch-todolist.md` is the goal-mode execution entry point. It controls how
work is selected, recorded, batched, verified, and committed so continuation does
not degrade into small function edits plus repeated tiny commits.

## Goal-Mode Continuation

When this topic is resumed through a broad goal such as `执行 work/bpi-rs-0.2-migration`, treat this file, `api-upgrade-protocol.md`, and `plans/batch-todolist.md` as the execution contract, then use `migration-status.md` only as the local progress board.

Default continuation is a complete reviewable batch, not an endpoint-sized or function-sized edit. Before code changes, select or add the next batch in `plans/batch-todolist.md`, record it in `migration-status.md`, and keep the batch scope large enough to be useful but small enough to review. Run TDD/focused checks inside the batch, then run full gates once and commit once after deck/status updates.

Default batch type remains a Probe-backed endpoint contract batch: choose a module or cohesive submodule that is not already complete, prepare drafts under `target/bpi-contract-drafts/...`, run real Probe profiles, keep raw Probe output under `target/bpi-probe-runs/...`, promote reviewed `contract.json` plus sanitized `responses/*.json`, update Rust params/models/tests from the observed behavior, verify, update the local status board, and commit the reviewed source/contracts.

Do not repeat a completed contract batch just because it is named as an example. For example, `video/info-read`, `login/read-info`, and `clientinfo/ip` already have promoted endpoint contracts in this workspace.

A non-Probe shared-core or domain-client bridge batch is allowed only when it is explicitly selected as such and it reuses already promoted contracts. In that case, record in `migration-status.md` that the batch is not an endpoint contract batch, that no Probe run is expected, and which existing contracts prove the request/response behavior.

Do not resume a completed batch from `plans/batch-todolist.md`. If the listed current batch is already committed, start by choosing one of the later candidate batches or by adding a new batch section with endpoints/contracts/exclusions and verification plan before editing Rust code.

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
- Do not commit every few functions. Commit once per completed batch after focused and full verification.
- Do not edit `migration-status.md` as a substitute for a real batch TODO. Goal-mode execution must keep `plans/batch-todolist.md` current enough for the next worker to continue.
- Do not write final contracts as guessed JSON.
- Do not store raw Probe output in `flightdeck/work`.
- Do not commit raw Probe output or account-sensitive data.
- Do not proceed with a module batch until the batch scope, Probe plan, and split/failure strategy are clear.
- If protocol and current instinct conflict, follow the protocol or stop and ask.
- Continue future batches using the endpoint fixture shape above.

