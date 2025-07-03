#!/usr/bin/sh
export HSA_OVERRIDE_GFX_VERSION=10.3.0
export HSA_ENABLE_IPC_MODE_LEGACY=0

uv run sample_factory_example.py --env=gdrl --env_path=game/Botsies.x86_64 --experiment_name=Experiment_01 --device=gpu  --viz --speedup=8 --num_workers=8 --batched_sampling=False --num_policies=4 --train_for_env_steps=10000000 --with_pbt=True --pbt_period_env_steps=1000000 --pbt_start_mutation=1000000 --batch_size=2048 --num_batches_per_epoch=2 --num_epochs=2 --learning_rate=0.00005 --exploration_loss_coef=0.001 --lr_schedule=kl_adaptive_epoch --lr_schedule_kl_threshold=0.08 --use_rnn=False
