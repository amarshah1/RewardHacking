"""Verus reference guide for LLM prompts.

Distilled from gold human-eval-verus benchmark implementations.
"""

VERUS_CHEAT_SHEET = """
## Verus Reference Guide (IMPORTANT — read before writing Verus code)

### Critical Limitations
- **NO floating point**: Verus does not support f64/f32. Reformulate float problems using integers (i64, i32, u64, etc.).
- **NO HashMap/BTreeMap**: Use Vec or arrays instead.
- **NO string type**: Use &[char] (input) or Vec<char> (output) for character sequences.
- **NO [] indexing in exec mode**: Use `*slice_index_get(slice, idx)` from `vstd::slice::*` instead.

### Required Structure
```rust
use vstd::prelude::*;
use vstd::slice::*;  // needed for slice_index_get

verus! {

// spec/proof helper functions go here

fn my_function(param: &[i64]) -> (result: Vec<i64>)
    requires
        param.len() > 0,
    ensures
        result@.len() == param@.len(),
{
    // implementation
}

} // verus!
fn main() {}   // OUTSIDE the verus! block
```

### Exec Mode vs Spec Mode
| Operation | Exec mode (runtime) | Spec mode (requires/ensures/invariant) |
|-----------|---------------------|----------------------------------------|
| Index a slice | `*slice_index_get(arr, i)` | `arr@[i as int]` or `arr@.index(i)` |
| Length | `arr.len()` (returns usize) | `arr@.len()` (returns int) |
| Compare collections | Not directly | Use `@` view: `vec@ == seq` |
| Quantifiers | Not allowed | `forall\|i: int\| ...`, `exists\|i: int\| ...` |

### Type Casting Rules
- In spec mode, indices must be `int`: use `i as int`
- In exec mode, indices must be `usize`: use `i as usize`
- Named return: `-> (result: Type)` so you can refer to `result` in ensures

### Loop Invariants (REQUIRED for every loop)
```rust
for i in 0..arr.len()
    invariant
        // maintain all needed properties
        forall|k: int| 0 <= k < i ==> arr@[k] >= 0,
{
    let val = *slice_index_get(arr, i);
    // ...
}
```
- `while` loops also need a `decreases` clause for termination.

### Common Imports
```rust
use vstd::prelude::*;           // always
use vstd::slice::*;             // for slice_index_get
use vstd::math::abs;            // for absolute value
use vstd::seq_lib::*;           // for .filter(), .contains(), .to_multiset()
```

### Spec Functions
```rust
spec fn is_even(n: int) -> bool {
    n % 2 == 0
}
```
Use `spec fn` for pure mathematical definitions referenced in requires/ensures.

### Proof Annotations
- `assert(condition);` — simple assertion
- `assert(cond) by { /* proof */ };` — assertion with proof block
- `proof { /* ... */ }` — proof block in exec code
- `#![trigger arr[i]]` — trigger hint for quantifiers
""".strip()
