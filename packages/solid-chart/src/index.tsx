import { onMount, onCleanup, createEffect, JSX, splitProps } from 'solid-js';
import type { ChartProps, TooltipInfo, ClickInfo } from './types';

export type { ChartProps, TooltipInfo, ClickInfo } from './types';
export type {
  CandleData,
  MultiSeriesInput,
  PieData,
  HistogramData,
  SparklineData,
  MultilineData,
  ChartType,
} from './types';

let wasmModule: any = null;

async function loadWasm() {
  if (wasmModule) return wasmModule;
  const mod = await import('./pkg/plychart.js');
  await mod.default();
  wasmModule = mod;
  return wasmModule;
}

export function Chart(allProps: ChartProps & { ref?: (el: HTMLCanvasElement) => void }): JSX.Element {
  const [props, rest] = splitProps(allProps, ['id', 'type', 'data', 'width', 'height', 'theme', 'ref']);

  let canvasRef: HTMLCanvasElement | undefined;
  const canvasId = () => props.id;
  const width = () => props.width ?? 400;
  const height = () => props.height ?? 300;
  const theme = () => props.theme ?? '{}';

  onMount(async () => {
    const wasm = await loadWasm();
    wasm.create_chart(canvasId(), width(), height());
    renderChart(wasm);
  });

  createEffect(() => {
    // Track data and theme changes
    const _d = props.data;
    const _t = theme();
    if (wasmModule) {
      renderChart(wasmModule);
    }
  });

  onCleanup(async () => {
    if (wasmModule) {
      wasmModule.destroy_chart(canvasId());
    }
  });

  function renderChart(wasm: any) {
    const id = canvasId();
    const t = theme();

    switch (props.type) {
      case 'candlestick':
        wasm.update_candles(id, props.data, t);
        break;
      case 'line':
        wasm.update_line(id, props.data, t);
        break;
      case 'area':
        wasm.update_area(id, props.data, t);
        break;
      case 'bar':
        wasm.update_bar(id, props.data, t);
        break;
      case 'heatmap':
        wasm.update_heatmap(id, props.data, t);
        break;
      case 'scatter':
        wasm.update_scatter(id, props.data, t);
        break;
      case 'gauge':
        try {
          const parsed = JSON.parse(props.data);
          wasm.update_gauge(id, parsed.value, parsed.max, parsed.color ?? '#c8a23c', t);
        } catch { /* ignore parse errors */ }
        break;
      case 'radar':
        try {
          const parsed = JSON.parse(props.data);
          wasm.update_radar(id, JSON.stringify(parsed.values), JSON.stringify(parsed.labels), parsed.color ?? '#c8a23c', t);
        } catch { /* ignore parse errors */ }
        break;
      case 'treemap':
        wasm.update_treemap(id, props.data, t);
        break;
      case 'waterfall':
        wasm.update_waterfall(id, props.data, t);
        break;
      case 'orderbook':
        wasm.update_order_book(id, props.data, t);
        break;
      case 'backtest':
        try {
          const parsed = JSON.parse(props.data);
          wasm.update_backtest(id, JSON.stringify(parsed.equity), JSON.stringify(parsed.drawdown), t);
        } catch { /* ignore parse errors */ }
        break;
      case 'correlation':
        try {
          const parsed = JSON.parse(props.data);
          wasm.update_correlation(id, JSON.stringify(parsed.matrix), JSON.stringify(parsed.labels), t);
        } catch { /* ignore parse errors */ }
        break;
      case 'pie':
        wasm.update_pie(id, props.data, t);
        break;
      case 'histogram':
        try {
          const parsed = JSON.parse(props.data);
          wasm.update_histogram(id, props.data, parsed.binCount ?? 20, t);
        } catch {
          wasm.update_histogram(id, props.data, 20, t);
        }
        break;
      case 'sparkline':
        try {
          const parsed = JSON.parse(props.data);
          wasm.update_sparkline(id, props.data, parsed.color ?? '#c8a23c', t);
        } catch {
          wasm.update_sparkline(id, props.data, '#c8a23c', t);
        }
        break;
    }
  }

  return (
    <canvas
      ref={(el) => {
        canvasRef = el;
        props.ref?.(el);
      }}
      id={canvasId()}
      width={width()}
      height={height()}
    />
  );
}

export async function getTooltipData(canvasId: string, x: number, y: number, dataJson: string): Promise<TooltipInfo | null> {
  const wasm = await loadWasm();
  const result = wasm.get_tooltip_data(canvasId, x, y, dataJson);
  try {
    const parsed = JSON.parse(result);
    return Object.keys(parsed).length > 0 ? parsed : null;
  } catch {
    return null;
  }
}

export async function getClickData(canvasId: string, x: number, y: number, dataLen: number): Promise<ClickInfo | null> {
  const wasm = await loadWasm();
  const result = wasm.get_click_data(canvasId, x, y, dataLen);
  try {
    const parsed = JSON.parse(result);
    return Object.keys(parsed).length > 0 ? parsed : null;
  } catch {
    return null;
  }
}

export async function destroyChart(canvasId: string): Promise<void> {
  const wasm = await loadWasm();
  wasm.destroy_chart(canvasId);
}
