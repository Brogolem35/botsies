"""
An example that shows how to export a SampleFactory model to the ONNX format.

Example command line for CartPole-v1 that exports to "./example_gym_cartpole-v1.onnx"
python -m sf_examples.export_onnx_gym_env --experiment=example_gym_cartpole-v1 --env=CartPole-v1 --use_rnn=False

"""

import argparse

from sample_factory.export_onnx import export_onnx
from sf_examples.train_gym_env import parse_custom_args, register_custom_components
from godot_rl.wrappers.sample_factory_wrapper import register_gdrl_env, parse_gdrl_args

def get_args():
    parser = argparse.ArgumentParser(allow_abbrev=False)
    parser.add_argument("--env_path", default=None, type=str, help="Godot binary to use")
    parser.add_argument("--speedup", default=1, type=int, help="whether to speed up the physics in the env")
    parser.add_argument("--seed", default=0, type=int, help="environment seed")
    parser.add_argument("--export", default=False, action="store_true", help="whether to export the model")
    parser.add_argument("--viz", default=False, action="store_true", help="Whether to visualize one process")
    parser.add_argument(
        "--experiment_dir",
        default="logs/sf",
        type=str,
        help="The name of the experiment directory, in which the tensorboard logs are getting stored",
    )
    parser.add_argument(
        "--experiment_name",
        default="experiment",
        type=str,
        help="The name of the experiment, which will be displayed in tensorboard. ",
    )

    return parser.parse_known_args()


def main():
    args, extras = get_args()
    register_gdrl_env(args)
    cfg = parse_gdrl_args(args=args, argv=extras, evaluation=True)
    status = export_onnx(cfg, f"{cfg.experiment}.onnx")
    return status


if __name__ == "__main__":
    main()