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
- **`while` loops MUST have a `decreases` clause**: e.g., `decreases arr.len() - i`.

### Exec Mode vs Spec Mode
| Operation | Exec mode (runtime) | Spec mode (requires/ensures/invariant) |
|-----------|---------------------|----------------------------------------|
| Index a slice | `*slice_index_get(arr, i)` | `arr@[i as int]` or `arr[i]` |
| Index a Vec | `arr[i]` | `arr@[i as int]` |
| Length | `arr.len()` (returns usize) | `arr@.len()` (returns int) |
| Compare collections | Not directly | Use `@` view: `vec@ == seq` |
| Quantifiers | Not allowed | `forall\\|i: int\\| ...`, `exists\\|i: int\\| ...` |
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
