"""OpenRouter API client using OpenAI-compatible SDK."""

import os
import re
import time
from openai import OpenAI
from dotenv import load_dotenv

load_dotenv()

DEFAULT_MODEL = "qwen/qwen3-235b-a22b"
BASE_URL = "https://openrouter.ai/api/v1"

# Free tier: can drop to 8 req/min under high demand → 8s between requests
FREE_TIER_DELAY = 8.0

# Global timestamp of last API request — shared across all callers
_last_request_time = 0.0


def get_client() -> OpenAI:
    """Create an OpenAI client configured for OpenRouter."""
    api_key = os.environ.get("OPENROUTER_API_KEY")
    if not api_key:
        raise ValueError(
            "OPENROUTER_API_KEY not set. "
            "Set it as an environment variable or in a .env file."
        )
    return OpenAI(base_url=BASE_URL, api_key=api_key)


def _rate_limit_wait(model: str):
    """Sleep if needed to stay under the free-tier rate limit."""
    global _last_request_time
    if not model.endswith(":free"):
        return
    elapsed = time.time() - _last_request_time
    if elapsed < FREE_TIER_DELAY:
        wait = FREE_TIER_DELAY - elapsed
        print(f"    (rate limit: waiting {wait:.1f}s)")
        time.sleep(wait)
    _last_request_time = time.time()


def _strip_think_tags(content: str) -> str:
    """Strip <think>...</think> reasoning blocks from Qwen3 output."""
    if content and "<think>" in content:
        content = re.sub(r"<think>.*?</think>", "", content, flags=re.DOTALL).strip()
    return content


def _generate_single(client, messages, model, temperature, max_tokens, return_raw):
    """Make a single API call with retry logic."""
    _rate_limit_wait(model)
    for attempt in range(3):
        try:
            response = client.chat.completions.create(
                model=model,
                messages=messages,
                temperature=temperature,
                max_tokens=max_tokens,
            )
            break
        except Exception as e:
            if "429" in str(e) and attempt < 2:
                wait = (attempt + 1) * 15
                print(f"    (429 rate limited, retrying in {wait}s...)")
                time.sleep(wait)
                global _last_request_time
                _last_request_time = time.time()
            else:
                raise

    raw_content = response.choices[0].message.content
    content = _strip_think_tags(raw_content)
    if return_raw:
        return (content, raw_content)
    return content


def generate(
    prompt: str,
    system_prompt: str = "",
    model: str = DEFAULT_MODEL,
    temperature: float = 0.8,
    max_tokens: int = 2048,
    n: int = 1,
    few_shot_messages: list[dict] | None = None,
    return_raw: bool = False,
    num_workers: int = 1,
) -> list[str] | list[tuple[str, str]]:
    """Generate completions from OpenRouter.

    Args:
        prompt: User message
        system_prompt: System message for context
        model: OpenRouter model ID
        temperature: Sampling temperature
        max_tokens: Maximum tokens to generate
        n: Number of completions to generate
        few_shot_messages: Optional list of {"role": "user"/"assistant", "content": ...}
            dicts inserted between system prompt and the final user message.
        return_raw: If True, return list of (cleaned, raw) tuples instead of just cleaned strings.
        num_workers: Number of parallel threads for the n samples (1 = sequential).

    Returns:
        List of completion strings, or list of (cleaned, raw) tuples if return_raw=True.
    """
    client = get_client()

    messages = []
    if system_prompt:
        messages.append({"role": "system", "content": system_prompt})
    if few_shot_messages:
        messages.extend(few_shot_messages)
    messages.append({"role": "user", "content": prompt})

    if num_workers <= 1 or n <= 1:
        return [
            _generate_single(client, messages, model, temperature, max_tokens, return_raw)
            for _ in range(n)
        ]

    from concurrent.futures import ThreadPoolExecutor
    def _call(_):
        return _generate_single(client, messages, model, temperature, max_tokens, return_raw)

    with ThreadPoolExecutor(max_workers=num_workers) as pool:
        return list(pool.map(_call, range(n)))


def generate_batch(
    prompts: list[dict],
    model: str = DEFAULT_MODEL,
    temperature: float = 0.8,
    max_tokens: int = 2048,
    num_workers: int = 1,
) -> list[str]:
    """Generate one completion per prompt, optionally in parallel.

    Args:
        prompts: List of dicts with "user", optional "system", optional "few_shot" keys.
        model: OpenRouter model ID.
        temperature: Sampling temperature.
        max_tokens: Max tokens per completion.
        num_workers: Number of parallel threads (1 = sequential).

    Returns:
        List of completion strings, one per prompt, in the same order.
    """
    def _gen_one(idx_prompt):
        idx, p = idx_prompt
        completions = generate(
            prompt=p["user"],
            system_prompt=p.get("system", ""),
            model=model,
            temperature=temperature,
            max_tokens=max_tokens,
            n=1,
            few_shot_messages=p.get("few_shot"),
        )
        return idx, completions[0]

    if num_workers <= 1:
        results = [None] * len(prompts)
        for idx, p in enumerate(prompts):
            _, result = _gen_one((idx, p))
            results[idx] = result
        return results

    from concurrent.futures import ThreadPoolExecutor
    results = [None] * len(prompts)
    with ThreadPoolExecutor(max_workers=num_workers) as executor:
        futures = executor.map(_gen_one, enumerate(prompts))
        for idx, result in futures:
            results[idx] = result
    return results
