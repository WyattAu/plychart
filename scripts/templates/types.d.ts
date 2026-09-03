/**
 * Ergonomic TypeScript types for @wyattau/plychart.
 *
 * The WASM API passes data as JSON strings; these interfaces describe the
 * shapes that JSON must have. See plychart.d.ts for the raw wasm-bindgen
 * signatures.
 */

/** OHLCV candle — universal time-series point for candle/line/area/bar charts. */
export interface CandleData {
  /** Unix timestamp in seconds (or x-index for non-time series). */
  time: number;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
}

/** Convenience builder for CandleData arrays from plain values. */
export function candles(values: number[]): CandleData[] {
  return values.map((v) => ({
    time: 0,
    open: 0,
    high: 0,
    low: 0,
    close: v,
    volume: 0,
  }));
}

/** One named series for update_line / update_area / update_scatter_multi. */
export interface Series {
  /** CSS color string, e.g. "#00e5ff". */
  color: string;
  data: CandleData[];
}

/** One named series for update_radar_multi. */
export interface RadarSeries {
  color: string;
  values: number[];
}

/** One named point-set for update_scatter_multi. */
export interface ScatterSeries {
  color: string;
  points: Array<[number, number]>;
}

/** [label, value] pair for update_pie / update_treemap. */
export type LabelValue = [string, number];

/** Row of stacked segments for update_stacked_bar, e.g. [issues, warnings, passed]. */
export type StackedSegments = number[];

/** Theme colors — pass "" to use the built-in dark preset. */
export interface ThemeJson {
  bg: string;
  text: string;
  text_muted: string;
  grid: string;
  up: string;
  down: string;
  accent: string;
  volume: string;
  crosshair: string;
}

/** Tooltip payload returned by get_tooltip_data (parsed from its JSON string). */
export interface TooltipData {
  index: number;
  time: number;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
}

/** Click payload returned by get_click_data (parsed from its JSON string). */
export interface ClickData {
  index: number;
  x: number;
  y: number;
  distance?: number;
}

/** Chart types accepted by get_click_data's chartType argument. */
export type ClickChartType = "pie" | "scatter" | "default";
