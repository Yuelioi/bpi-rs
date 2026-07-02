# bpi-rs API Upgrade Protocol

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans before implementing this protocol task-by-task. Use rust-best-practices and rust-testing whenever Rust code changes.

**Goal:** Upgrade bpi-rs APIs in module-sized batches, using real Probe results to correct request contracts, params, response models, tests, and future bpi-go references.

**Architecture:** The migration unit is a module or cohesive submodule batch, not a single endpoint. A batch contains several related endpoints, runs real probes for applicable anonymous/normal/vip account states, then updates contracts and Rust code from observed behavior. Probe tooling may generate request drafts, but real Probe output is the authority.

**Tech Stack:** Rust 2024, reqwest, serde, thiserror, tokio, tracing, existing `src/probe`, `tests/contracts`, local `account.toml`, and local `target` artifacts.

---

## Non-Negotiable Constraints

- Do not invent JSON, response fields, examples, or docs.
- Do not treat hand-written JSON as truth.
- Do not treat generated request drafts as truth.
- Real Probe results are the primary evidence for API behavior.
- Raw Probe outputs must stay local under `target/bpi-probe-runs/...`.
- Do not store raw Probe outputs in `flightdeck/work`.
- Do not commit raw Probe outputs, cookies, `SESSDATA`, `bili_jct`, `buvid`, account-specific response headers, or account-identifying data when avoidable.
- Confirmed request contracts may be committed under `tests/contracts/...`.
- Sanitized endpoint fixtures may be committed under `tests/contracts/<domain>/<endpoint>/responses/...` only after review.
- Default commit unit is one module batch, not one endpoint.
- Do not create hundreds of endpoint-sized commits.
- Do not make a single commit so large that review or rollback becomes impractical.

## Batch Granularity

`src` has hundreds of Rust files. Endpoint-by-endpoint migration would pollute git history, while whole-repo migration is too large to verify. Use module batches.

Default batch sizes:

- Small top-level module: one batch.
  Examples: `wallet`, `clientinfo`, `web_widget`, `activity`, `opus`.
- Medium cohesive module: one batch when most endpoints share auth/risk level.
  Examples: `danmaku`, `vip`, `video_ranking`, `message`, `search`, `bangumi`, `cheese`, `historytoview`, `fav`, `note`.
- Large module: split by submodule or responsibility.
  Examples: `video/info`, `video/collection`, `login/login_info`, `login/member_center`, `user/relation`, `dynamic/detail-readonly`, `creativecenter/statistics`.
- High-risk or mutating module: split smaller and require explicit gating.
  Examples: `live`, `dynamic/publish`, `comment/action`, `fav/action`, `user/relation/action`, `creativecenter/upload`.

A normal batch should contain roughly 3-15 endpoints. A one-endpoint batch is allowed only when the endpoint is special, risky, blocked, or cannot share contracts/tests with others.

## Batch Failure Strategy

Do not let one bad endpoint block a whole module batch.

If a batch has partial failures:

- Successful endpoints may continue in the batch.
- Failed endpoints must be listed in the batch notes with reason.
- Failed endpoints are moved to a follow-up batch if they are deprecated, grey-released, region-limited, require special headers, require special account state, or need mutating permissions.
- A batch can be split after Probe if the actual behavior is more diverse than expected.
- Do not promote contracts for endpoint/profile pairs that were not successfully probed unless the contract explicitly documents why probing was impossible.

## Account Profiles

The supported Probe account states are:

- `anonymous`: no account/cookie.
- `normal`: logged-in non-VIP account.
- `vip`: logged-in VIP account.

`account.toml` may provide profiles in structured form:

```toml
[probe.vip]
bili_jct = "..."
dede_user_id = 123
dede_user_id_ckmd5 = "..."
sessdata = "..."
buvid3 = "..."

[probe.normal]
bili_jct = "..."
dede_user_id = 456
dede_user_id_ckmd5 = "..."
sessdata = "..."
buvid3 = "..."
```

Flat semantic suffixes are allowed:

```toml
bili_jct_vip = "..."
dede_user_id_vip = 123
sessdata_vip = "..."
buvid3_vip = "..."

bili_jct_normal = "..."
dede_user_id_normal = 456
sessdata_normal = "..."
buvid3_normal = "..."
```

Arbitrary names such as `account2`, `*_2`, or numbered fields must not be treated as profiles.

## Artifact Locations

Local only:

```text
target/bpi-contract-drafts/<domain>/<batch>/
target/bpi-probe-runs/<domain>/<batch>/<endpoint>/<profile>.response.json
target/bpi-probe-notes/<domain>/<batch>.md
```

Commit only after review:

```text
tests/contracts/<domain>/<endpoint>/contract.json
tests/contracts/<domain>/<endpoint>/responses/<case>.json
```

`flightdeck/work` is for plans, protocols, and decisions. It is not a raw Probe output directory.

## Request Draft Policy

Request drafts are useful, but they are not evidence.

Allowed ways to create drafts:

- Script-assisted from existing Rust endpoint code.
- Script-assisted from a module inventory table.
- Manual draft for a small batch when generator support would be more expensive than the batch itself.

Rules:

- Drafts must be marked as drafts and stored under `target/bpi-contract-drafts/...`.
- A draft becomes a committed contract only after Probe succeeds and the result is reviewed.
- If the draft is generated from wrong Rust code, Probe should expose the failure. Fix the Rust request construction or draft source, then rerun Probe.
- If a generator cannot parse a module, do not get stuck endlessly improving the generator. Use manual drafts for that batch and record the reason.
- Generator improvements should be their own batch only when they help multiple modules.

## Probe Flow Policy

Some endpoints require values derived from an earlier API response, such as one-time tokens, generated IDs, or QR login keys. Use Probe flows for those cases instead of committing a stale literal value.

Rules:

- Flow drafts live under `target/bpi-contract-drafts/...` unless the flow contract has been reviewed and contains no account-specific or one-time values.
- Flow execution output lives under `target/bpi-probe-runs/...`.
- Use JSON Pointer extraction from prior Probe results and `${name}` substitution in later step contracts.
- Do not promote a dynamic step as a standalone request contract if its request depends on a one-time value.
- A dynamic flow contract may be committed only when reviewed placeholders, extraction paths, and expected API behavior are stable and do not expose secrets.

## What To Compare

Avoid useless self-validation. Comparing a draft to the request sent by that same draft is not enough.

For each endpoint, compare these things:

- Current Rust endpoint request construction vs candidate request contract.
- Probe captured request vs candidate request contract.
- Probe response body vs Rust response model.
- Anonymous response vs authenticated response.
- Normal response vs VIP response.
- Observed API errors vs current error handling.

Useful mismatches:

- Rust code sends missing, extra, or wrongly named query/body fields.
- Rust code serializes default values differently from the real API expectation.
- Anonymous/normal/vip return different shapes or error codes.
- Response model has fields with wrong type, wrong optionality, or missing aliases.
- Current tests use random parameters that do not match observed request requirements.

## Per-Module Batch Execution Steps

### Task 1: Select Batch

- [ ] Pick one module batch using the batch granularity rules.
- [ ] List endpoints in the batch.
- [ ] Classify each endpoint as public read-only, authenticated read-only, mutating, or high-risk live.
- [ ] Exclude mutating endpoints unless the batch explicitly enables `BPI_MUTATING_TEST=1`.
- [ ] Decide which profiles apply: anonymous, normal, vip.

### Task 2: Prepare Request Drafts

- [ ] Generate or write request drafts for applicable endpoint/profile pairs.
- [ ] Store drafts under `target/bpi-contract-drafts/<domain>/<batch>/`.
- [ ] Include method, URL, query/body, required headers, auth profile, auth requirements, and expected API code if already known.
- [ ] Do not commit drafts.

### Task 3: Execute Probe

Run Probe from drafts or candidate contracts, writing raw output under `target`:

```powershell
$env:BPI_PROBE = "1"
cargo run --quiet --bin bpi-probe -- <draft-or-contract.json> account.toml target\bpi-probe-runs\<domain>\<batch>\<endpoint>\<profile>.response.json
```

- [ ] Run anonymous where safe.
- [ ] Run normal where safe.
- [ ] Run vip where safe.
- [ ] Record blocked endpoint/profile pairs in `target/bpi-probe-notes/<domain>/<batch>.md`.

### Task 4: Review Probe Results

- [ ] Compare request construction, captured request, response body, and Rust model.
- [ ] Decide whether each mismatch belongs to Rust code, draft, model, or test data.
- [ ] Split failed or special endpoints out of the batch when needed.
- [ ] Decide which contracts can be promoted.

### Task 5: Promote Contracts And Fixtures

- [ ] Promote one reviewed endpoint contract into `tests/contracts/<domain>/<endpoint>/contract.json`.
- [ ] Put reviewed/sanitized response fixtures under `tests/contracts/<domain>/<endpoint>/responses/*.json`.
- [ ] Keep profile differences in `contract.json` `cases`, not in duplicated per-profile request files.
- [ ] Keep fixtures minimal and remove account-specific data.
- [ ] Do not commit raw Probe output.

### Task 6: Upgrade Rust Code

- [ ] Update typed params to match promoted contracts.
- [ ] Update response models to match observed response bodies.
- [ ] Preserve observed API error behavior.
- [ ] Avoid broad `serde_json::Value` unless the observed field is genuinely variable.
- [ ] Keep shared model edits scoped. If a shared model change affects other modules, either prove the impact with tests or split it into a separate shared-model batch.

### Task 7: Test

- [ ] Add contract parser tests.
- [ ] Add parameter serialization tests.
- [ ] Add response model parsing tests from sanitized fixtures or local Probe outputs when present.
- [ ] Add error behavior tests for observed API errors.
- [ ] Run focused checks:

```powershell
cargo test --all-features --lib probe
cargo test --all-features --lib <domain>
```

### Task 8: Verify And Commit Batch

- [ ] Run full verification:

```powershell
cargo fmt --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo check --all-features
cargo test --all-features --lib
```

- [ ] Confirm raw artifacts are not tracked:

```powershell
git status --short
git status --short --ignored=matching target\bpi-contract-drafts
git status --short --ignored=matching target\bpi-probe-runs
git status --short --ignored=matching target\bpi-probe-notes
```

- [ ] Commit one module batch:

```powershell
git add src tests/contracts
git commit -m "feat(<domain>): validate <batch> api contracts"
```

## Done Definition For One Module Batch

A batch is done only when:

- Applicable endpoints have reviewed contracts or documented exceptions.
- Applicable profiles were probed or documented as skipped.
- Raw artifacts remain under `target`.
- Promoted contracts match observed request behavior.
- Rust params match promoted contracts.
- Rust models parse observed successful responses.
- Observed API errors are tested.
- Failed endpoints are split out with notes.
- Sensitive values are not committed.
- `task check_all` passes.

## Commit Policy

- Prefer one commit per completed module batch.
- Do not commit one endpoint at a time by default.
- Do not combine unrelated modules in one commit.
- Do not bundle generator/tooling changes with a large endpoint migration unless the tooling change is tiny and only supports that batch.
- If generator/tooling work is substantial, commit it separately before the module batch.
- If a module batch becomes too large to review, split it by submodule before committing.

## Current Pilot Batch

`clientinfo/ip` is the first reviewed pilot batch for this protocol.

It demonstrates:

- anonymous/normal/vip profile probing for a public read-only endpoint
- promoted request contracts under `tests/contracts/clientinfo/ip/`
- local raw Probe output kept under `target/bpi-probe-runs/...`
- request-contract validation against endpoint construction
- response model validation from local Probe output when present

Future migrations should apply the same evidence rules at module-batch scale.
