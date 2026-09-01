// plychart Demo Application
// Showcases all 13 chart types with sample data

const CHART_TYPES = [
    { id: 'candlestick', name: 'Candlestick', fn: 'update_candles', dataKey: 'ohlcv' },
    { id: 'line', name: 'Line', fn: 'update_line', dataKey: 'ohlcv' },
    { id: 'area', name: 'Area', fn: 'update_area', dataKey: 'ohlcv' },
    { id: 'bar', name: 'Bar', fn: 'update_bar', dataKey: 'ohlcv' },
    { id: 'heatmap', name: 'Heatmap', fn: 'update_heatmap', dataKey: 'heatmap' },
    { id: 'scatter', name: 'Scatter', fn: 'update_scatter', dataKey: 'scatter' },
    { id: 'gauge', name: 'Gauge', fn: 'update_gauge', dataKey: 'gauge' },
    { id: 'radar', name: 'Radar', fn: 'update_radar', dataKey: 'radar' },
    { id: 'treemap', name: 'Treemap', fn: 'update_treemap', dataKey: 'treemap' },
    { id: 'waterfall', name: 'Waterfall', fn: 'update_waterfall', dataKey: 'waterfall' },
    { id: 'orderbook', name: 'Order Book', fn: 'update_order_book', dataKey: 'orderBook' },
    { id: 'backtest', name: 'Backtest', fn: 'update_backtest', dataKey: 'backtest' },
    { id: 'correlation', name: 'Correlation', fn: 'update_correlation', dataKey: 'correlation' }
];

let wasmMod = null;

async function init() {
    try {
        // Load WASM module
        const module = await import('./pkg/plychart.js');
        await module.default();
        wasmMod = module;

        // Load sample data
        const response = await fetch('./data.json');
        const data = await response.json();

        // Create chart cards
        const container = document.getElementById('charts');
        CHART_TYPES.forEach(chart => {
            const card = createChartCard(chart, data);
            container.appendChild(card);
        });

        // Initialize all charts
        CHART_TYPES.forEach(chart => {
            try {
                initChart(chart, data);
            } catch (e) {
                console.error(`Failed to init ${chart.name}:`, e);
                updateStatus(chart.id, 'Error: ' + e.message);
            }
        });
    } catch (e) {
        console.error('Failed to load WASM:', e);
        document.getElementById('charts').innerHTML =
            '<p style="color:#f87171; padding:2rem;">Failed to load WASM module. Run build.sh first.</p>';
    }
}

function createChartCard(chart, data) {
    const card = document.createElement('div');
    card.className = 'chart-card';
    card.innerHTML = `
        <h3>${chart.name}</h3>
        <canvas id="chart-${chart.id}" width="400" height="300"></canvas>
        <div class="controls">
            <button onclick="zoomIn('${chart.id}')">Zoom In</button>
            <button onclick="zoomOut('${chart.id}')">Zoom Out</button>
            <button onclick="resetChart('${chart.id}')">Reset</button>
        </div>
        <div class="status" id="status-${chart.id}">Initializing...</div>
    `;
    return card;
}

function initChart(chart, data) {
    const canvasId = `chart-${chart.id}`;

    // Create chart
    wasmMod.create_chart(canvasId, 400, 300);

    // Update with data
    const chartData = data[chart.dataKey];
    if (!chartData) {
        updateStatus(chart.id, 'No data');
        return;
    }

    if (chart.fn === 'update_gauge') {
        wasmMod.update_gauge(canvasId, chartData.value, chartData.max, '#c8a23c');
    } else if (chart.fn === 'update_radar') {
        wasmMod.update_radar(canvasId, JSON.stringify(chartData.values), JSON.stringify(chartData.labels), '#c8a23c');
    } else if (chart.fn === 'update_backtest') {
        wasmMod.update_backtest(canvasId, JSON.stringify(chartData.equity), JSON.stringify(chartData.drawdown));
    } else if (chart.fn === 'update_correlation') {
        wasmMod.update_correlation(canvasId, JSON.stringify(chartData.matrix), JSON.stringify(chartData.labels));
    } else if (chart.fn === 'update_order_book') {
        wasmMod.update_order_book(canvasId, JSON.stringify(chartData));
    } else if (chart.fn === 'update_treemap' || chart.fn === 'update_waterfall') {
        // These expect [[name, value], ...] format
        wasmMod[chart.fn](canvasId, JSON.stringify(chartData));
    } else if (chart.fn === 'update_heatmap') {
        wasmMod.update_heatmap(canvasId, JSON.stringify(chartData));
    } else {
        // OHLCV-based charts: convert [ts, o, h, l, c, v] to JSON
        const json = JSON.stringify(chartData);
        wasmMod[chart.fn](canvasId, json);
    }

    updateStatus(chart.id, `Rendered ${chartData.length || Object.keys(chartData).length} points`);
}

function updateStatus(chartId, message) {
    const el = document.getElementById(`status-${chartId}`);
    if (el) el.textContent = message;
}

// Global functions for button controls
window.zoomIn = (chartId) => {
    updateStatus(chartId, 'Zoom in (interaction layer)');
};

window.zoomOut = (chartId) => {
    updateStatus(chartId, 'Zoom out (interaction layer)');
};

window.resetChart = (chartId) => {
    const chart = CHART_TYPES.find(c => c.id === chartId);
    if (chart) {
        initChart(chart, window._demoData || {});
        updateStatus(chartId, 'Reset');
    }
};

// Initialize on load
init();
