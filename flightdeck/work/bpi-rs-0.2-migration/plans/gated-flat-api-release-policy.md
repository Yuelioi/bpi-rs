# Gated Flat API Release Policy

Batch: `release/gated-flat-api-policy`

Type: Non-Probe release policy documentation batch.

Status: Implemented and verified in the working tree; commit pending human approval.

## Why

The 0.2 migration has already promoted safe read contracts, added module-client bridges, and removed low-risk/deprecated/helper flat `BpiClient` methods. The current direct public async inventory under `impl BpiClient` is:

```text
COUNT=107
article=4
audio=4
bangumi=2
comment=6
creativecenter=11
danmaku=8
dynamic=7
electric=3
fav=7
historytoview=6
live=12
login=4
manga=8
message=1
note=3
user=9
video=9
vip=3
```

These remaining methods are not default safe cleanup targets. They are mutating/write operations, session/login flows, risk-control-sensitive operations, or Probe-blocked read work. In default goal-mode, they remain legacy compatibility-only flat APIs.

## Policy

No default source cleanup should remove, migrate, Probe, or contract-promote these remaining flat methods without an explicit new batch decision.

The remaining 107 flat methods are compatibility-only for this release state. They may stay in the crate while the 0.2 module-client API is reviewed, unless the user explicitly selects a follow-up batch with the safety controls below.

Future mutating or session batches require:

- explicit user selection of the module/submodule and endpoint list
- `BPI_MUTATING_TEST=1`
- account profile requirements documented before execution
- dry-run or no-op behavior where the endpoint supports it
- rollback/cleanup notes for any unavoidable side effect
- raw Probe output only under `target/bpi-probe-runs/...`
- request drafts only under `target/bpi-contract-drafts/...`
- sanitized promoted contracts and fixtures only after review
- no live side effects in default verification commands

`manga/download-read` remains Probe-blocked. Do not promote contracts or source changes from prior API `code=99` responses. Retry only after a valid current chapter id, web-reader handshake, or other working flow is identified.

`login` SMS/password/logout/sign-update and other session-sensitive methods require an explicit session-flow batch. They must not be pulled into a generic cleanup batch.

No `flightdeck/cockpit.md` update belongs to this policy batch. The cockpit remains a stable task index.

## No Default Source Cleanup

Default goal-mode continuation is exhausted for low-risk direct flat API cleanup. The next source-changing work must be one of:

- an explicitly enabled mutating/write module batch
- an explicitly enabled session/login flow batch
- a successful current `manga/download-read` Probe path
- a user-approved API compatibility decision to keep or remove selected legacy flat methods

## Verification Plan

```powershell
rg -n "release/gated-flat-api-policy|COUNT=107|BPI_MUTATING_TEST|manga/download-read|compatibility-only|No default source cleanup" flightdeck\work\bpi-rs-0.2-migration\plans\gated-flat-api-release-policy.md flightdeck\work\bpi-rs-0.2-migration\plans\batch-todolist.md flightdeck\work\bpi-rs-0.2-migration\index.md flightdeck\work\bpi-rs-0.2-migration\migration-status.md
git diff --check
git diff -- flightdeck/cockpit.md
git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
```

Cargo gates are not required unless Rust source, tests, contracts, or compiled rustdoc examples change.

## Observed Verification

```text
rg -n "release/gated-flat-api-policy|COUNT=107|BPI_MUTATING_TEST|manga/download-read|compatibility-only|No default source cleanup" flightdeck\work\bpi-rs-0.2-migration\plans\gated-flat-api-release-policy.md flightdeck\work\bpi-rs-0.2-migration\plans\batch-todolist.md flightdeck\work\bpi-rs-0.2-migration\index.md flightdeck\work\bpi-rs-0.2-migration\migration-status.md
  found the policy batch marker, COUNT=107 inventory, BPI_MUTATING_TEST gate, manga/download-read block, compatibility-only policy, and No default source cleanup language.

git diff --check
  exited 0 with only existing LF/CRLF conversion warnings.

git diff -- flightdeck/cockpit.md
  no diff.

git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
  migration-status.md and target/ remain ignored.
```
