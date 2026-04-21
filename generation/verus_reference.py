"""Verus reference guide for LLM prompts.

Distilled from gold human-eval-verus benchmark implementations.
"""

VERUS_CHEAT_SHEET = """
## Verus Reference Guide (IMPORTANT — read before writing Verus code)

### Critical Limitations
- **NO floating point**: Verus does not support f64/f32. Reformulate float problems using integers (i64, i32, u64, etc.).
- **NO HashMap/BTreeMap**: Use Vec or arrays instead.
- **NO string type**: Use &[char] (input) or Vec<char> (output) for character sequences.
- **NO [] indexing in exec mode for slices**: Use `*slice_index_get(slice, idx)` from `vstd::slice::*`. (Vec and array indexing like `arr[i]` works in exec mode.)
- **`int` and `nat` types are ghost-only**: Cannot use `as int` or `as nat` in exec code. Use concrete types (i64, usize, etc.) in exec mode. Only use `as int` inside spec/proof contexts (requires, ensures, invariant, proof blocks).
- **`vstd::math::abs` is spec-only**: Cannot call `abs()` in exec code. Instead, use manual comparison: `if a > b { a - b } else { b - a }`.
- **`spec fn` functions CANNOT be called from exec code**: You can only call `spec fn` inside `requires`, `ensures`, `invariant`, `assert`, and `proof` blocks. In exec (runtime) code, you must re-implement the logic inline. For example, if you have `spec fn xor_char(a: char, b: char) -> char { if a == b { '0' } else { '1' } }`, you CANNOT write `result.push(xor_char(a, b))` — instead write `result.push(if a == b { '0' } else { '1' })`.
- **Vec is NOT Seq**: `Vec<T>` and `Seq<T>` are different types. Spec functions use `Seq<T>`, exec code uses `Vec<T>`. To pass a `Vec<T>` where `Seq<T>` is expected (e.g. in spec contexts like ensures/invariant), use the `@` view operator: `v@` converts `Vec<T>` to `Seq<T>`. For `Vec<Vec<char>>`, `strings@` gives `Seq<Vec<char>>` — to get `Seq<Seq<char>>`, map each inner vec: `strings@[i]@`. Write spec fns to accept the types they'll actually receive in spec contexts.
- **`while` loops MUST have a `decreases` clause**: e.g., `decreases arr.len() - i`.

### Exec Mode vs Spec Mode
| Operation | Exec mode (runtime) | Spec mode (requires/ensures/invariant) |
|-----------|---------------------|----------------------------------------|
| Index a slice | `*slice_index_get(arr, i)` | `arr@[i as int]` or `arr[i]` |
| Index a Vec | `arr[i]` | `arr@[i as int]` |
| Length | `arr.len()` (returns usize) | `arr@.len()` (returns int) |
| Compare collections | Not directly | Use `@` view: `vec@ == seq` |
| Quantifiers | Not allowed | `forall\\|i: int\\| ...`, `exists\\|i: int\\| ...` |
| Call spec fn | NOT allowed — inline the logic | Allowed |
| Types | i32, i64, usize, u32, etc. | int, nat (plus concrete types) |

### Common Imports
```rust
use vstd::prelude::*;           // always required
use vstd::slice::*;             // for slice_index_get (when using &[T] params)
use vstd::math::abs;            // for absolute value (spec-only!)
use vstd::seq_lib::*;           // for .filter(), .contains(), .to_multiset()
```

### Key Rules
- Named return: `-> (result: Type)` so you can refer to `result` in ensures
- `fn main() {}` goes OUTSIDE the `verus! { }` block
- Every loop needs an `invariant` clause
- `while` loops also need `decreases`
- Use `spec fn` for pure mathematical definitions referenced in requires/ensures
- Use `ghost` variables to capture values for use in proof contexts: `let ghost old = arr@;`
- Use `reveal(Seq::filter);` after filter-related loop steps
- Use `assert(seq@.take((i + 1) as int).drop_last() == seq@.take(i as int));` for take-based invariants

### Common Pitfalls (AVOID THESE)
- **Declaring `let` but then `.push()`**: `let result = Vec::new(); result.push(x);` FAILS — you cannot call `.push()` on an immutable variable. Always use `let mut result = Vec::new();` if you intend to `.push()` to it. Same for any `.insert()`, `.remove()`, `.pop()`, or reassignment.
- **Assigning to undeclared variables**: `result = x` fails. Use `let result = x;` (or `let mut result = x;`). Do NOT reassign to a variable you haven't declared with `let`. At the end of a function, do not write `result = x;` — instead just write `x` (implicit return) or `return x;`.
- **Missing `{` on loop body; semicolon after invariant**: Every loop is written as `for i in 0..n invariant <clauses>, { <body> }`. The invariant clauses are comma-separated and must end with a `{` opening the body. NEVER write `invariant 0 <= i <= n;` with a semicolon — that's a parse error. Write `invariant 0 <= i <= n, { ... }` or just `invariant 0 <= i <= n { ... }`.
- **Nested loops need their own `invariant`**: Every loop (including inner loops) needs its own `invariant` block. Do NOT write `for sub_index in 0..n { ... }` — write `for sub_index in 0..n invariant ... { ... }`.
- **Inclusive ranges `..=` are NOT supported in Verus for loops**: `for i in 0..=n` FAILS with "trait bound `RangeInclusive<usize>: ForLoopGhostIteratorNew` is not satisfied". Use exclusive ranges: `for i in 0..n+1` or `for i in 0..(n+1)`. This applies to both inner and outer loops.
- **Using `int`/`nat` in exec contexts**: `0int`, `1int`, `x as int`, `.take(0 as int)`, `.subrange(0, i as int)` with the result stored in an exec variable — ALL fail in exec code. The types `int` and `nat` are ghost-only. In exec code, use `usize`/`i64`/etc. To compute a length/index for exec use, use concrete types directly: `let len: usize = v.len();` — NOT `let len = v@.len() as int;`. Only use `as int` inside spec/proof/invariant/ensures/requires contexts.
- **Indexing `&[T]` slices in spec**: Spec-mode indexing on a slice parameter `s: &[T]` uses `s[i]` (returns T, works in spec only) or `s@[i]` for explicit view. In exec code, use `*slice_index_get(s, idx)`.
- **Calling spec fn from exec**: `let x = my_spec_fn(a);` fails if `my_spec_fn` is a `spec fn`. Inline the logic in exec code.
- **Spec fn call argument count**: When calling a spec fn, pass EXACTLY the number of arguments it declares. Do NOT pass extra arguments like closures unless the spec fn signature explicitly takes a closure parameter. If you need fold-like behavior, use `Seq::fold_left` / `Seq::fold_right` directly, not as arguments to your own spec fn.
- **Forgetting `=~=` for sequence equality**: When proving `result@ == some_seq`, use `assert(result@ =~= some_seq);` — Verus often needs extensional equality hints.
- **`take`/`drop_last` invariant pattern**: After a `push` inside a loop over `0..n`, add `assert(arr@.take((i + 1) as int).drop_last() == arr@.take(i as int));` to help the verifier relate consecutive iterations.
- **Quantifier triggers**: If a `forall` uses an expression like `arr[i]`, ensure triggers exist. Verus may need explicit `#[trigger]` or `#![trigger ...]` annotations.
- **Vec-of-Vec view types**: For `v: Vec<Vec<char>>`, `v@` is `Seq<Vec<char>>` NOT `Seq<Seq<char>>`. Use `v@.map_values(|inner: Vec<char>| inner@)` to get `Seq<Seq<char>>`, or design spec fns to accept `Seq<Vec<char>>` directly.
- **Avoid `.iter()` / `.enumerate()` / iterator chains**: Verus does not support Rust iterators. Use indexed `for i in 0..v.len()` loops instead.

### Complete Working Example
```rust
use vstd::prelude::*;

verus! {

pub open spec fn is_vowel_spec(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
        || c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U'
}

fn is_vowel(c: char) -> (is_vowel: bool)
    ensures
        is_vowel == is_vowel_spec(c),
{
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
        || c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U'
}

fn remove_vowels(str: &[char]) -> (str_without_vowels: Vec<char>)
    ensures
        str_without_vowels@ == str@.filter(|x: char| !is_vowel_spec(x)),
{
    let ghost str_length = str.len();
    let mut str_without_vowels: Vec<char> = Vec::new();
    assert(str@.take(0int).filter(|x: char| !is_vowel_spec(x)) == Seq::<char>::empty());

    for index in 0..str.len()
        invariant
            str_without_vowels@ == str@.take(index as int).filter(|x: char| !is_vowel_spec(x)),
    {
        if !is_vowel(str[index]) {
            str_without_vowels.push(str[index]);
        }
        assert(str@.take((index + 1) as int).drop_last() == str@.take(index as int));
        reveal(Seq::filter);
    }
    assert(str@ == str@.take(str_length as int));
    str_without_vowels
}

} // verus!
fn main() {}
```
This example demonstrates: spec fn + matching exec fn, named return, loop invariant with take/filter, ghost variables, reveal, and assert patterns.
""".strip()
