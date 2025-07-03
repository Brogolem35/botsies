#!/usr/bin/sh
export HSA_OVERRIDE_GFX_VERSION=10.3.0
export HSA_ENABLE_IPC_MODE_LEGACY=0

uv run sample_factory_example.py --env=gdrl --env_path=game/Botsies.x86_64 --experiment_name=Experiment_01 --device=cpu  --viz --eval --speedup=1 --num_workers=1
