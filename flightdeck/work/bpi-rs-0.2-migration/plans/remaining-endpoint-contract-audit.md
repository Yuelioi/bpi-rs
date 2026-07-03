# Remaining Endpoint Contract Audit

## Scope

This audit checks whether another safe Probe-backed endpoint contract batch remains after
the completed read batches and the `manga/download-read` Probe block.

This is not a non-Probe bridge batch. No shared-core or domain-client bridge work was
selected here.

## Evidence Reviewed

- `flightdeck/work/bpi-rs-0.2-migration/goal.md`
- `flightdeck/work/bpi-rs-0.2-migration/api-upgrade-protocol.md`
- `flightdeck/work/bpi-rs-0.2-migration/migration-status.md`
- `tests/contracts/**/contract.json`
- `rg` inventory of `pub async fn` methods under `src`
- `rg` inventory of request execution calls such as `send_bpi`, `send_bpi_payload`,
  `.get(...)`, and `.post(...)`
- `src/manga/download.rs`
- `plans/manga-download-read-probe-block.md`

## Findings

Safe read batches with promoted endpoint contracts are already covered across the current
implemented modules, including the batches that were easy to mistake as incomplete from
top-level status rows:

- `fav/read` is complete despite the stale pinned-row `Planned` label.
- `login/notice/login-log` and `login/notice/login-notice` both have promoted contracts.
- Audio extra-read endpoints have promoted contracts for stream URL, collection status,
  coin count, music-list/menu, and rank reads.
- `creativecenter/season`, `creativecenter/videos`, `creativecenter/statistics`, and
  `creativecenter/railgun-read` have promoted contracts for their read endpoints.
- `video/info-read`, `video/playurl`, `video/collection-read`, and `video/player-read`
  are already complete and must not be repeated.

The remaining endpoint work falls into these buckets:

- **Probe-blocked read:** `manga/download-read`, specifically private helper endpoints in
  `src/manga/download.rs` (`GetImageIndex` and `ImageToken`). Repeated real Probe attempts
  returned HTTP 200/API `code = 99`, so no contract should be promoted yet.
- **Mutating or account-state-changing:** article actions, audio action/subscribe,
  bangumi follow, comment actions, creativecenter season/edit/upload/opus mutations,
  danmaku send/recall/thumb/report/edit/buy, dynamic publish/actions, electric B coin and
  message/reply actions, fav actions, history/to-view mutations, live mutations and
  moderation writes, manga buy/clock-in/share/exchange, message send, note add/delete,
  user relation/group actions, video actions/report/collection actions, and VIP
  action/clockin.
- **Flow-sensitive auth/session:** login SMS/password/cookie refresh/logout/sign update.
- **Deprecated or documented exceptions:** dynamic legacy read endpoints that returned
  HTTP 404 HTML and were recorded as deprecated exceptions.
- **Wrappers or compatibility methods:** module-client and legacy compatibility methods
  reuse already promoted contracts rather than defining new endpoint behavior.
- **Static/local helpers:** ID enums, signing helpers, transport internals, and other
  functions that are not remote endpoint contracts.

## Decision

Do not start a guessed contract batch just to keep endpoint churn moving. The next normal
endpoint continuation still needs either:

- a newly identified safe read endpoint not covered above;
- a new valid `manga/download-read` chapter/flow/handshake that moves `code = 99` to a
  successful Probe result; or
- explicit user selection of a gated mutating/flow-sensitive batch with the required
  safety controls.

If none of those exists, the next useful migration work is shared-core/domain-client
foundation or bridge work, but that must be explicitly recorded as a non-Probe batch before
starting.
