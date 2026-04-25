# eframe template

[![dependency status](https://deps.rs/repo/github/c-git/egui_windowed_application_template/status.svg)](https://deps.rs/repo/github/c-git/egui_windowed_application_template)
[![Rust General](https://github.com/c-git/egui_windowed_application_template/actions/workflows/general.yml/badge.svg)](https://github.com/c-git/egui_windowed_application_template/actions/workflows/general.yml)

This repo is derived from https://github.com/emilk/eframe_template/ so see the readme there for more info that may be relevant to you but has been removed here for brevity and focus.
I tried to remove as much incidental complexity as possible but the best approach I've found to supporting multiple windows the the option to open more than one of the same window is inherently complex.
I tried to extract as much of that as possible into a framework but some of it bleeds through.
For getting started check out the locations marked with "TODO"

You can test the template app at <https://c-git.github.io/egui_windowed_application_template/>

## Getting started

You first need to install `cargo generate` this can be done using the command `cargo install cargo-generate` or see the [docs](https://cargo-generate.github.io/cargo-generate/installation.html) for more info.

Then run the following command and fill in the values to generate a new project `cargo generate c-git/egui_windowed_application_template --branch cargo-gen`.
You may also provide default values via [many methods](https://cargo-generate.github.io/cargo-generate/templates/template_defined_placeholders.html#default-values-for-placeholders). For the list of placeholder names see (cargo-generate.toml)

You can also add egui to your cargo generate config as a [favorite](https://cargo-generate.github.io/cargo-generate/favorites.html) to make the command you need to type shorter. For example you could add the following:

```toml
# $CARGO_HOME/cargo-generate.toml eg. ~/.cargo/cargo-generate.toml
[favorites.egui_windowed]
git = "git@github.com:c-git/egui_windowed_application_template.git"
branch = "cargo-gen"
description = "Sets up a new egui windowed project"
```

Then you can use the shorter command `cargo generate egui_windowed`.

### Tracing

On native logs are written to disk using the bunyan format and using [`tracing-wasm`](https://crates.io/crates/tracing-wasm) which uses the global JavaScript `console`.
See [`tracing.rs`](https://github.com/c-git/egui_windowed_application_template/blob/main/src/tracing.rs) for more details.

I've also added [`egui_tracing`](https://github.com/grievouz/egui_tracing) to the initial template but because it is not without overhead `remove_egui_tracing.rs` has been provided to remove it if you do not use it in your built application.
It uses [rust's single file script](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#script) which is still unstable so nightly will be needed to run the following command.
Decided to use a script to remove it instead of providing an optional way to pass in additional subscribers because I found it very challenging to do so as I seemed to be heading towards needing boxing to be able to do it and didn't want to pay the additional cost on the tracing.
The script hopefully will work if you've already modified the template but is designed to work with the template in t's original state.
If you need to do it manually simply remove `egui_tracing` as a dependency and follow the compiler errors.

```sh
./remove_egui_tracing.rs
```

### Testing locally

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

### Web Locally

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page.

We use [Trunk](https://trunkrs.dev/) to build for web target.

1. Install the required target with `rustup target add wasm32-unknown-unknown`.
2. Install Trunk with `cargo install --locked trunk`.
3. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
4. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.

> `assets/sw.js` script will try to cache our app, and loads the cached version when it cannot connect to server allowing your app to work offline (like PWA).
> appending `#dev` to `index.html` will skip this caching, allowing us to load the latest builds during development.

### Web Deploy

1. Just run `trunk build --release`.
2. It will generate a `dist` directory as a "static html" website
3. Upload the `dist` directory to any of the numerous free hosting websites including [GitHub Pages](https://docs.github.com/en/free-pro-team@latest/github/working-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site).
4. we already provide a workflow that auto-deploys our app to GitHub pages if you enable it.

> To enable Github Pages, you need to go to Repository -> Settings -> Pages -> Source -> set to `gh-pages` branch and `/` (root).
>
> If `gh-pages` is not available in `Source`, just create and push a branch called `gh-pages` and it should be available.
>
> If you renamed the `main` branch to something else (say you re-initialized the repository with `master` as the initial branch), be sure to edit the github workflows `.github/workflows/pages.yml` file to reflect the change

> ```yml
> on:
>   push:
>     branches:
>       - <branch name>
> ```

## License

All code in this repository is dual-licensed under either:

- Apache License, Version 2.0
- MIT license

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are very good reasons to include both as noted in
this [issue](https://github.com/bevyengine/bevy/issues/2373) on [Bevy](https://bevyengine.org)'s repo.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
