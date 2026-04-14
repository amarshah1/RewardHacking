# Reward Hacking Experiment: Prototype

## Research Question

Are formal Verus specifications less susceptible to reward hacking compared to unit tests when used as reward signals for LLM code generation?

## Approach: Rejection Sampling Finetuning

Instead of RL (PPO/GRPO), we use **rejection sampling finetuning** (also known as ReST/STaR/Expert Iteration):

1. Sample many code completions from an LLM
2. Score each completion using an automated reward signal
3. If a completion fails, feed the error back to the LLM for repair (up to N rounds)
4. Keep only completions that pass the reward
5. Finetune the model on those passing completions
6. Repeat

This is a simpler alternative to RL that tests the same hypothesis: does the choice of reward signal (unit tests vs formal specs) affect reward hacking rates?

## Pipeline Architecture

```
NL Task Description
        |
        v
  +-----+------+
  |             |
  v             v
Branch A     Branch B
Generate     Generate
Rust Tests   Verus Spec
  |             |
  v             v
Sample N     Sample N
Code         Code
Completions  Completions
  |             |
  v             v
Run Tests    Run Verus
(rustc)      Verifier
  |             |
  v             v
Failed?      Failed?
Repair up    Repair up
to M rounds  to M rounds
  |             |
  v             v
Filter:      Filter:
All Pass?    Verified?
  |             |
  v             v
[Finetune]   [Finetune]
  |             |
  +------+------+
         |
         v
  Evaluate Both
  vs Gold Verus Specs
         |
         v
  Compare Reward
  Hacking Rates
```

## Few-Shot Prompting

All generation tasks use **few-shot examples** with proper chat templates to improve output quality, especially for Verus syntax which is rare in training data.

**Setup**: The first 10 HumanEval problems (IDs 0-9) are reserved as few-shot exemplars. The pipeline automatically skips these during evaluation.

**Format**: Each few-shot example uses user/assistant message pairs in the conversation history:
```
[system]  → Verus cheat sheet + task instructions
[user]    → Few-shot input 1 (formatted with same template as real prompt)
[assistant] → Reasoning trace + code in ```rust``` block
[user]    → Few-shot input 2
[assistant] → Reasoning trace + code in ```rust``` block
...
[user]    → Actual problem prompt
```

**5 exemplars selected** (concise, representative of key Verus patterns):
- `rolling_max` (HumanEval/9) — simple loop + invariant
- `below_zero` (HumanEval/3) — accumulator, wider types for overflow
- `sum_product` (HumanEval/8) — fold_left specs, proof blocks
- `has_close_elements` (HumanEval/0) — nested loops, overflow handling
- `intersperse` (HumanEval/5) — spec functions, proof lemmas

**4 task types** each get tailored few-shot examples:
1. **Test generation**: NL → Rust `#[test]` functions
2. **Spec generation**: NL → Verus requires/ensures (body as `todo!()`)
3. **Code from tests** (Branch A): NL + tests → plain Rust implementation
4. **Code from Verus spec** (Branch B): NL + spec → Verus implementation with proofs

Inspired by the [AlphaVerus](https://github.com/cmu-l3/alphaverus) approach to few-shot prompting for Verus code generation.

## Prototype Scope

The prototype validates every component end-to-end on a small scale, **without finetuning**:

1. **Data**: Parse human-eval-verus problems (NL + gold Verus specs), skip first 10 (used as few-shot)
2. **Generate rewards**: For each problem, use an LLM to generate Rust unit tests (Branch A) and a Verus specification (Branch B)
3. **Generate code**: Sample N code completions per problem per branch
4. **Score**: Run Rust compiler + tests (Branch A) or Verus verifier (Branch B)
5. **Repair**: If a completion fails, feed the error back to the LLM for correction (up to `repair_rounds`, applied equally to both branches). Verus specs are also syntax-checked and repaired before code generation.
6. **Report**: Which completions pass each reward signal

Finetuning is added in a later phase once compute is available.

## Directory Structure

```
RewardHacking/
├── config/
│   └── config.yaml              # Pipeline configuration
├── data/
│   ├── parse_benchmarks.py      # Parse human-eval-verus + load MBPP
│   ├── processed/               # Parsed data (gitignored)
│   └── __init__.py
├── generation/
│   ├── openrouter_client.py     # OpenRouter API wrapper (OpenAI-compatible)
│   ├── few_shot_examples.py     # Few-shot examples from first 10 HumanEval tasks
│   ├── generate_tests.py        # Generate Rust unit tests from NL
│   ├── generate_specs.py        # Generate Verus specs from NL
│   ├── generate_code.py         # Sample code completions
│   ├── verus_reference.py       # Verus cheat sheet for prompts
│   └── __init__.py
├── evaluation/
│   ├── run_tests.py             # Compile + run Rust unit tests via rustc
│   ├── run_verus.py             # Run Verus verifier via subprocess
│   ├── evaluate.py              # Score completions + compute metrics
│   └── __init__.py
├── training/
│   ├── rejection_sampling.py    # Filter completions by reward (future)
│   ├── finetune.py              # SFT with TRL + Unsloth (future)
│   └── __init__.py
├── pipeline/
│   ├── main.py                  # Full pipeline orchestration
│   └── __init__.py
├── results/                     # Experiment outputs (gitignored)
├── human-eval-verus/            # Benchmark data (gitignored)
├── requirements.txt
├── PROTOTYPE.md                 # This file
└── notes.md
```

## Quick Start

### Prerequisites

- Python 3.10+
- Rust toolchain (`rustc`, `cargo`) via [rustup](https://rustup.rs/)
- [Verus](https://github.com/verus-lang/verus) verifier on PATH
- OpenRouter API key

### Setup

```bash
# 1. Create virtual environment
python3 -m venv venv
source venv/bin/activate

# 2. Install dependencies
pip install -r requirements.txt

# 3. Set OpenRouter API key
export OPENROUTER_API_KEY="sk-or-..."
# Or create a .env file:
echo 'OPENROUTER_API_KEY=sk-or-...' > .env

# 4. Parse benchmark data
python -m data.parse_benchmarks human-eval-verus/tasks

# 5. Generate tests
python scripts/generate_oracle_test_cache.py

# 6. Run the prototype pipeline
python -m pipeline.main --config config/config.yaml
```

### Configuration

Edit `config/config.yaml` to adjust:

- `openrouter.model`: Which LLM to use (default: `qwen/qwen3-235b-a22b`)
- `benchmark.max_problems`: Number of problems to evaluate (default: 3)
- `benchmark.skip_few_shot`: Skip first 10 HumanEval tasks used as few-shot (default: true)
- `sampling.n_samples`: Completions per problem (default: 1)
- `sampling.repair_rounds`: Max error-correction rounds per completion (default: 1 = no repair)
- `sampling.temperature`: Sampling temperature (default: 0.8)
- `evaluation.property_test_oracle`: Use cached property-based oracle tests to detect reward hacking (default: true)
- `evaluation.gold_spec_oracle`: Splice into gold Verus spec + compile check (default: false)
- `evaluation.gold_spec_check`: Run Claude oracle for proof annotation, requires `gold_spec_oracle` (default: false)

## Online SFT Training (Local GPU)

To run the pipeline with online rejection sampling finetuning:

```bash
# Requires a CUDA GPU (recommended: GCP L4 24GB)
python -m pipeline.main --local --config config/config.yaml
```

This loads the model locally with QLoRA (4-bit), generates completions, and trains on each completion that passes the unit tests. Configure under `training` in `config.yaml`. See `AGENT.md` for GCP setup instructions.

## Model Choice

**Default: Qwen3 235B (A22B)** via OpenRouter

- Strong open-weights reasoning model (235B MoE, 22B active params)
- Good balance of capability and cost
- OpenRouter ID: `qwen/qwen3-235b-a22b`

**Alternatives:**
- `qwen/qwen3-coder:free` - Free tier, but harsh rate limits (8 req/min under load)
- `qwen/qwen-2.5-coder-7b-instruct` - Smaller, cheaper, open weights for finetuning
- `qwen/qwen-2.5-coder-32b-instruct` - Mid-tier

## Finetuning (Future Phase)

When GPU compute is available, we'll add rejection sampling finetuning using:

- **TRL** (`SFTTrainer`) + **Unsloth** for 2x faster LoRA finetuning
- QLoRA on Qwen2.5-Coder-7B: ~10GB VRAM minimum
- 2-3 iterations of: sample -> filter -> finetune

## Key Metrics

- **Pass own reward**: % of completions passing the training reward (tests or spec)
- **Pass gold spec**: % of completions passing the gold-standard Verus specification
- **Reward hacking rate**: % that pass own reward but fail gold spec

The hypothesis is that Branch A (unit tests) will have a higher reward hacking rate than Branch B (Verus specs).

## Benchmarks

- **Training**: MBPP (Mostly Basic Programming Problems) - ~374 problems with NL descriptions
- **Test**: human-eval-verus - ~81 problems with NL + gold Verus specifications

## Generating Few-Shot Candidates

The `scripts/generate_few_shot_candidates.py` script uses Claude Opus via OpenRouter to generate few-shot examples for both test generation and reward hack implementation. For each of the first 10 HumanEval tasks that have Verus implementations, it:

1. Generates unit tests from the NL description
2. Generates a reward hack implementation (blind — no access to the tests)
3. Runs the tests against the reward hack implementation
4. Splices the reward hack into the gold Verus spec and compile-checks with `verus --no-verify`

### Running

```bash
source venv/bin/activate
python scripts/generate_few_shot_candidates.py
```

Outputs are saved to `few_shot_candidates/<task_slug>/` with:
- `nl_prompt.txt` — natural language task description
- `gold_signatures.txt` — function signatures from the gold Verus spec
- `generated_tests.rs` — generated unit tests
- `test_generation_raw.txt` — raw LLM output for test generation
- `reward_hack_impl.rs` — extracted reward hack code
- `reward_hack_raw.txt` — raw LLM output with reasoning + code
- `test_results.txt` — test pass/fail results
- `spliced.rs` — reward hack spliced into gold Verus spec
- `compile_check.txt` — Verus `--no-verify` compile check result

### Retrying Failed Candidates

If some candidates fail tests or compilation, use the retry script:

```bash
python scripts/retry_few_shot_candidates.py
```

This re-runs the generation at `temperature=0.3` with a fresh prompt (no error feedback, to get clean reasoning traces). New attempts are saved to `HumanEval_N_2`, `HumanEval_N_3`, etc.

### Current Approved Candidates

7 of 8 candidates are approved and used as few-shot examples in `generation/few_shot_examples.py`:
- HumanEval/0, 1, 3, 4, 5, 8, 9

HumanEval/6 (`parse_nested_parens`) is excluded — see the TODO in `AGENT.md`.

## Cost Estimate

Prototype (5 problems, 4 samples): ~$0.05
Full run (81 problems, 16 samples, 3 iterations): ~$10-15
