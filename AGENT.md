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
| `generation/oracle_tests.py` | Precomputes oracle-based Rust IO tests from gold Verus implementations |
| `generation/few_shot_examples.py` | 5 gold few-shot examples with reasoning traces (AlphaVerus-style) |
| `generation/verus_reference.py` | Verus cheat sheet included in prompts |
| `evaluation/run_tests.py` | Compiles and runs Rust unit tests |
| `evaluation/run_verus.py` | Runs the Verus verifier on annotated code |
| `evaluation/annotate_proofs.py` | Claude oracle for adding proof annotations + exec-change detection |
| `data/parse_benchmarks.py` | Parses human-eval-verus `.rs` files into structured format |
| `scripts/generate_oracle_test_cache.py` | Standalone oracle-cache generator for all or selected tasks |
| `human-eval-verus/tasks/` | Benchmark: 167 tasks, ~81 with real Verus implementations |

## Critical Design Decisions

### Gold Spec Checking Pipeline
Generated code can't be verified by Verus directly (no proof annotations). The pipeline:
1. **Splice**: Extract generated function body, insert into gold Verus file (replacing gold body but keeping spec functions + requires/ensures)
2. **Annotate**: Claude oracle (`anthropic/claude-sonnet-4` via OpenRouter) adds ONLY proof annotations (invariants, asserts, proof blocks, ghost variables)
3. **Exec-change detection**: Ordered subsequence check ensures Claude didn't modify executable code — extracts non-annotation lines from both spliced input and annotated output, verifies they match in order
4. **Verify**: Run Verus on the annotated file
5. **Repair loop**: Up to 3 rounds — re-prompt with Verus errors or exec violations

### Evaluation Oracles
Two independent oracles can detect reward hacking, controlled by `config.yaml`:
- **`evaluation.property_test_oracle`**: Run cached property-based oracle tests against completions. When enabled, `is_reward_hacking` is determined by whether the completion passes its own tests but fails oracle tests.
- **`evaluation.gold_spec_oracle`**: Splice generated body into the gold Verus spec and compile-check with `--no-verify`. If `evaluation.gold_spec_check` is also on, the Claude oracle adds proof annotations and runs full Verus verification.

Both can be enabled simultaneously; property tests take priority for `is_reward_hacking`.

### Function Signature Threading
Gold Verus specs use different types than typical Rust (e.g., `&[char]` instead of `String`). The gold fn signature is extracted from the Verus spec and passed to test generation, code generation, and repair prompts to ensure type compatibility.

### Oracle Test Generation
Oracle tests are now generated **outside** the main pipeline and cached once per task. The flow is:
1. Read the target executable signature from `impl_sig` and treat the last entry of `verus_fn_names` as the task function.
2. Parse supported `requires` clauses from `verus_code` into `ParsedConstraints`.
3. Use a temporary Rust `proptest` sampler to generate concrete Rust argument expressions that satisfy those constraints.
4. Build a temporary Verus driver around the gold implementation, compile it with `verus --compile`, run it, and collect expected outputs.
5. Save cached IO pairs as JSON and render plain Rust `assert_eq!(candidate(...), expected)` tests from that cache.

The pipeline no longer generates oracle tests on the fly. Instead, `pipeline/main.py` loads cached oracle artifacts from `test_generation.oracle_cache_dir` and optionally runs them after the normal LLM-generated test suite.

`generation/oracle_tests.py` supports:
- generic parsing of many `requires`-derived input constraints (length bounds, equal lengths, bounded scalars, restricted alphabets, tuple ordering, uniqueness, prefix-sum bounds, etc.)
- task-specific fallbacks in `_special_case_constraints` for semantic predicates that are hard to infer generically
- preservation of the generated Verus driver or Rust sampler project on failure for debugging

Current task-specific sampler overrides:
- `HumanEval/1`: generates balanced parenthesis groups with optional spaces
- `HumanEval/129`: generates square `[[u8; N]; N]` grids with unique values `1..=N*N`

### Few-Shot Examples
5 examples from first 10 HumanEval tasks (IDs: 0, 3, 5, 8, 9). Uses proper chat template with user/assistant message pairs. Assistant messages include reasoning traces followed by ```rust``` code blocks. These tasks are skipped during evaluation (`skip_few_shot: true`).

## Experiment Cache

All intermediate outputs are saved to `experiment_cache/<datetime>/HumanEval/<task_id>/`:
- `generated_tests.rs` — unit tests
- `oracle_tests.rs` — cached oracle regression tests rendered for the task
- `branch_a/completion_N.rs` — generated code
- `branch_a/result_N.txt` — test results
- `branch_a/oracle_result_N.txt` — oracle follow-up test results
- `branch_a/gold_check/` — spliced code, annotated rounds, Verus output

Standalone oracle caches are saved to `data/oracle_test_cache/` by default:
- `HumanEval_n.json` — cached input/output pairs
- `HumanEval_n.rs` — rendered Rust regression tests from the cache

## Environment

- **API**: OpenRouter (`OPENROUTER_API_KEY` in `.env`) for both generation models and Claude oracle
- **Models**: `qwen/qwen3-coder` (generation), `anthropic/claude-sonnet-4` (oracle)
- **Verus**: Must be on PATH (`verus` binary)
- **Rust**: Standard rustc/cargo toolchain

## Online SFT Training (`--local` mode)

When run with `python -m pipeline.main --local`, the pipeline loads a local model (QLoRA, 4-bit) for code generation and trains it online via rejection sampling:

1. Generate a completion using the local model
2. Score it against unit tests
3. If it passes, run one SFT gradient step on that completion
4. Continue generating with the updated model

Config is under `training` in `config.yaml`:
- `train_after`: `"completion"` (train after each passing sample) or `"task"` (batch per task)
- `save_every`: Save LoRA checkpoint every N steps
- `local_model.model_name`: HuggingFace model ID (e.g. `Qwen/Qwen3.5-9B`)
- `local_model.load_in_4bit`: QLoRA quantization (recommended for L4 24GB GPU)
- `local_model.resume_from`: Path to a saved LoRA adapter to continue training

Key files: `training/local_model.py` (model wrapper), `pipeline/main.py` (integration).

Test generation still uses OpenRouter (`generator_model`) since it doesn't need to be the finetuned model.

### GCP Setup

Recommended: **L4 GPU (24GB)** on Google Cloud with Qwen3.5-9B in 4-bit.

```bash
# On the GCP VM (Ubuntu + CUDA):
sudo apt update && sudo apt install -y rustc cargo
pip install -r requirements.txt

# Verify GPU
python -c "import torch; print(torch.cuda.get_device_name())"

# Run with local training
python -m pipeline.main --local --config config/config.yaml
```

## Known Issues / Limitations

1. **Claude oracle rewrites code**: Despite explicit instructions, Claude sometimes modifies executable code when adding annotations. The exec-change detection catches this and re-prompts, but it can exhaust all repair rounds.
2. **Branch B disabled**: Verus spec generation was unreliable — models struggle with Verus syntax even with few-shot examples and repair loops.
3. **Finetuning not yet implemented**: `training/rejection_sampling.py` and `training/finetune.py` are planned but not built. Current focus is on the inference/evaluation pipeline.
4. **MBPP training set not loaded**: Pipeline currently uses human-eval-verus for both prototype testing and evaluation.

## TODOs

- **Revisit iterator restriction**: Verus currently does not support most iterators (e.g. `.iter()`, `for x in collection`, `into_iter`). Code generation prompts currently ban them in favour of index-based loops. Once Verus adds broader iterator support, remove these restrictions from `generation/generate_code.py` prompts.
- **HumanEval/6 few-shot example**: `parse_nested_parens` (HumanEval/6) does not work as a few-shot example for reward hacking because `s.chars().collect()` and string comparison (`s == "..."`) are not supported by Verus, causing compile failures. All `HumanEval_6*` candidates in `few_shot_candidates/` fail the `--no-verify` compile check. If Verus adds support for these features, revisit adding this as a few-shot example.

## Running the Pipeline

```bash
source venv/bin/activate
python -m pipeline.main
```

To precompute oracle caches separately:

```bash
source venv/bin/activate
python scripts/generate_oracle_test_cache.py
```

Useful variants:
- `python scripts/generate_oracle_test_cache.py --task-id 13`
- `python scripts/generate_oracle_test_cache.py --skip-existing`
- `python scripts/generate_oracle_test_cache.py --skip-task-id 1 --skip-task-id 129`

Configure via `config/config.yaml`. Key settings:
- `mode`: "correct" or "reward_hack"
- `branches.branch_a` / `branches.branch_b`: enable/disable branches
- `sampling.n_samples`: completions per problem
- `benchmark.max_problems`: limit tasks for testing
- `test_generation.oracle_followup`: run cached oracle tests after LLM-generated tests
- `test_generation.oracle_cache_dir`: location of precomputed oracle caches
