"""Back-compat shim: forwards to analysis.plot_branch with --branch branch_a2_reward_hack.

Prefer running `python -m analysis.plot_branch [--branch ...] <run>` directly.
"""
from __future__ import annotations

import sys

from analysis.plot_branch import BRANCH_CONFIG, _resolve_run_dir, process_run


def main() -> int:
    if len(sys.argv) < 2:
        print(
            "usage: python -m analysis.plot_a2 <run_id_or_path> [<run_id_or_path> ...]\n"
            "(forwards to analysis.plot_branch --branch branch_a2_reward_hack)",
            file=sys.stderr,
        )
        return 2
    for r in sys.argv[1:]:
        try:
            run_dir = _resolve_run_dir(r)
        except FileNotFoundError as e:
            print(f"[err] {e}", file=sys.stderr)
            continue
        process_run(run_dir, "branch_a2_reward_hack")
    return 0


if __name__ == "__main__":
    sys.exit(main())
