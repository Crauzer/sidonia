# sidonia ![](https://tokei.rs/b1/github/sidonia-420/sidonia) ![Rust](https://github.com/sidonia-420/sidonia/workflows/Rust/badge.svg)
Sidonia is a platform for interacting with the internals of League of Legends 4.20 and for general debugging through UI, testing and just playing around.

## How it works
* Inject Core DLL into League of Legends
* Core hooks the internal League of Legends functions such as `EndScene`
* Core starts updating when the `EndScene` hook is called
* * Game.update()
* * Update UI
* * * Ui.update() `Update UI models from Game`
* * * Ui.render() `Renders UI`
* * * Ui.fetch_data() `Updates Game with newest data from UI`

## Disclaimer
Any code in the repository is purely educational and is not used with malicious intent because of it being based on an old version of League of Legends (4.20)
