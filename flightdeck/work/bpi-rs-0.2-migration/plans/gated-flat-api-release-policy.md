# Gated Flat API Release Policy

Batch: `release/gated-flat-api-policy`

Type: Non-Probe release policy documentation batch.

Status: Superseded by `flat-api/remove-remaining-legacy-flat-methods` and
`48cd13c feat(api): move legacy write surface to module clients`.

## Why

This policy was written when the 0.2 migration had already promoted safe read contracts,
added module-client bridges, and removed low-risk/deprecated/helper flat `BpiClient`
methods. At that point, the direct public async inventory under `impl BpiClient` was:

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

Those remaining methods were not default safe cleanup targets. They were mutating/write
operations, session/login flows, risk-control-sensitive operations, or unavailable
reader-proof work.

## Current State

This policy is now historical. The later breaking cleanup removed the remaining direct
flat methods, and `48cd13c` restored the old write/session/mutating capability surface on
module clients.

Current release state:

- direct `BpiClient` flat async inventory is `COUNT=0`;
- old write/session/mutating method names now live on module clients;
- future work should target module clients, shared internals, docs, or an explicitly
  approved gated batch.

## Gated Work

Future mutating or session batches still require:

- explicit user selection of the module/submodule and endpoint list;
- `BPI_MUTATING_TEST=1`;
- account profile requirements documented before execution;
- dry-run or no-op behavior where the endpoint supports it;
- rollback/cleanup notes for any unavoidable side effect;
- raw Probe output only under `target/bpi-probe-runs/...`;
- request drafts only under `target/bpi-contract-drafts/...`;
- sanitized promoted contracts and fixtures only after review;
- no live side effects in default verification commands.

`manga/download-read` is not implemented in the current migration. Do not promote
contracts or source changes from prior API `code=99` responses. Reopen it only as a
dedicated proof-provider/API-design batch for the current reader `m2`/`m1` fields.

`login` SMS/password/logout/sign-update and other session-sensitive methods require an
explicit session-flow batch. They must not be pulled into a generic cleanup batch.

No `flightdeck/cockpit.md` update belongs to this policy. The cockpit remains a stable task
index.

## No Default Source Cleanup

Default goal-mode continuation is exhausted for low-risk direct flat API cleanup because
there are no direct public async flat methods left on `BpiClient`. The next source-changing
work must be one of:

- an explicitly enabled mutating/write module batch;
- an explicitly enabled session/login flow batch;
- a user-approved `manga/download-read` proof-provider/API-design batch;
- a user-approved API design/cleanup batch over module-client names, wrappers,
  static/local helpers, or release docs.

## Verification Plan

```powershell
rg -n "release/gated-flat-api-policy|COUNT=0|BPI_MUTATING_TEST|manga/download-read|No default source cleanup|Superseded" flightdeck\work\bpi-rs-0.2-migration\plans\gated-flat-api-release-policy.md flightdeck\work\bpi-rs-0.2-migration\plans\batch-todolist.md flightdeck\work\bpi-rs-0.2-migration\index.md
git diff --check
git diff -- flightdeck/cockpit.md
git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
```

Cargo gates are not required unless Rust source, tests, contracts, or compiled rustdoc
examples change.
