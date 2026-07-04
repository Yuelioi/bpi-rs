# Remaining Flat API Boundary Audit

## Batch

`shared-core/remaining-flat-api-boundary-audit`

## Type

Non-Probe release cleanup audit batch.

## Why This Exists

The staged flat read cleanup batches removed the low-risk compatibility shims
whose module-client replacements were already backed by promoted contracts.
This audit updates the remaining boundary after
`flat-api/remove-login-safe-read-flow-shims` so future work does not keep
guessing at function-sized removals.

This is not a Probe batch and does not change Rust behavior.

## Evidence Commands

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
        [pscustomobject]@{
          Domain = $parts[1]
          File = $file
          Line = $i + 1
          Method = $matches[1]
        }
      }
      $depth += ([regex]::Matches($line, '\{')).Count
      $depth -= ([regex]::Matches($line, '\}')).Count
      if ($depth -le 0) { $inImpl = $false }
    }
  }
}
$rows.Count
$rows | Group-Object Domain | Sort-Object Name
$rows | Sort-Object File,Line
```

Observed result:

```text
Total remaining direct flat async methods: 112
```

## Current Surface By Module

| Module | Remaining direct flat async methods |
| --- | ---: |
| article | 4 |
| audio | 4 |
| bangumi | 2 |
| comment | 6 |
| creativecenter | 11 |
| danmaku | 8 |
| dynamic | 10 |
| electric | 3 |
| fav | 7 |
| historytoview | 6 |
| live | 12 |
| login | 4 |
| manga | 8 |
| message | 1 |
| note | 3 |
| user | 9 |
| utils | 2 |
| video | 9 |
| vip | 3 |
| **Total** | **112** |

## Boundary Classification

The remaining flat methods are not unhandled safe read shims. They fall into
these boundary classes:

| Boundary | Remaining examples |
| --- | --- |
| gated/mutating/session | article like/coin/favorite; audio favorite/coin/subscribe; bangumi follow/unfollow; comment add/like/dislike/delete/top/report; creativecenter opus/season/edit/upload; danmaku send/recall/buy/thumbup/report/edit; dynamic publish/action; electric B coin/message/reply; fav folder/resource writes; history/to-view delete/add/clear/shadow-set; live send/manage/moderation writes; login SMS/logout/sign update; manga buy/clock-in/share/exchange; message send; note add/delete; user relation/group/space notice writes; video like/coin/favorite/report/collection writes; VIP action/clockin |
| deprecated/documented exception | `dynamic_card_detail`, `dynamic_repost_detail`, and `dynamic_spec_item_likes`, which are already marked deprecated after observed HTTP 404 text/html responses from `vc.bilibili.com` |
| Probe-blocked read | `manga/download-read` helper endpoints remain blocked by repeated HTTP 200/API `code = 99` Probe results; do not promote contracts without a current valid chapter/flow/handshake |
| helper/API decision | `get_wbi_sign` and `get_wbi_sign2` are signing helpers rather than endpoint methods; they need an explicit public/internal API decision |

## Decision

Do not continue removing flat methods by name unless a new reviewable boundary
batch is selected first. The next normal continuation must be one of:

- an explicitly enabled mutating/write module-client batch with safety controls;
- a successful new `manga/download-read` Probe path;
- a deprecated legacy removal decision for the dynamic 404 methods;
- a helper API boundary decision for WBI signing helpers;
- or a release/API policy batch that declares which gated legacy flat methods
  remain compatibility-only for 0.2.

No Probe run, contract promotion, Rust source change, or `flightdeck/cockpit.md`
update is expected for this audit.

## Verification

```powershell
rg -n "Total remaining direct flat async methods: 112|gated/mutating/session|deprecated/documented exception|Probe-blocked read|helper" flightdeck\work\bpi-rs-0.2-migration\plans\remaining-flat-api-boundary-audit.md
git diff --check
git diff -- flightdeck/cockpit.md
git status --short --ignored=matching flightdeck\work\bpi-rs-0.2-migration\migration-status.md target\bpi-contract-drafts target\bpi-probe-runs target\bpi-probe-notes
```

Cargo gates are not required because this batch changes only Flightdeck planning
documents and does not change Rust source, tests, contracts, or compiled docs.
