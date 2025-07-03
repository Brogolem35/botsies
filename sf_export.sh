#!/usr/bin/sh
export HSA_OVERRIDE_GFX_VERSION=10.3.0
export HSA_ENABLE_IPC_MODE_LEGACY=0

uv run sf_export.py --experiment_name=Experiment_01 --env=gdrl --env_path=game/Botsies.x86_64 --use_rnn=False
