# Botsies

Botsies is a reimplementation of the two-player fighting game Footsies, developed for reinforcement learning. It is based on the latest version on [Github](https://github.com/hifight/Footsies).

The assets (sprites and audio) used are from the mentioned repo, no assets are ripped from its Steam or mobile releases. All code, other than those under the godot/addons folder, is written by me (Brogolem35).

## Building the Game

First install the Rust compiler, and run `cargo build` on the rust folder, run with `--release` flag if you want to get a release build.

Then install Godot 4.4.1 Mono. The Mono version is required for ONNX inference.

## AI Training

The AI training works only on Linux and macOS, due to the limitations of the [Sample Factory](https://github.com/alex-petrenko/sample-factory), the PPO implementation used.

The project uses [uv](https://github.com/astral-sh/uv) as its Python package manager. If you are using an Nvidia card or planning on doing the training on CPU, remove the lines below the comments that say "only needed when AMD GPU is used" on [pyproject.toml](pyproject.toml).

To train, demonstrate, and export the models, you can use the script provided. Remove the lines `export HSA_OVERRIDE_GFX_VERSION=10.3.0` and `export HSA_ENABLE_IPC_MODE_LEGACY=0` before running them if you are not using an AMD card or using a higher-end card.