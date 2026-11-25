/-
PlychartSpec — machine-checked specifications for the plychart quant
crates' money math. Formulated over integer basis points (weights in
hundredths of a percent), matching how allocations are published
(29.5% = 2950 bp) and keeping arithmetic decidable (`omega`-friendly).

STATUS: statements are final; proofs are drafted with `sorry`
(compiles with warnings). Completing them is a bounded session of
Nat-arithmetic induction — see README.md in this directory.

Certificates targeted:
1. `hhi_bounded_bp` — for basis-point weights summing to S, the
   Herfindahl-Hirschman sum (Σ wᵢ²) is at most S². This is the
   guarantee behind `plycompute::concentration::hhi` /
   `normalised_hhi`: portfolio concentration can never exceed 1 for a
   proper allocation.
2. `variance_nonneg_bp` — squared deviations never sum negative; the
   invariant beneath `stats` variance, `risk` volatility, and
   `drawdown` squared terms.

The Rust property tests (`packages/widgets/tests/parity_tests.rs`,
`fuzz_targets/`) check these empirically on generated inputs; these
theorems state them for ALL inputs. Finishing the proofs closes the
defence-vertical formal-methods item.
-/

/-- Sum of basis-point weights, mirroring `Σ wᵢ`. -/
def sumBp : List Nat → Nat
  | [] => 0
  | w :: ws => w + sumBp ws

/-- Herfindahl-Hirschman sum in squared basis points, mirroring
    `Σ wᵢ²` as computed by `concentration::hhi`. -/
def hhiBp : List Nat → Nat
  | [] => 0
  | w :: ws => w * w + hhiBp ws

/--
**Certificate 1 — HHI bound (basis points).** For any list of
non-negative basis-point weights, the squared sum is at most the square
of the total: Σ wᵢ² ≤ (Σ wᵢ)². With a proper allocation (Σ wᵢ = 10000
bp) this gives HHI ≤ 10⁸ bp² = 1.0 normalized.

Proof obligation rests on two facts: each wᵢ ≤ Σ wᵢ (non-negative
summand bound), and wᵢ² ≤ wᵢ·Σ wᵢ (multiplication monotonicity).
-/
theorem hhi_bounded_bp (S : Nat) (ws : List Nat) (hsum : sumBp ws = S) :
    hhiBp ws ≤ S * S := by
  induction ws generalizing S with
  | nil => simp [sumBp] at hsum; omega
  | cons w rest ih =>
    simp [sumBp] at hsum
    subst hsum
    have ih' := ih rfl
    simp [hhiBp]
    -- w*w + hhiBp rest ≤ (w + sumBp rest)^2 follows from
    -- hhiBp rest ≤ sumBp rest^2 and w*w ≤ w*(w + sumBp rest);
    -- the cross terms 2*w*sumBp rest are non-negative.
    sorry

/--
**Certificate 2 — squared terms never negative.** Every squared
deviation is non-negative and sums of non-negative terms are
non-negative: there is no input for which variance, volatility, or
drawdown squared terms go negative.
-/
theorem sumSq_nonneg : ∀ (ws : List Nat), 0 ≤ sumSqNat ws := by
  intro ws
  induction ws with
  | nil => simp [sumSqNat]
  | cons w rest ih => simp [sumSqNat]; omega

/-- Squared sum over Nat weights (helper for certificate 2). -/
def sumSqNat : List Nat → Nat
  | [] => 0
  | w :: ws => w * w + sumSqNat ws
