// Lean compiler output
// Module: PlychartSpec
// Imports: public import Init public meta import Init
#include <lean/lean.h>
#if defined(__clang__)
#pragma clang diagnostic ignored "-Wunused-parameter"
#pragma clang diagnostic ignored "-Wunused-label"
#elif defined(__GNUC__) && !defined(__CLANG__)
#pragma GCC diagnostic ignored "-Wunused-parameter"
#pragma GCC diagnostic ignored "-Wunused-label"
#pragma GCC diagnostic ignored "-Wunused-but-set-variable"
#endif
#ifdef __cplusplus
extern "C" {
#endif
lean_object* lean_nat_mul(lean_object*, lean_object*);
lean_object* lean_nat_add(lean_object*, lean_object*);
double lean_float_of_nat(lean_object*);
double lean_float_mul(double, double);
double lean_float_add(double, double);
LEAN_EXPORT lean_object* lp_PlychartSpec_sumBp(lean_object*);
LEAN_EXPORT lean_object* lp_PlychartSpec_sumBp___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_PlychartSpec_hhiBp(lean_object*);
LEAN_EXPORT lean_object* lp_PlychartSpec_hhiBp___boxed(lean_object*);
static lean_once_cell_t lp_PlychartSpec_sumSq___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static double lp_PlychartSpec_sumSq___closed__0;
LEAN_EXPORT double lp_PlychartSpec_sumSq(lean_object*);
LEAN_EXPORT lean_object* lp_PlychartSpec_sumSq___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_PlychartSpec_sumBp(lean_object* v_x_1_){
_start:
{
if (lean_obj_tag(v_x_1_) == 0)
{
lean_object* v___x_2_; 
v___x_2_ = lean_unsigned_to_nat(0u);
return v___x_2_;
}
else
{
lean_object* v_head_3_; lean_object* v_tail_4_; lean_object* v___x_5_; lean_object* v___x_6_; 
v_head_3_ = lean_ctor_get(v_x_1_, 0);
v_tail_4_ = lean_ctor_get(v_x_1_, 1);
v___x_5_ = lp_PlychartSpec_sumBp(v_tail_4_);
v___x_6_ = lean_nat_add(v_head_3_, v___x_5_);
lean_dec(v___x_5_);
return v___x_6_;
}
}
}
LEAN_EXPORT lean_object* lp_PlychartSpec_sumBp___boxed(lean_object* v_x_7_){
_start:
{
lean_object* v_res_8_; 
v_res_8_ = lp_PlychartSpec_sumBp(v_x_7_);
lean_dec(v_x_7_);
return v_res_8_;
}
}
LEAN_EXPORT lean_object* lp_PlychartSpec_hhiBp(lean_object* v_x_9_){
_start:
{
if (lean_obj_tag(v_x_9_) == 0)
{
lean_object* v___x_10_; 
v___x_10_ = lean_unsigned_to_nat(0u);
return v___x_10_;
}
else
{
lean_object* v_head_11_; lean_object* v_tail_12_; lean_object* v___x_13_; lean_object* v___x_14_; lean_object* v___x_15_; 
v_head_11_ = lean_ctor_get(v_x_9_, 0);
v_tail_12_ = lean_ctor_get(v_x_9_, 1);
v___x_13_ = lean_nat_mul(v_head_11_, v_head_11_);
v___x_14_ = lp_PlychartSpec_hhiBp(v_tail_12_);
v___x_15_ = lean_nat_add(v___x_13_, v___x_14_);
lean_dec(v___x_14_);
lean_dec(v___x_13_);
return v___x_15_;
}
}
}
LEAN_EXPORT lean_object* lp_PlychartSpec_hhiBp___boxed(lean_object* v_x_16_){
_start:
{
lean_object* v_res_17_; 
v_res_17_ = lp_PlychartSpec_hhiBp(v_x_16_);
lean_dec(v_x_16_);
return v_res_17_;
}
}
static double _init_lp_PlychartSpec_sumSq___closed__0(void){
_start:
{
lean_object* v___x_18_; double v___x_19_; 
v___x_18_ = lean_unsigned_to_nat(0u);
v___x_19_ = lean_float_of_nat(v___x_18_);
return v___x_19_;
}
}
LEAN_EXPORT double lp_PlychartSpec_sumSq(lean_object* v_x_20_){
_start:
{
if (lean_obj_tag(v_x_20_) == 0)
{
double v___x_21_; 
v___x_21_ = lean_float_once(&lp_PlychartSpec_sumSq___closed__0, &lp_PlychartSpec_sumSq___closed__0_once, _init_lp_PlychartSpec_sumSq___closed__0);
return v___x_21_;
}
else
{
lean_object* v_head_22_; lean_object* v_tail_23_; double v___x_24_; double v___x_25_; double v___x_26_; double v___x_27_; double v___x_28_; 
v_head_22_ = lean_ctor_get(v_x_20_, 0);
v_tail_23_ = lean_ctor_get(v_x_20_, 1);
v___x_24_ = lean_unbox_float(v_head_22_);
v___x_25_ = lean_unbox_float(v_head_22_);
v___x_26_ = lean_float_mul(v___x_24_, v___x_25_);
v___x_27_ = lp_PlychartSpec_sumSq(v_tail_23_);
v___x_28_ = lean_float_add(v___x_26_, v___x_27_);
return v___x_28_;
}
}
}
LEAN_EXPORT lean_object* lp_PlychartSpec_sumSq___boxed(lean_object* v_x_29_){
_start:
{
double v_res_30_; lean_object* v_r_31_; 
v_res_30_ = lp_PlychartSpec_sumSq(v_x_29_);
lean_dec(v_x_29_);
v_r_31_ = lean_box_float(v_res_30_);
return v_r_31_;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_Init(uint8_t builtin);
void lean_initialize_runtime_module();
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_PlychartSpec_PlychartSpec(uint8_t builtin) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
lean_initialize_runtime_module();
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
