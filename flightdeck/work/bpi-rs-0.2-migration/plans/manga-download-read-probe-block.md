# Manga Download Read Probe Block

## Scope

`manga/download-read` covers the private helper endpoints in `src/manga/download.rs`:

- `manga_image_index` -> `POST https://manga.bilibili.com/twirp/comic.v1.Comic/GetImageIndex`
- `manga_image_token` -> `POST https://manga.bilibili.com/twirp/comic.v1.Comic/ImageToken`

`manga_image_token` depends on image paths returned by `GetImageIndex`, so `GetImageIndex`
must succeed first.

## Probe Attempts

Local raw Probe outputs stay under `target/bpi-probe-runs/manga/download-read/...`.

Attempts made so far:

- Existing trials for `ep_id = 482133` with anonymous, normal, and vip profiles.
- Existing web-style trials for `ep_id = 321912` with `device=pc&platform=web`, manga
  referer, origin, and browser-like headers.
- Existing anonymous `ComicDetail` discovery trials for `comic_id = 24442`.
- Current normal/vip `ComicDetail` discovery trials for `comic_id = 24442`, using
  `device=pc&platform=web`, cookie, manga referer, origin, and browser-like headers.
- Current normal/vip `GetImageIndex` trials using camelCase `epId = 321912`.
- Current normal/vip `GetImageIndex` trials using another public README sample
  `ep_id = 334263` with `mc26742/334263` referer.

All current and prior `GetImageIndex` / `ComicDetail` download-read probes returned HTTP
200 with API `code = 99`. Authenticated profiles returned the generic business-exception
message. No promoted success contract should be created from these attempts.

## Reference Check

External references still describe the same PC request shape already tested here:

- `GetImageIndex?device=pc&platform=web`
- Cookie-backed request
- JSON body containing `ep_id`
- manga chapter referer and browser-style headers

Those references support the request-shape trial, but local Probe remains authoritative for
this workspace and currently contradicts promotion.

## Decision

Keep `manga/download-read` Probe-blocked until one of these is available:

- a current, account-readable manga chapter id known to succeed for normal or vip;
- a current valid `ComicDetail` flow that returns an `ep_list` under the configured Probe
  profiles;
- updated Probe support for any current manga anti-abuse or crypto handshake required by
  the web reader.

Do not promote `GetImageIndex` or `ImageToken` contracts from `code = 99` responses, because
the current evidence does not prove the stable request/response contract for a readable
chapter.
