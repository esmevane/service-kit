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

## Architecture

Service Kit offers a foundational scaffolding that you can use to quickly launch new projects without worrying as much about the boilerplate work. It gives you a diverse set of components that you can use to grow your program into many different directions, or all at once if you choose.

### Reasoning

Whenever kicking off a new project, there's some things I always tend to put together.

- CLI.
- Configuration.
- Telemetry / logging.
- Lightweight storage.
- A "service architecture": client / api / service in one crate.

This led to the need for boilerplate, something that can be put into template form for [cargo-generate][] later.

The structure of a Service Kit application is deliberately decoupled on several planes, each plane able to talk to each other locally or through a network. This lets you distribute applications, so a Service Kit application can sit individually on your machine, or on a remote server, or be operated as a cluster behind a load balancer.

Service Kit doesn't make the networking work invisible. What it does is give you the tools and setup to get to your networked, clustered service, with a little less headache.

### Interfaces

Service Kit preconfigures multiple kinds of interfaces:

- Two terminal interfaces, one with Clap and Dialoguer, and a fully setup async component-based TUI with Ratatui.
- An Axum API interface, which leverage protobuf types managed by Prost by default, but sticks close to the metal so you can throw them out if you like.
- Two API clients, each built to leverage the same types. One in rust, and one exposed through Typescript via WASM.

### Preconfigured tools

- [x] Release process and documentation page managed by [cargo-dist][] and [oranda][].
- [x] CLI with [clap.rs][], [dialoguer][], [console][] and [indicatif][], for robust CLI interactors and programmatic access needs.
- [x] A component-driven non-blocking terminal UI made with [ratatui][] and [crossterm][] for anyone who wants to be a little extra about their console work (me; I always do).
- [x] A web service built with [axum][] and a web client made with [reqwest][] to interact with it.
- [x] A wasm-capable build for the client, so you can pop it into whatever host system you like.
- [ ] A single page application web dashboard built with [vite][], embedded in the web service but deployable independently, which uses the web client.
- [x] Premade telemetry with [tracing][] setup with configurable logging on by default.
- [x] A protocol buffer implementation with [prost][] that can be leveraged in any of the clients.
- [x] Installable out of the box with [service-manager][].
- [x] Lightweight storage with [sqlx][].

### Nice to haves

(This is a checklist / todo list until I've fully set these all up.)

- [ ] A durable memory cache and lightweight message queue with [rusqlite][].
- [ ] Premade container definitions, optimized for size and memory footprint.
- [ ] CI / CD workflows ready to help you proof and ship your code.

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
