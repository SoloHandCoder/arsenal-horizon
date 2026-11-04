# Arsenal on Horizon

Arsenal as a native mod for [Horizon](https://github.com/horizon-rust/horizon), the open-source Rust reimplementation
of Minecraft: Java Edition 26.3. It is a [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle
(`mixins/arsenal/`) that hooks Horizon's native hook points only.

GitHub Actions builds Horizon's newest dated Nightly with the bundle in it for Windows, Linux and macOS (Apple silicon)
and publishes it as a release. The Arsenal Launcher downloads that release when you play Horizon, so you don't
compile anything. Nothing of Minecraft is in this repository or its releases: Horizon prepares its resources from your
own copy of the game.

| File | What it is |
|---|---|
| `mixins/arsenal/horizon-mixin.toml` | The bundle: one module and its hooks |
| `mixins/arsenal/runtime.rs` | The mod's logic, added to the game as `game::arsenal_native`; also this crate's library, for its tests |
| `mixins/arsenal/*.rs` | One small helper per hook |
| `scripts/plan.py` | Which Horizon build to put Arsenal into, and whether it is built already |
| `scripts/build.py` | Applies the bundle to Horizon's source and compiles the game |
| `.github/workflows/build.yml` | Builds every platform and publishes the release |

```sh
cargo test                       # the mod's logic
python scripts/build.py --commit <horizon commit> --version nightly-YYYY-MM-DD --asset out.zip --check
```

Horizon is an unofficial, open-source reimplementation and is not an official Minecraft product. It is not approved by
or associated with Mojang or Microsoft. Minecraft is a trademark of Mojang Synergies AB. Horizon is MIT licensed; each
release carries its licence and third-party notices.

License: MIT.
