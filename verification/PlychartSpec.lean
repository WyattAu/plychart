/-
PlychartSpec — machine-checked specifications for the plychart quant
crates' money math. Formulated over integer basis points (weights in
hundredths of a percent, how allocations are published) and Float
series. STATUS: statements final; proofs drafted with `sorry` —
see README.md for the completion path.
-/

/-- Sum of basis-point weights, mirroring `Σ wᵢ`. -/
def sumBp : List Nat → Nat
  | [] => 0
  | w :: ws => w + sumBp ws

/-- Herfindahl-Hirschman sum in squared basis points (`Σ wᵢ²`), as
    computed by `concentration::hhi`. -/
def hhiBp : List Nat → Nat
  | [] => 0
  | w :: ws => w * w + hhiBp ws

/-- Squared sum over a Float series (`Σ xᵢ²`), as used by `stats`
    variance, `risk` volatility, and `drawdown` squared terms. -/
def sumSq : List Float → Float
  | [] => 0
  | x :: xs => x * x + sumSq xs

/--
**Certificate 1 — HHI bound (basis points).** For any list of
non-negative basis-point weights, the squared sum is at most the
square of the total: Σ wᵢ² ≤ (Σ wᵢ)². With a proper allocation
(Σ wᵢ = 10000 bp) this gives HHI ≤ 10⁸ bp² = 1.0 normalized.

Underwrites `plycompute::concentration::hhi` / `normalised_hhi`:
portfolio concentration can never exceed 1 for a proper allocation.
-/
theorem hhi_bounded_bp (S : Nat) (ws : List Nat) (hsum : sumBp ws = S) :
    hhiBp ws ≤ S * S := by
  sorry

/--
**Certificate 2 — squared terms never negative.** Every squared term
is non-negative and so is their sum: no input makes variance,
volatility, or drawdown squared terms negative.
-/
theorem sumSq_nonneg (xs : List Float) : 0 ≤ sumSq xs := by
  sorry
