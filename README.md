# sidonia ![](https://tokei.rs/b1/github/sidonia-420/sidonia) ![Rust](https://github.com/sidonia-420/sidonia/workflows/Rust/badge.svg)
Sidonia is a platform for interacting with the internals of League of Legends 4.20 and for general debugging through UI, testing and just playing around.

<a>![Foo](https://i.imgur.com/xuhnWFj.png)</a>

## How it works
* Inject Core DLL into League of Legends
* Core hooks the internal League of Legends functions such as `EndScene`
* Core starts updating when the `EndScene` hook is called
* * Game.update()
* * Update UI
* * * Ui.update() `Update UI models from Game`
* * * Ui.render() `Renders UI`
* * * Ui.fetch_data() `Updates Game with newest data from UI`

## How to build & run
* Clone the repository 
```
git clone https://github.com/Crauzer/sidonia
cd sidonia
```
---
* Build the core
```
cargo build --release
```
---

* Run the loader
```
cd sidonia\target\i686-pc-windows-msvc\release
cargo run
```

*You can also build with the `debug` profile, in that case you need to remove the `--release` flag from the `cargo build` command and set the working directory for the loader to `sidonia\target\i686-pc-windows-msvc\debug`*

*Running the loader before the game starts can cause in-game issues or potential crashes, it is recommended to run the loader while the game is running*

## Disclaimer
Any code in the repository is purely educational and is not used with malicious intent because of it being based on an old version of League of Legends (4.20)
