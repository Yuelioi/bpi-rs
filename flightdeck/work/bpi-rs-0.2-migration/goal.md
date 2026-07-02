# bpi-rs 0.2 API Migration Goal

## Objective

Upgrade bpi-rs APIs in high-quality, maintainable module batches.

The migration must use real Probe results to validate request contracts, typed params, response models, error behavior, tests, and future bpi-go reference material.

## Required Protocol

Before doing implementation work, read and follow:

```text
flightdeck/work/bpi-rs-0.2-migration/api-upgrade-protocol.md
```

That protocol is the source of truth for:

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

## Execution Guardrails

- Do not migrate endpoint-by-endpoint by default.
- Do not create hundreds of tiny commits.
- Do not write final contracts as guessed JSON.
- Do not store raw Probe output in `flightdeck/work`.
- Do not commit raw Probe output or account-sensitive data.
- Do not proceed with a module batch until the batch scope, Probe plan, and split/failure strategy are clear.
- If protocol and current instinct conflict, follow the protocol or stop and ask.

