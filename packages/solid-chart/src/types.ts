/** OHLCV candle data point. */
export interface CandleData {
  time: number;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
}

/** Multi-series input for line/area charts. */
export interface MultiSeriesInput {
  color?: string;
  data?: CandleData[];
}

/** Pie/donut chart data: [["label", value], ...] */
export type PieData = [string, number][];

/** Histogram chart data: array of numeric values. */
export type HistogramData = number[];

/** Sparkline chart data: array of numeric values. */
export type SparklineData = number[];

/** Multiline chart data: array of series with optional color. */
export type MultilineData = MultiSeriesInput[];

/** Tooltip info returned by get_tooltip_data. */
export interface TooltipInfo {
  index: number;
  time: number;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
}

/** Click info returned by get_click_data. */
export interface ClickInfo {
  index: number;
  x: number;
  y: number;
}

/** Chart type identifiers. */
export type ChartType =
  | 'candlestick'
  | 'line'
  | 'area'
  | 'bar'
  | 'heatmap'
  | 'scatter'
  | 'gauge'
  | 'radar'
  | 'treemap'
  | 'waterfall'
  | 'orderbook'
  | 'backtest'
  | 'correlation'
  | 'pie'
  | 'histogram'
  | 'sparkline';

/** Chart component props. */
export interface ChartProps {
  id: string;
  type: ChartType;
  data: string;
  width?: number;
  height?: number;
  theme?: string;
}
