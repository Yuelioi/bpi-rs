# Cockpit — bpi-rs

Focus: `work/bpi-rs-0.2-migration/` — staged 0.2 redesign for a high-quality Rust SDK.

## In flight

- `work/bpi-rs-0.2-migration/` — staged 0.2 migration is active; goal-mode continuation now defaults to Probe-backed endpoint contract batches unless an explicit non-Probe bridge batch is selected.

## Next

- Choose the next incomplete module or cohesive submodule from `flightdeck/work/bpi-rs-0.2-migration/migration-status.md`. For normal endpoint migration, follow `api-upgrade-protocol.md`: draft under `target`, run Probe, promote reviewed `contract.json` plus sanitized fixtures, then update Rust/tests. A remaining endpoint-candidate audit found no new safe read batch; `manga/download-read` is Probe-blocked by repeated API `code = 99` results. Before continuing, identify a new safe read endpoint, a valid manga download flow/chapter/handshake, an explicitly enabled gated/mutating batch, or explicitly record non-Probe bridge work. Completed bridge examples now include `clientinfo/module-client-bridge`, `web_widget/module-client-bridge`, `activity/module-client-bridge`, `audio/module-client-bridge`, `article/module-client-bridge`, `fav/module-client-bridge`, `cheese/module-client-bridge`, `bangumi/module-client-bridge`, `video_ranking/module-client-bridge`, `search/module-client-bridge`, `wallet/module-client-bridge`, `opus/module-client-bridge`, `misc/module-client-bridge`, `message/module-client-bridge`, `manga/module-client-bridge`, `electric/module-client-bridge`, `creativecenter/module-client-bridge`, `dynamic/module-client-bridge`, `vip/module-client-bridge`, `comment/module-client-bridge`, `historytoview/module-client-bridge`, `note/module-client-bridge`, `danmaku/module-client-bridge`, `live/public-core-client-bridge`, `login/safe-flow-client-bridge`, `user/module-client-bridge`, `video/module-client-bridge`, and `video/collection-player-client-bridge`; do not repeat them. Use `plans/batch-todolist.md` for the larger-batch execution cadence: record batch first, implement the whole batch, verify once, then commit once.

## Open questions
