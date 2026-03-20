# Reward Hacking Experiment: Prototype

## Research Question

Are formal Verus specifications less susceptible to reward hacking compared to unit tests when used as reward signals for LLM code generation?

## Approach: Rejection Sampling Finetuning

Instead of RL (PPO/GRPO), we use **rejection sampling finetuning** (also known as ReST/STaR/Expert Iteration):

1. Sample many code completions from an LLM
2. Score each completion using an automated reward signal
3. Keep only completions that pass the reward
4. Finetune the model on those passing completions
5. Repeat

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

## Prototype Scope

The prototype validates every component end-to-end on a small scale, **without finetuning**:

1. **Data**: Parse 5 human-eval-verus problems (NL + gold Verus specs)
2. **Generate rewards**: For each problem, use an LLM to generate Rust unit tests (Branch A) and a Verus specification (Branch B)
3. **Generate code**: Sample 4 code completions per problem per branch
4. **Score**: Run Rust compiler + tests (Branch A) or Verus verifier (Branch B)
5. **Report**: Which completions pass each reward signal

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
│   ├── generate_tests.py        # Generate Rust unit tests from NL
│   ├── generate_specs.py        # Generate Verus specs from NL
│   ├── generate_code.py         # Sample code completions
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

# 5. Run the prototype pipeline
python -m pipeline.main --config config/config.yaml
```

### Configuration

Edit `config/config.yaml` to adjust:

- `openrouter.model`: Which LLM to use (default: `qwen/qwen3-coder:free`)
- `benchmark.max_problems`: Number of problems for prototype (default: 5)
- `sampling.n_samples`: Completions per problem (default: 4)
- `sampling.temperature`: Sampling temperature (default: 0.8)

## Model Choice

**Default: Qwen3 Coder 480B (free)** via OpenRouter

- Strongest free coding model on OpenRouter (480B MoE, 35B active params)
- 262K context window, optimized for agentic coding tasks
- Free tier: rate-limited to ~20 req/min, 200 req/day
- OpenRouter ID: `qwen/qwen3-coder:free`

**Paid alternatives (no rate limits):**
- `qwen/qwen3-coder` - Same model, paid ($0.22/M input, $1/M output)
- `qwen/qwen-2.5-coder-7b-instruct` - Smaller, cheaper ($0.03/$0.09), open weights for finetuning
- `qwen/qwen-2.5-coder-32b-instruct` - Mid-tier ($0.66/$1.00)

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

## Cost Estimate

Prototype (5 problems, 4 samples): ~$0.05
Full run (81 problems, 16 samples, 3 iterations): ~$10-15
