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

## Proof completion sketch (certificate 1)

`hhi_bounded_bp` induction over the weight list, S = sumBp ws:

- nil case: closes by `simp`/`omega` (0 ≤ 0 * 0).
- cons case (w, rest, R = sumBp rest):
  - `hR : hhiBp rest ≤ R * R` — IH at S = R (`ih (sumBp rest) rfl`).
  - `hw : w * w ≤ w * (w + R)` — `Nat.mul_le_mul_left w (Nat.le_add_right w R)`.
  - `hR2 : hhiBp rest ≤ R * (w + R)` — trans of hR and
    `Nat.mul_le_mul_left R (Nat.le_add_left R w)`.
  - `hk1 : w * (w + R) = (w + R) * w` — mul_comm.
  - `hR2b : hhiBp rest ≤ (w + R) * R` — trans hR2 + mul_comm.
  - `hSS : (w + R) * w + (w + R) * R = (w + R) * (w + R)` —
    `(Nat.mul_add (w + R) w R).symm`.
  - Close with `omega` over the commuted atoms: goal `b + a ≤ c4 + c5`
    follows from `a ≤ c4` (hw+hk1), `b ≤ c5` (hR2b), `c4 + c5 = c3` (hSS).

Elaboration-order caveat discovered: `Nat.mul_le_mul`'s implicit {n₂ m₂}
unify from whichever argument is elaborated first — state hR2 as a trans
chain instead of a direct two-sided application.

## First convergence attempt (2026-11-25, not converged)

A calc-chain proof of `hhi_bounded_bp` was drafted (IH tail bound, head
square via `Nat.mul_le_mul_left`, tail-square trans, commutation facts,
distributivity `hSS`) and closed with `omega` over the commuted atoms.
It did not converge in core Lean 4.36: the `Nat.mul_le_mul_left`
signature takes `k` explicitly before the inequality, and the implicit
{n m} unification order across chained `have`s fought the elaborator.
The sketch above is the completion map; the empirical half (property
tests + fuzz) carries the enforcement meanwhile.
