# Module Client Coverage Audit

## Batch

`shared-core/module-client-coverage-audit`

## Type

Non-Probe docs/code audit. No Rust implementation, Probe run, or contract promotion is expected unless the audit discovers a small cohesive missing bridge batch.

## Purpose

Check whether promoted contracts under `tests/contracts/**/contract.json` are reachable through module-client code after the latest `video` and `live` bridge batches.

This audit exists because the migration has shifted from endpoint contract promotion into module-client surface cleanup. It should prevent repeating completed bridge work while still making any remaining promoted-contract gap explicit.

## Evidence Commands

Promoted contract names not present in `src/**/client.rs`:

```powershell
$contracts = Get-ChildItem -Recurse tests\contracts -Filter contract.json | ForEach-Object {
    $json = Get-Content -Raw $_.FullName | ConvertFrom-Json
    [pscustomobject]@{
        Path = Resolve-Path -Relative $_.FullName
        Name = [string]$json.name
        Domain = ($_.FullName -split '\\tests\\contracts\\')[1].Split('\\')[0]
    }
}
$clientText = (Get-ChildItem -Recurse src -Filter client.rs | ForEach-Object {
    Get-Content -Raw $_.FullName
}) -join "`n"
$contracts |
    Where-Object { $_.Name -and ($clientText -notmatch [regex]::Escape($_.Name)) } |
    Sort-Object Domain,Path |
    Format-Table -AutoSize
```

Observed output:

```text
Path                                          Name          Domain
----                                          ----          ------
.\tests\contracts\login\qr\flow\contract.json login.qr.flow login
```

Promoted contract count by domain:

```powershell
$contracts = Get-ChildItem -Recurse tests\contracts -Filter contract.json | ForEach-Object {
    $rel = (Resolve-Path -Relative $_.FullName).TrimStart('.','\')
    $parts = $rel -split '\\'
    $json = Get-Content -Raw $_.FullName | ConvertFrom-Json
    [pscustomobject]@{
        Domain = $parts[2]
        Path = $rel
        Name = [string]$json.name
    }
}
"Total contracts: $($contracts.Count)"
$contracts | Group-Object Domain | Sort-Object Name | Select-Object Name,Count | Format-Table -AutoSize
```

Observed output:

```text
Total contracts: 206

Name           Count
----           -----
activity           2
article            4
audio             16
bangumi            6
cheese             4
clientinfo         1
comment            4
creativecenter    14
danmaku           12
dynamic           13
electric          10
fav                6
historytoview      3
live              20
login             13
manga              5
message            3
misc               4
note               7
opus               1
search            11
user              16
video             17
video_ranking      9
vip                1
wallet             1
web_widget         3
```

Module-client method and payload/raw call inventory:

```powershell
$clientMethods = Get-ChildItem -Recurse src -Filter client.rs | ForEach-Object {
    $rel = Resolve-Path -Relative $_.FullName
    $text = Get-Content -Raw $_.FullName
    $methodCount = ([regex]::Matches($text, '(?m)^\s*pub\s+(?:async\s+)?fn\s+')).Count
    $payloadCount = ([regex]::Matches($text, 'send_bpi_(?:optional_)?payload|send_raw_bytes|send_raw_bpi_request')).Count
    [pscustomobject]@{
        Client = $rel
        Methods = $methodCount
        PayloadOrRawCalls = $payloadCount
    }
}
$clientMethods | Sort-Object Client | Format-Table -AutoSize
```

Observed highlights:

```text
.\src\live\client.rs          20  20
.\src\video\client.rs         17  17
.\src\user\client.rs          16  16
.\src\creativecenter\client.rs 14  14
.\src\dynamic\client.rs       13  13
.\src\search\client.rs        11   3
.\src\danmaku\client.rs       12   4
.\src\login\client.rs         12  11
.\src\misc\client.rs           5   4
```

The lower helper counts for `search`, `danmaku`, `login`, and `misc` are expected:

- `search` typed-search methods share dynamic endpoint labels and WBI request construction.
- `danmaku` includes raw bytes/protobuf/XML paths, so payload-envelope helper counts are not one-to-one with methods.
- `login.qr_poll` needs cookie extraction from the raw `reqwest::Response` before envelope decoding.
- `misc.bili_ticket_string` is a convenience wrapper around `bili_ticket`.

## Exception Review

`tests/contracts/login/qr/flow/contract.json` is intentionally not a module-client method contract.

It is a Probe flow contract:

- step `generate` calls `login.qr_generate.anonymous`
- step `poll` calls `login.qr_poll.anonymous`
- `qrcode_key` is extracted from the generated QR response and substituted into poll

The module client already exposes the stable endpoint operations:

```text
LoginClient::qr_generate() -> login.qr_generate
LoginClient::qr_poll(LoginQrPollParams) -> login.qr_poll
```

`rg "login\.qr_(generate|poll)|qr_generate|qr_poll|qrcode_key" src\login tests\contracts\login\qr -n` confirms the endpoint contracts, params, client methods, and tests are present. A higher-level `qr_flow()` method would need external user scanning/poll lifecycle policy and would not be a simple endpoint bridge.

## Conclusion

No new safe non-Probe module-client bridge batch was found among promoted contracts.

The only unmatched promoted contract name is the reviewed `login.qr.flow` Probe flow contract, which is already covered by `qr_generate` and `qr_poll` endpoint methods plus Probe flow tests. Treat it as a documented exception, not a missing bridge.

Normal continuation should return to:

- a newly identified Probe-backed safe endpoint batch,
- a valid current `manga/download-read` flow/chapter/handshake,
- or an explicitly enabled gated/mutating batch.

Do not repeat completed bridge batches.
