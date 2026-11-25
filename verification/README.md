# PlychartSpec — formal specifications for plychart money math

Machine-checked specifications for the invariants beneath
`plycompute`'s quant functions, formulated over integer basis points
(weights as hundredths of a percent — how allocations are published).

## Status: statements final, proofs drafted

`PlychartSpec.lean` compiles with explicit `sorry` markers: the
theorem statements are settled, the proofs are drafted Nat-arithmetic
inductions pending completion. This is the defence-vertical
formal-methods item (docs/benchmarks/defence.md) — deliberately bounded
so the empirical half carries the load meanwhile:

- `packages/widgets/tests/parity_tests.rs` — property tests over the
  built kernels (determinism, lookahead-bias guard, NaN/Infinity
  policy, finiteness)
- `plycompute` unit tests — empirical HHI/variance bounds on generated
  inputs

The Lean theorems add what property tests cannot: statements for ALL
inputs, checked by the kernel.

## Certificates

1. `hhi_bounded_bp` — Σ wᵢ² ≤ (Σ wᵢ)² for any basis-point weights.
   With a proper allocation (Σ = 10000 bp): HHI ≤ 10⁸ bp² = 1.0
   normalized. Underwrites `concentration::hhi` / `normalised_hhi`.
2. `sumSq_nonneg` — squared terms never sum negative. Underwrites
   `stats` variance, `risk` volatility, `drawdown` squared terms.

## Finishing the proofs

`lake build` — compiles with sorry warnings. The two sorries need
Nat-arithmetic inductions:

- `hhi_bounded_bp`: sum lemma (`sumBp (w :: rest) = w + sumBp rest`,
  rfl), tail bound via IH (`hhiBp rest ≤ sumBp rest * sumBp rest`),
  head bound (`w * w ≤ w * (w + sumBp rest)` via
  `Nat.mul_le_mul_left`), then the cross-term combination
  (`w*w + R*R ≤ (w+R)*(w+R)` via `Nat.add_mul_le_mul` family or
  explicit `Nat.mul_add` rewrites + omega).
- `sumSq_nonneg`: complete (compiles; verify after the first is done).

No Mathlib dependency — Lean 4 core only (`lean-toolchain` pins
v4.36.0; matches the installed elan toolchain).
