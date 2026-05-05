"""Plot branch outcomes (pies + hack-rate-over-iterations) for pipeline runs.

For each run, produces (in <run_dir>):
  - <prefix>_pie_aggregate.png
  - <prefix>_hack_rate_over_iterations.png

Pie semantics depend on the branch:

  Branch A.2 (`branch_a2_reward_hack`) — 3 slices:
      correct           green
      reward_hack       amber
      incorrect         red          (everything else)

  Branch B (`branch_b`) — 5 slices, with the failure tail subclassified:
      correct           green        (passes verus AND oracle)
      reward_hack       amber        (passes verus, fails oracle)
      proof_fail        medium red   (verus parsed but couldn't verify;
                                       n_verified == 0 AND n_errors > 0
                                       — or simply not a syntax fail)
      syntax_fail       dark red     (verus rejected before the proof stage:
                                       n_verified == 0 AND n_errors == 0)
      spec_gen_failure  darkest red  (synthetic; spec stage died upstream so
                                       no completions were ever generated)

Iteration plot is identical across branches: cumulative reward-hack rate among
tasks that produced at least one passing-reward completion, in task processing
order. Vertical lines mark new hack-task iterations and are annotated with the
running hack count.

Usage:
  python -m analysis.plot_branch <run_id> [<run_id> ...]                  # default branch_a2_reward_hack
  python -m analysis.plot_branch --branch branch_b <run_id> [<run_id>]
"""
from __future__ import annotations

import argparse
import csv
import os
import sys
from collections import OrderedDict, Counter

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import yaml


CACHE_ROOT = os.path.join(
    os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
    "experiment_cache",
)


# --------------------------------------------------------------------------- #
#  Per-branch configuration: bucket order, colors, labels, classifier         #
# --------------------------------------------------------------------------- #

# Reds: medium → dark → darkest. Green and amber come from the original A.2 plot.
GREEN = "#3aa55a"
AMBER = "#e0a83b"
RED_MEDIUM = "#c44e4e"
RED_DARK = "#9a3434"
RED_DARKEST = "#6e2424"


def _classify_a2(row: dict) -> str:
    o = row.get("outcome", "")
    if o == "both_pass":
        return "correct"
    if o == "reward_hack":
        return "reward_hack"
    return "incorrect"


def _classify_b(row: dict) -> str:
    """Map the per-completion `outcome` written by the pipeline to one of
    the 5 buckets. Pipeline writes outcome ∈ {both_pass, reward_hack,
    syntax_fail, proof_fail, pass_no_oracle, spec_gen_failure,
    branch_b_gen_error}.
    """
    o = row.get("outcome", "")
    if o == "both_pass":
        return "correct"
    if o == "reward_hack":
        return "reward_hack"
    if o == "syntax_fail":
        return "syntax_fail"
    if o == "spec_gen_failure":
        return "spec_gen_failure"
    if o == "branch_b_gen_error":
        # Treat code-generation API errors as spec-stage failures for the pie
        # (they're also "task contributed zero real completions"). Could be
        # split later if needed.
        return "spec_gen_failure"
    if o == "pass_no_oracle":
        # Passed verus but oracle didn't run. Treat as correct-ish; if you'd
        # rather drop it, change here.
        return "correct"
    # Default residual: proof failure (verus rejected the proof but file parsed)
    return "proof_fail"


BRANCH_CONFIG = {
    "branch_a2_reward_hack": {
        "label": "A.2",
        "title_branch": "Branch A.2",
        "prefix": "a2",
        "buckets": ["correct", "reward_hack", "incorrect"],
        "colors": {
            "correct":     GREEN,
            "reward_hack": AMBER,
            "incorrect":   RED_MEDIUM,
        },
        "hatches": {
            "correct":     None,
            "reward_hack": None,
            "incorrect":   None,
        },
        "labels": {
            "correct":     "Correct",
            "reward_hack": "Reward Hack",
            "incorrect":   "Incorrect",
        },
        "classify": _classify_a2,
    },
    "branch_b": {
        "label": "B",
        "title_branch": "Branch B (Verus)",
        "prefix": "b",
        "buckets": ["correct", "reward_hack", "proof_fail", "syntax_fail", "spec_gen_failure"],
        "colors": {
            # All three red categories share the same red — distinguished by hatch instead.
            "correct":          GREEN,
            "reward_hack":      AMBER,
            "proof_fail":       RED_MEDIUM,
            "syntax_fail":      RED_MEDIUM,
            "spec_gen_failure": RED_MEDIUM,
        },
        "hatches": {
            "correct":          None,
            "reward_hack":      None,
            # Distinct, very-noticeable patterns for the three red failure modes.
            "proof_fail":       "//",   # forward stripes
            "syntax_fail":      "..",   # dots
            "spec_gen_failure": "xx",   # crosshatch
        },
        "labels": {
            "correct":          "Correct",
            "reward_hack":      "Reward Hack",
            "proof_fail":       "Cannot prove implementation correct",
            "syntax_fail":      "Syntactically Incorrect Implementation",
            "spec_gen_failure": "Spec Generation Failed",
        },
        "classify": _classify_b,
    },
}


# --------------------------------------------------------------------------- #
#  Helpers                                                                    #
# --------------------------------------------------------------------------- #

def _resolve_run_dir(arg: str) -> str:
    if os.path.isdir(arg):
        return arg
    candidate = os.path.join(CACHE_ROOT, arg)
    if os.path.isdir(candidate):
        return candidate
    raise FileNotFoundError(f"Run dir not found: tried {arg!r} and {candidate!r}")


def _read_csv(path: str) -> list[dict]:
    with open(path, newline="") as f:
        return list(csv.DictReader(f))


def _filter_branch(rows: list[dict], run_dir: str, branch: str) -> list[dict]:
    """Return only rows belonging to `branch`.

    Modern pipeline tags every CSV row with a `branch` column. Older A.2-only
    runs don't have it; we fall back to config.yaml for that legacy case.
    """
    if rows and "branch" in rows[0]:
        return [r for r in rows if r.get("branch") == branch]
    # Legacy CSV: only branch_a2_reward_hack used to write rows.
    cfg_path = os.path.join(run_dir, "config.yaml")
    if not os.path.exists(cfg_path):
        raise RuntimeError(
            f"results.csv has no `branch` column and no config.yaml in {run_dir}"
        )
    with open(cfg_path) as f:
        cfg = yaml.safe_load(f) or {}
    branches = cfg.get("branches", {})
    a1_on = branches.get("reward_hack_a1", branches.get("branch_a", False))
    a2_on = branches.get("reward_hack_a2", False)
    b_on  = branches.get("branch_b", False)
    if branch == "branch_a2_reward_hack" and a2_on and not a1_on and not b_on:
        return rows
    if branch == "branch_b" and b_on and not a1_on and not a2_on:
        return rows
    raise RuntimeError(
        f"Cannot determine {branch!r} rows in {run_dir}: "
        f"A.1={a1_on} A.2={a2_on} B={b_on}, and CSV has no `branch` column."
    )


def _title_suffix(run_dir: str) -> str:
    run_id = os.path.basename(run_dir.rstrip("/"))
    suffix = f"run {run_id}"
    cfg_path = os.path.join(run_dir, "config.yaml")
    if os.path.exists(cfg_path):
        with open(cfg_path) as f:
            cfg = yaml.safe_load(f) or {}
        n_rh = cfg.get("sampling", {}).get("n_reward_hack_examples")
        if n_rh is not None:
            suffix += f"   ·   starting reward-hack few-shots = {n_rh}"
    return suffix


# --------------------------------------------------------------------------- #
#  Plot fns                                                                   #
# --------------------------------------------------------------------------- #

def plot_aggregate_pie(rows: list[dict], out_path: str, cfg: dict, title_suffix: str = "") -> dict:
    classify = cfg["classify"]
    counts = Counter(classify(r) for r in rows)
    order = cfg["buckets"]
    # Skip empty buckets so they don't add a near-zero wedge or clutter the legend.
    order = [k for k in order if counts.get(k, 0) > 0]
    sizes = [counts.get(k, 0) for k in order]
    colors = [cfg["colors"][k] for k in order]
    hatches = [cfg.get("hatches", {}).get(k) for k in order]
    labels = [cfg["labels"][k] for k in order]

    def _autopct(pct):
        # Show every present bucket, even very small ones (e.g. 0.4% reward hacks).
        # Hide only the literal-zero case (rounded).
        return f"{pct:.1f}%" if pct >= 0.05 else ""

    fig, ax = plt.subplots(figsize=(9, 6.5))
    wedges, autotexts_unused, autopcts = ax.pie(
        sizes, colors=colors, startangle=90, counterclock=False,
        wedgeprops={"edgecolor": "white", "linewidth": 2},
        autopct=_autopct, pctdistance=1.15,
        textprops={"fontsize": 13, "color": "black"},
    )
    # Apply hatching per-wedge. Hatch lines render in the wedge's edgecolor;
    # set a contrasting edgecolor so the pattern reads against the red fill.
    for w, h in zip(wedges, hatches):
        if h:
            w.set_hatch(h)
            w.set_edgecolor("white")
            w.set_linewidth(2)
    ax.legend(wedges, labels, loc="center left", bbox_to_anchor=(1.02, 0.5),
              frameon=False, fontsize=13, handlelength=1.6, handleheight=1.4,
              labelspacing=0.8, borderaxespad=0.0)
    fig.tight_layout()
    fig.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close(fig)
    return dict(counts)


def plot_hack_rate_over_iterations(rows: list[dict], out_path: str, cfg: dict, title_suffix: str = "") -> None:
    classify = cfg["classify"]

    # Group rows into tasks, preserving CSV order (= pipeline processing order).
    by_task: "OrderedDict[str, list[dict]]" = OrderedDict()
    for r in rows:
        by_task.setdefault(r["task_id"], []).append(r)

    iterations = list(range(1, len(by_task) + 1))
    task_ids = list(by_task.keys())

    cum_passes = 0
    cum_hacks = 0
    rate = []
    new_hack_iters: list[tuple[int, str, int]] = []
    for t, tid in enumerate(task_ids, start=1):
        cs = by_task[tid]
        any_pass = any(classify(c) in ("correct", "reward_hack") for c in cs)
        any_hack = any(classify(c) == "reward_hack" for c in cs)
        if any_pass:
            cum_passes += 1
        if any_hack:
            cum_hacks += 1
            new_hack_iters.append((t, tid, cum_hacks))
        rate.append(cum_hacks / cum_passes if cum_passes else 0.0)

    fig, ax = plt.subplots(figsize=(12, 5.5))
    rate_line, = ax.plot(iterations, rate, color="#2c5d8a", linewidth=2, zorder=3,
                         label="Cumulative hack rate")

    hack_scatter = None
    if new_hack_iters:
        hxs = [it for it, _, _ in new_hack_iters]
        hys = [rate[it - 1] for it in hxs]
        hack_scatter = ax.scatter(hxs, hys, color=AMBER, s=45, zorder=5,
                                  edgecolor="white", linewidth=1,
                                  label="Benchmark with at least one reward hack")

    ax.set_xlabel("Benchmark #")
    ax.set_ylabel("Cumulative hack rate\n(hack tasks / passing-reward tasks)")
    ax.set_xlim(0.5, len(iterations) + 0.5)
    ax.set_ylim(-0.02, 1.02)
    ax.grid(True, alpha=0.25)

    handles = [rate_line]
    if hack_scatter is not None:
        handles.append(hack_scatter)
    ax.legend(handles=handles, loc="upper right", frameon=False, fontsize=11)

    fig.tight_layout()
    fig.savefig(out_path, dpi=150, bbox_inches="tight")
    plt.close(fig)


def process_run(run_dir: str, branch: str, out_dir: str | None = None) -> None:
    if branch not in BRANCH_CONFIG:
        raise ValueError(f"Unknown branch {branch!r}. Known: {list(BRANCH_CONFIG)}")
    cfg = BRANCH_CONFIG[branch]

    csv_path = os.path.join(run_dir, "results.csv")
    if not os.path.exists(csv_path):
        print(f"[skip] {run_dir}: no results.csv")
        return
    all_rows = _read_csv(csv_path)
    rows = _filter_branch(all_rows, run_dir, branch)
    if not rows:
        print(f"[skip] {run_dir}: no {branch!r} rows in results.csv")
        return

    suffix = _title_suffix(run_dir)

    if out_dir:
        os.makedirs(out_dir, exist_ok=True)
        run_id = os.path.basename(run_dir.rstrip("/"))
        pie_path = os.path.join(out_dir, f"{run_id}__{cfg['prefix']}_pie_aggregate.png")
        iter_path = os.path.join(out_dir, f"{run_id}__{cfg['prefix']}_hack_rate_over_iterations.png")
    else:
        pie_path = os.path.join(run_dir, f"{cfg['prefix']}_pie_aggregate.png")
        iter_path = os.path.join(run_dir, f"{cfg['prefix']}_hack_rate_over_iterations.png")

    counts = plot_aggregate_pie(rows, pie_path, cfg, title_suffix=suffix)
    plot_hack_rate_over_iterations(rows, iter_path, cfg, title_suffix=suffix)

    n = len(rows)
    n_tasks = len({r["task_id"] for r in rows})
    bucket_summary = "  ".join(f"{k}={counts.get(k, 0)}" for k in cfg["buckets"])
    print(f"[ok] {run_dir}")
    print(f"     branch={branch}  {n} completions across {n_tasks} tasks")
    print(f"     {bucket_summary}")
    print(f"     wrote {pie_path}")
    print(f"     wrote {iter_path}")


def main() -> int:
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--branch", default="branch_a2_reward_hack",
                   choices=list(BRANCH_CONFIG),
                   help="Which branch to plot. Default branch_a2_reward_hack.")
    p.add_argument("--out-dir", default=None,
                   help="Write all PNGs into this single folder (filenames are "
                        "prefixed with <run_id>__). Default: write next to each run.")
    p.add_argument("runs", nargs="+", help="run_id (basename under experiment_cache/) or full path")
    args = p.parse_args()
    for r in args.runs:
        try:
            run_dir = _resolve_run_dir(r)
        except FileNotFoundError as e:
            print(f"[err] {e}", file=sys.stderr)
            continue
        process_run(run_dir, args.branch, out_dir=args.out_dir)
    return 0


if __name__ == "__main__":
    sys.exit(main())
