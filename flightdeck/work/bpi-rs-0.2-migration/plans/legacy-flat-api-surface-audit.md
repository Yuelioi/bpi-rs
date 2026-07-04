# Legacy Flat API Surface Audit

## Batch

`shared-core/legacy-flat-api-surface-audit`

## Type

Non-Probe release cleanup audit batch.

## Why This Exists

The 0.2 migration now documents module clients as the primary API, and the
module-client coverage audit found no missing safe bridge over promoted
contracts. Stage 6 still has a separate release-cleanup requirement: remove or
consciously retain the old flat `BpiClient` endpoint surface.

This audit records the remaining surface before any removal work. It is not a
Probe batch and it does not change Rust behavior.

## Reproduction Commands

Count public async methods declared directly inside `impl BpiClient` blocks:

```powershell
$files = rg --files src | Where-Object { $_ -like '*.rs' }
$rows = foreach ($file in $files) {
  $lines = Get-Content $file
  $inImpl = $false
  $depth = 0
  for ($i = 0; $i -lt $lines.Count; $i++) {
    $line = $lines[$i]
    if (-not $inImpl -and $line -match '^impl BpiClient\s*\{') {
      $inImpl = $true
      $depth = 0
    }
    if ($inImpl) {
      if ($line -match '^\s*pub\s+async\s+fn\s+([A-Za-z0-9_]+)') {
        $parts = $file -split '[\\/]'
        [pscustomobject]@{ Domain = $parts[1]; File = $file; Line = $i + 1; Method = $matches[1] }
      }
      $depth += ([regex]::Matches($line, '\{')).Count
      $depth -= ([regex]::Matches($line, '\}')).Count
      if ($depth -le 0) { $inImpl = $false }
    }
  }
}
$rows.Count
```

Observed result:

```text
328
```

Aggregate by top-level source module:

```powershell
$rows | Group-Object Domain | Sort-Object Name |
  ForEach-Object { "{0}`t{1}" -f $_.Count, $_.Name }
```

## Current Surface By Module

| Module | Flat public async methods |
| --- | ---: |
| activity | 3 |
| article | 8 |
| audio | 20 |
| bangumi | 11 |
| cheese | 5 |
| clientinfo | 1 |
| comment | 10 |
| creativecenter | 25 |
| danmaku | 20 |
| dynamic | 23 |
| electric | 13 |
| fav | 13 |
| historytoview | 9 |
| live | 32 |
| login | 19 |
| manga | 13 |
| message | 4 |
| misc | 5 |
| note | 10 |
| opus | 1 |
| search | 11 |
| user | 27 |
| utils | 2 |
| video | 26 |
| video_ranking | 9 |
| vip | 4 |
| wallet | 1 |
| web_widget | 3 |
| **Total** | **328** |

## Risk Classification

A conservative filename/name heuristic splits the remaining methods into:

```text
read_or_helper: 227
gated_or_mutating: 101
```

The heuristic intentionally over-flags some files that mix reads and writes.
Examples:

- `live/manage.rs` includes `live_version`, which has module-client read coverage
  but lives beside room management mutations.
- `electric/charge_msg.rs` includes private read methods such as
  `electric_remark_list` and `electric_remark_detail` beside message/reply
  mutations.
- `audio/action.rs` includes read-like status/count methods beside actual
  action methods.
- `manga/clockin.rs` includes `manga_clock_in_info` beside clock-in mutations.

Treat this split as a review queue, not an automated deletion list.

## Interpretation

The 328 methods are not evidence that endpoint contract migration is incomplete.
They are mostly compatibility shims and legacy flat entry points. The previously
recorded `shared-core/module-client-coverage-audit` found 206 promoted
contracts and no missing safe module-client bridge, with only `login.qr.flow`
remaining as a Probe-flow exception over existing QR endpoint methods.

The remaining flat surface is therefore a release-cleanup/API-shape problem:

- Reads already covered by module clients can be deprecated or removed in
  reviewable compatibility batches.
- Mutating, gated, or flow-sensitive methods should not be removed blindly
  because many of them still have no promoted safe contract or no replacement
  module-client write surface.
- Helper methods such as `get_wbi_sign` and `get_wbi_sign2` need an explicit
  public/internal API decision rather than being treated as endpoint methods.

## Recommended Follow-Up Batches

1. `flat-api/deprecate-safe-read-shims`

   Add deprecation attributes and migration notes for low-risk flat read methods
   whose module-client replacements are already verified. This creates a softer
   compatibility boundary before physical removal and avoids breaking all legacy
   tests at once.

2. `flat-api/remove-small-read-shims`

   Remove the smallest modules first after deprecation: `clientinfo`, `wallet`,
   `opus`, `web_widget`, `activity`, and `message`. Rewrite affected legacy
   tests to module-client calls in the same batch.

3. `flat-api/remove-medium-read-shims`

   Remove medium safe read surfaces in module batches: `bangumi`, `cheese`,
   `search`, `video_ranking`, `fav`, `note`, `historytoview`, `comment`, and
   `misc`.

4. `flat-api/remove-large-read-shims`

   Split high-volume modules by responsibility: `video`, `live`, `dynamic`,
   `user`, `creativecenter`, `audio`, `danmaku`, and `electric`.

5. `flat-api/gated-mutating-decision`

   For mutating, account-changing, upload, login/session, and flow-sensitive
   methods, decide whether to:

   - keep them as explicitly legacy-gated compatibility APIs,
   - add replacement module-client write APIs behind `BPI_MUTATING_TEST=1`, or
   - remove them as out of scope for 0.2.

## Verification

This audit is docs-only:

- no Rust source changed,
- no tests or contracts changed,
- no Probe run was expected,
- no raw Probe output was created,
- `flightdeck/cockpit.md` should remain unchanged.

