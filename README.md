# wrnotifier

A small program that watches for CTGP World Records and posts them in Discord channels through webhooks.

### Usage

1. Clone the repository.
2. [Install Rust](https://www.rust-lang.org/tools/install)
3. Create a `webhooks.json` file in the root directory with the following structure:

```json
[
  {
    "url": "YOUR_DISCORD_WEBHOOK_URL",
    "200cc": true,
    "150cc": true
  }
]
```

4. Run the program with `cargo run -r`. Optionally you can enable more logging with `RUST_LOG=debug cargo run -r`.

Every minute it will send a request to the CTGP Ghost DB and check for new world records. If it finds any, it will post them to the specified Discord webhooks.
