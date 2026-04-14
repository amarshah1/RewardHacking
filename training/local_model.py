"""Local model wrapper for online rejection sampling finetuning.

Loads a causal LM with QLoRA (4-bit quantization + LoRA adapter) and exposes
both generation and single-step training on the same GPU.  Designed as a
drop-in replacement for the OpenRouter generate() path when running locally.

Usage:
    model = LocalModel("Qwen/Qwen3-4B", lora_r=32)
    completions = model.generate(messages, n=1, temperature=0.8)
    model.train_step(messages + [{"role": "assistant", "content": completion}])
    model.save("checkpoints/step_42")
"""

import os
import re
from dataclasses import dataclass, field
from typing import Optional

import torch
from transformers import (
    AutoModelForCausalLM,
    AutoTokenizer,
    BitsAndBytesConfig,
)
from peft import (
    LoraConfig,
    TaskType,
    get_peft_model,
    prepare_model_for_kbit_training,
)


@dataclass
class LocalModelConfig:
    """Configuration for the local model."""
    model_name: str = "Qwen/Qwen3-4B"
    lora_r: int = 32
    lora_alpha: int = 16
    lora_dropout: float = 0.05
    lora_target_modules: list[str] = field(
        default_factory=lambda: ["q_proj", "k_proj", "v_proj", "o_proj",
                                  "gate_proj", "up_proj", "down_proj"]
    )
    learning_rate: float = 2e-4
    max_seq_len: int = 4096
    load_in_4bit: bool = True
    gradient_accumulation_steps: int = 1
    # Resume from a previously saved LoRA adapter
    resume_from: Optional[str] = None


class LocalModel:
    """Wraps a QLoRA model for generation + online training."""

    def __init__(self, config: LocalModelConfig):
        self.config = config
        self.device = "cuda" if torch.cuda.is_available() else "cpu"
        self.step_count = 0

        # --- Tokenizer ---
        self.tokenizer = AutoTokenizer.from_pretrained(
            config.model_name,
            trust_remote_code=True,
            use_fast=True,
        )
        if self.tokenizer.pad_token is None:
            self.tokenizer.pad_token = self.tokenizer.eos_token

        # --- Quantization ---
        bnb_config = None
        if config.load_in_4bit:
            bnb_config = BitsAndBytesConfig(
                load_in_4bit=True,
                bnb_4bit_quant_type="nf4",
                bnb_4bit_compute_dtype=torch.bfloat16,
                bnb_4bit_use_double_quant=True,
            )

        # --- Base model ---
        self.model = AutoModelForCausalLM.from_pretrained(
            config.model_name,
            quantization_config=bnb_config,
            torch_dtype=torch.bfloat16,
            device_map="auto",
            trust_remote_code=True,
            use_cache=False,  # incompatible with gradient checkpointing
        )

        if config.load_in_4bit:
            self.model = prepare_model_for_kbit_training(self.model)

        # --- LoRA ---
        lora_config = LoraConfig(
            task_type=TaskType.CAUSAL_LM,
            r=config.lora_r,
            lora_alpha=config.lora_alpha,
            lora_dropout=config.lora_dropout,
            target_modules=config.lora_target_modules,
            bias="none",
        )
        self.model = get_peft_model(self.model, lora_config)

        # Load existing adapter weights if resuming
        if config.resume_from and os.path.isdir(config.resume_from):
            print(f"  Resuming LoRA adapter from {config.resume_from}")
            self.model.load_adapter(config.resume_from, adapter_name="default")

        self.model.print_trainable_parameters()

        # --- Optimizer ---
        self.optimizer = torch.optim.AdamW(
            self.model.parameters(),
            lr=config.learning_rate,
            weight_decay=0.01,
        )

        # Track training state
        self._grad_accum_count = 0
        self._grad_accum_loss = 0.0

    # ------------------------------------------------------------------
    # Generation
    # ------------------------------------------------------------------

    def generate(
        self,
        messages: list[dict],
        temperature: float = 0.8,
        max_new_tokens: int = 4096,
        n: int = 1,
    ) -> list[str]:
        """Generate completions from chat messages.

        Args:
            messages: List of {"role": ..., "content": ...} dicts
                      (system, user, assistant for few-shot, then final user).
            temperature: Sampling temperature.
            max_new_tokens: Max tokens to generate.
            n: Number of completions.

        Returns:
            List of completion strings (assistant responses).
        """
        self.model.eval()

        # Apply chat template to get input_ids
        text = self.tokenizer.apply_chat_template(
            messages,
            tokenize=False,
            add_generation_prompt=True,
        )
        inputs = self.tokenizer(text, return_tensors="pt").to(self.device)
        input_len = inputs["input_ids"].shape[1]

        completions = []
        with torch.no_grad():
            for _ in range(n):
                outputs = self.model.generate(
                    **inputs,
                    max_new_tokens=max_new_tokens,
                    temperature=temperature if temperature > 0 else None,
                    do_sample=temperature > 0,
                    top_p=0.95 if temperature > 0 else None,
                    pad_token_id=self.tokenizer.pad_token_id,
                )
                # Decode only the generated tokens (not the prompt)
                generated_ids = outputs[0][input_len:]
                text_out = self.tokenizer.decode(generated_ids, skip_special_tokens=True)
                # Strip think tags (Qwen3 reasoning)
                text_out = self._strip_think_tags(text_out)
                completions.append(text_out)

        return completions

    # ------------------------------------------------------------------
    # Training
    # ------------------------------------------------------------------

    def train_step(self, messages: list[dict]) -> float:
        """Run one gradient step on a single chat example.

        Args:
            messages: Full conversation including the assistant response to
                      train on. Format: [{role, content}, ...] ending with
                      an assistant message.

        Returns:
            Loss value for this example.
        """
        self.model.train()

        # Apply chat template — the full conversation including assistant response
        text = self.tokenizer.apply_chat_template(
            messages,
            tokenize=False,
            add_generation_prompt=False,
        )
        encodings = self.tokenizer(
            text,
            return_tensors="pt",
            truncation=True,
            max_length=self.config.max_seq_len,
        ).to(self.device)

        input_ids = encodings["input_ids"]
        attention_mask = encodings["attention_mask"]

        # Build labels: mask everything except the final assistant turn
        labels = input_ids.clone()

        # Find where the assistant response starts by encoding everything
        # up to (but not including) the final assistant message
        messages_without_last = messages[:-1]
        prefix_text = self.tokenizer.apply_chat_template(
            messages_without_last,
            tokenize=False,
            add_generation_prompt=True,  # includes the assistant prompt token
        )
        prefix_ids = self.tokenizer(
            prefix_text,
            return_tensors="pt",
            truncation=True,
            max_length=self.config.max_seq_len,
        )["input_ids"]
        prefix_len = prefix_ids.shape[1]

        # Mask the prefix tokens (set to -100 so they don't contribute to loss)
        labels[:, :prefix_len] = -100

        # Forward pass
        outputs = self.model(
            input_ids=input_ids,
            attention_mask=attention_mask,
            labels=labels,
        )
        loss = outputs.loss

        # Gradient accumulation
        scaled_loss = loss / self.config.gradient_accumulation_steps
        scaled_loss.backward()

        self._grad_accum_count += 1
        self._grad_accum_loss += loss.item()

        if self._grad_accum_count >= self.config.gradient_accumulation_steps:
            torch.nn.utils.clip_grad_norm_(self.model.parameters(), 1.0)
            self.optimizer.step()
            self.optimizer.zero_grad()
            self.step_count += 1
            self._grad_accum_count = 0
            avg_loss = self._grad_accum_loss / self.config.gradient_accumulation_steps
            self._grad_accum_loss = 0.0
            return avg_loss

        return loss.item()

    # ------------------------------------------------------------------
    # Save / Load
    # ------------------------------------------------------------------

    def save(self, output_dir: str):
        """Save the LoRA adapter and tokenizer."""
        os.makedirs(output_dir, exist_ok=True)
        self.model.save_pretrained(output_dir)
        self.tokenizer.save_pretrained(output_dir)
        # Save optimizer state for resuming
        torch.save({
            "optimizer_state_dict": self.optimizer.state_dict(),
            "step_count": self.step_count,
        }, os.path.join(output_dir, "training_state.pt"))
        print(f"  Saved LoRA adapter to {output_dir} (step {self.step_count})")

    # ------------------------------------------------------------------
    # Helpers
    # ------------------------------------------------------------------

    @staticmethod
    def _strip_think_tags(content: str) -> str:
        """Strip <think>...</think> reasoning blocks from Qwen3 output."""
        if content and "<think>" in content:
            content = re.sub(r"<think>.*?</think>", "", content, flags=re.DOTALL).strip()
        return content
