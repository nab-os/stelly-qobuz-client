# stelly-qobuz-client

> **Archived.** Superseded by [stelly-provider](https://github.com/nab-os/stelly-provider),
> the protocol every Stelly music provider answers, Qobuz's gateway included
> since stelly-qobuz 0.2.0. Stelly 1.2 and later use that instead.

The other side of stelly-qobuz, the gateway that holds a Qobuz account and
its one request queue for Stelly, stelly-mix and anything else on the
machine: its wire types, and a client of it.

`api` names every call the gateway answers once; the `async` feature sends
them through reqwest, `blocking` through ureq, and with neither it is the
wire types alone.

```toml
stelly-qobuz-client = { git = "https://github.com/nab-os/stelly-qobuz-client", tag = "v0.1.0", features = ["async"] }
```

```rust
let gateway = stelly_qobuz_client::Client::new(Config::from_env(None));
let album = gateway.send(api::album("0060254728690")).await?;
let file = gateway.at(Priority::Bulk).send(api::file_url("12345", 5)).await?;
```

Clients read `STELLY_QOBUZ_URL` (default `http://127.0.0.1:7710`) and
`STELLY_QOBUZ_TOKEN`, from the environment or a .env they name.

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
