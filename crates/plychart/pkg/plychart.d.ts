/* tslint:disable */
/* eslint-disable */
/**
 * Update chart with OHLCV candle data (candlestick, line, area, bar).
 */
export function update_candles(canvas_id: string, data_json: string, theme_json: string): void;
/**
 * Create a chart bound to a canvas element.
 */
export function create_chart(canvas_id: string, width: number, height: number): void;
/**
 * Update chart with backtest equity + drawdown data.
 */
export function update_backtest(canvas_id: string, equity_json: string, drawdown_json: string, theme_json: string): void;
/**
 * Update chart with sparkline data.
 * data_json: `[value1, value2, ...]`
 */
export function update_sparkline(canvas_id: string, data_json: string, color: string, theme_json: string): void;
/**
 * Get click data for a mouse position.
 * Returns JSON: `{index, x, y}` with the nearest data index.
 */
export function get_click_data(canvas_id: string, x: number, y: number, data_len: number): string;
/**
 * Destroy a chart and clean up resources.
 */
export function destroy_chart(canvas_id: string): void;
/**
 * Update chart with histogram data.
 * data_json: `[value1, value2, ...]`
 */
export function update_histogram(canvas_id: string, data_json: string, bin_count: number, theme_json: string): void;
/**
 * Update chart with multi-series radar data.
 * data_json: `[{color: "#ff0000", values: [v1, v2, ...]}, ...]`
 */
export function update_radar_multi(canvas_id: string, data_json: string, labels_json: string, theme_json: string): void;
/**
 * Get tooltip data for a mouse position over a candlestick/line/area/bar chart.
 * Returns JSON: `{index, time, open, high, low, close, volume}` or `{}` if no data.
 */
export function get_tooltip_data(canvas_id: string, x: number, y: number, data_json: string): string;
/**
 * Update chart with area data.
 * Single series: `[{time,open,high,low,close,volume}]`
 * Multi series: `[{color:"#ff0000", data:[{time,open,high,low,close,volume}]}, ...]`
 */
export function update_area(canvas_id: string, data_json: string, theme_json: string): void;
/**
 * Update chart with multi-series scatter data.
 * data_json: `[{color: "#ff0000", points: [[x,y], [x,y], ...]}, ...]`
 */
export function update_scatter_multi(canvas_id: string, data_json: string, theme_json: string): void;
/**
 * Update chart with waterfall data.
 */
export function update_waterfall(canvas_id: string, data_json: string, theme_json: string): void;
/**
 * Update chart with bar data.
 */
export function update_bar(canvas_id: string, data_json: string, theme_json: string): void;
/**
 * Update chart with pie/donut data.
 * data_json: `[["label", value], ...]`
 */
export function update_pie(canvas_id: string, data_json: string, theme_json: string): void;
/**
 * Update chart with line data.
 * Single series: `[{time,open,high,low,close,volume}]`
 * Multi series: `[{color:"#ff0000", data:[{time,open,high,low,close,volume}]}, ...]`
 */
export function update_line(canvas_id: string, data_json: string, theme_json: string): void;
/**
 * Update chart with scatter data.
 */
export function update_scatter(canvas_id: string, data_json: string, theme_json: string): void;
/**
 * Update chart with heatmap matrix data.
 */
export function update_heatmap(canvas_id: string, data_json: string, theme_json: string): void;
/**
 * Update chart with gauge value.
 */
export function update_gauge(canvas_id: string, value: number, max: number, color: string, theme_json: string): void;
/**
 * Update chart with order book data.
 */
export function update_order_book(canvas_id: string, data_json: string, theme_json: string): void;
/**
 * Update chart with treemap data.
 */
export function update_treemap(canvas_id: string, data_json: string, theme_json: string): void;
/**
 * Update chart with radar data.
 */
export function update_radar(canvas_id: string, values_json: string, labels_json: string, color: string, theme_json: string): void;
/**
 * Update chart with correlation matrix.
 */
export function update_correlation(canvas_id: string, matrix_json: string, labels_json: string, theme_json: string): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly create_chart: (a: number, b: number, c: number, d: number) => [number, number];
  readonly destroy_chart: (a: number, b: number) => [number, number];
  readonly get_click_data: (a: number, b: number, c: number, d: number, e: number) => [number, number, number, number];
  readonly get_tooltip_data: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number, number, number];
  readonly update_area: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly update_backtest: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number];
  readonly update_bar: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly update_candles: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly update_correlation: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number];
  readonly update_gauge: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number];
  readonly update_heatmap: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly update_histogram: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number];
  readonly update_line: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly update_order_book: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly update_pie: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly update_radar: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => [number, number];
  readonly update_radar_multi: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number];
  readonly update_scatter: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly update_scatter_multi: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly update_sparkline: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number];
  readonly update_treemap: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly update_waterfall: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
