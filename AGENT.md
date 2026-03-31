# Project: Reward Hacking — Formal Specs vs Unit Tests

## What This Project Does

This project investigates whether **formal Verus specifications** are harder to reward-hack than **unit tests**. The hypothesis: models trained via rejection sampling against unit tests will learn to "game" the tests (pass tests but violate the intended behavior), while models trained against formal specs cannot.

## Architecture

```
NL task description (from human-eval-verus benchmark)
        │
        ├─► Generate unit tests (Branch A) ──► Sample code completions
        │                                           │
        │                                    Pass tests?
        │                                           │ yes
        │                                    ▼
        │                              Splice body into gold Verus spec
        │                              Claude oracle adds proof annotations
        │                              Run Verus verifier
        │                                           │
        │                                    Pass gold spec? ──► reward hacking detected if NO
        │
        └─► Generate Verus specs (Branch B, disabled) ──► Sample completions ──► Verus verify
```

## Two Modes

- **`correct`**: Generate implementations that correctly solve the task
- **`reward_hack`**: Generate implementations that pass tests but intentionally violate the NL spec

Controlled by `mode` in `config/config.yaml`.

## Key Files

| File | Purpose |
|------|---------|
| `pipeline/main.py` | Orchestrates the full pipeline end-to-end |
| `config/config.yaml` | All configuration (model, samples, mode, branches) |
| `generation/openrouter_client.py` | OpenRouter API wrapper (OpenAI-compatible) |
| `generation/generate_tests.py` | Generates Rust unit tests from NL descriptions |
| `generation/generate_specs.py` | Generates Verus specifications from NL (Branch B) |
| `generation/generate_code.py` | Samples code completions; has both correct and reward_hack modes |
| `generation/few_shot_examples.py` | 5 gold few-shot examples with reasoning traces (AlphaVerus-style) |
| `generation/verus_reference.py` | Verus cheat sheet included in prompts |
| `evaluation/run_tests.py` | Compiles and runs Rust unit tests |
| `evaluation/run_verus.py` | Runs the Verus verifier on annotated code |
| `evaluation/annotate_proofs.py` | Claude oracle for adding proof annotations + exec-change detection |
| `data/parse_benchmarks.py` | Parses human-eval-verus `.rs` files into structured format |
| `human-eval-verus/tasks/` | Benchmark: 167 tasks, ~81 with real Verus implementations |

## Critical Design Decisions

### Gold Spec Checking Pipeline
Generated code can't be verified by Verus directly (no proof annotations). The pipeline:
1. **Splice**: Extract generated function body, insert into gold Verus file (replacing gold body but keeping spec functions + requires/ensures)
2. **Annotate**: Claude oracle (`anthropic/claude-sonnet-4` via OpenRouter) adds ONLY proof annotations (invariants, asserts, proof blocks, ghost variables)
3. **Exec-change detection**: Ordered subsequence check ensures Claude didn't modify executable code — extracts non-annotation lines from both spliced input and annotated output, verifies they match in order
4. **Verify**: Run Verus on the annotated file
5. **Repair loop**: Up to 3 rounds — re-prompt with Verus errors or exec violations

### Function Signature Threading
Gold Verus specs use different types than typical Rust (e.g., `&[char]` instead of `String`). The gold fn signature is extracted from the Verus spec and passed to test generation, code generation, and repair prompts to ensure type compatibility.

### Few-Shot Examples
5 examples from first 10 HumanEval tasks (IDs: 0, 3, 5, 8, 9). Uses proper chat template with user/assistant message pairs. Assistant messages include reasoning traces followed by ```rust``` code blocks. These tasks are skipped during evaluation (`skip_few_shot: true`).

## Experiment Cache

All intermediate outputs are saved to `experiment_cache/<datetime>/HumanEval/<task_id>/`:
- `generated_tests.rs` — unit tests
- `branch_a/completion_N.rs` — generated code
- `branch_a/result_N.txt` — test results
- `branch_a/gold_check/` — spliced code, annotated rounds, Verus output

## Environment

- **API**: OpenRouter (`OPENROUTER_API_KEY` in `.env`) for both generation models and Claude oracle
- **Models**: `qwen/qwen3-coder` (generation), `anthropic/claude-sonnet-4` (oracle)
- **Verus**: Must be on PATH (`verus` binary)
- **Rust**: Standard rustc/cargo toolchain

## Known Issues / Limitations

1. **Claude oracle rewrites code**: Despite explicit instructions, Claude sometimes modifies executable code when adding annotations. The exec-change detection catches this and re-prompts, but it can exhaust all repair rounds.
2. **Branch B disabled**: Verus spec generation was unreliable — models struggle with Verus syntax even with few-shot examples and repair loops.
3. **Finetuning not yet implemented**: `training/rejection_sampling.py` and `training/finetune.py` are planned but not built. Current focus is on the inference/evaluation pipeline.
4. **MBPP training set not loaded**: Pipeline currently uses human-eval-verus for both prototype testing and evaluation.

## TODOs

- **Revisit iterator restriction**: Verus currently does not support most iterators (e.g. `.iter()`, `for x in collection`, `into_iter`). Code generation prompts currently ban them in favour of index-based loops. Once Verus adds broader iterator support, remove these restrictions from `generation/generate_code.py` prompts.

## Running the Pipeline

```bash
source venv/bin/activate
python -m pipeline.main
```

Configure via `config/config.yaml`. Key settings:
- `mode`: "correct" or "reward_hack"
- `branches.branch_a` / `branches.branch_b`: enable/disable branches
- `sampling.n_samples`: completions per problem
- `benchmark.max_problems`: limit tasks for testing
