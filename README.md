# Service Kit

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Service kit is rust template for web services that sets up a lot of common boilerplate out of the gate.

- A service that is easy to install and manage on MacOS, Linux, and Windows hosts.
- A distributable client/server architecture so you can have your service installed somewhere, and your clients somewhere else.
- A WASM-capable client that you can embed in Javascript or Typescript applications.
- A web dashboard embedded in the program or deployable separately.
- A client CLI which lets you quickly work with the API in programmatic environments.
- A component-based terminal UI which lets you quickly work with the API in console environments.
- A prebuilt release and documentation hub setup.
- Configuration, telemetry, and lightweight database storage support out of the box.

...And more.

## Reasoning

Whenever kicking off a new project, there's some things I always tend to put together.

- CLI.
- Configuration.
- Telemetry / logging.
- Lightweight storage.
- A "service architecture": client / api / service in one crate.

This led to the need for boilerplate, something that can be put into template form for [cargo-generate][] later.

## Preconfigured tools

(This is a checklist / todo list until I've fully set these all up.)

- [ ] Release and documentation page managed by [cargo-dist][] and [oranda][].
- [x] CLI with [clap.rs][], [dialoguer][], [console][] and [indicatif][], for robust CLI interactors and programmatic access needs.
- [x] A component-driven non-blocking terminal UI made with [ratatui][] and [crossterm][] for anyone who wants to be a little extra about their console work (me; I always do).
- [x] A web service built with [axum][] and a web client made with [reqwest][] to interact with it.
- [x] A wasm-capable build for the client, so you can pop it into whatever host system you like.
- [ ] A single page application web dashboard built with [vite][], embedded in the web service but deployable independently, which uses the web client.
- [x] Premade telemetry with [tracing][] setup with configurable logging on by default.
- [x] A protocol buffer implementation with [prost][] that can be leveraged in any of the clients.
- [x] Installable out of the box with [service-manager][].
- [x] Lightweight storage with [sqlx][].
- [ ] A durable memory cache and lightweight message queue with [rusqlite][].

## License

This project is MIT licensed.

As everything in here is configuration boilerplate, tying together a bunch of differently licensed open source tools, you will need to independently examine and make sure you support those licenses as well. Once you establish a project based on this boilerplate, you inherit those licenses.

## Code of Conduct

This project has a code of conduct for all contributors and contributions. There's no commitment here to support non-productive, collaborative behavior. Instead of engaging with bad faith behavior, any offender will be removed.

[sqlx]: https://github.com/launchbadge/sqlx
[cargo-dist]: https://github.com/axodotdev/cargo-dist
[oranda]: https://github.com/axodotdev/oranda
[clap.rs]: https://github.com/clap-rs/clap
[dialoguer]: https://github.com/console-rs/dialoguer
[indicatif]: https://github.com/console-rs/indicatif
[console]: https://github.com/console-rs/console
[ratatui]: https://github.com/ratatui-org/ratatui
[cargo-generate]: https://github.com/cargo-generate/cargo-generate
[axum]: https://github.com/tokio-rs/axum
[vite]: https://vitejs.dev/
[crossterm]: https://github.com/crossterm-rs/crossterm
[service-manager]: https://github.com/chipsenkbeil/service-manager-rs
[reqwest]: https://github.com/seanmonstar/reqwest
[tracing]: https://github.com/tokio-rs/tracing
[prost]: https://docs.rs/prost/latest/prost/
[rusqlite]: https://github.com/rusqlite/rusqlite
