//! Interaction state machine — zoom, pan, crosshair tracking.
//!
//! Framework-agnostic: accepts raw input events, returns updated viewport state.
//! Consumers (Leptos, SolidJS, vanilla JS) wire DOM events to these methods.

/// Mouse position relative to the chart area.
#[derive(Debug, Clone, Copy, Default)]
pub struct MousePosition {
    /// X coordinate in pixels relative to chart area left edge.
    pub x: f64,
    /// Y coordinate in pixels relative to chart area top edge.
    pub y: f64,
    /// Whether the mouse is inside the chart area.
    pub in_chart: bool,
}

/// Interaction state for a chart.
#[derive(Debug, Clone)]
pub struct ChartInteraction {
    /// Current viewport (start index + count).
    pub viewport: ViewportState,
    /// Current mouse position.
    pub mouse: MousePosition,
    /// Chart area bounds (set once on resize).
    pub area: AreaState,
}

/// Viewport state (zoom + pan).
#[derive(Debug, Clone, Copy)]
pub struct ViewportState {
    /// Index of the first visible data point.
    pub start: usize,
    /// Number of visible data points.
    pub count: usize,
    /// Whether to use logarithmic Y-axis.
    pub log_scale: bool,
}

impl Default for ViewportState {
    fn default() -> Self {
        Self { start: 0, count: 100, log_scale: false }
    }
}

/// Chart area bounds (updated on resize).
#[derive(Debug, Clone, Copy)]
pub struct AreaState {
    /// CSS width of the canvas.
    pub width: f64,
    /// CSS height of the canvas.
    pub height: f64,
    /// Left padding (Y-axis width).
    pub pad_left: f64,
    /// Right padding.
    pub pad_right: f64,
    /// Top padding.
    pub pad_top: f64,
    /// Bottom padding (time axis + volume).
    pub pad_bottom: f64,
    /// Volume pane height.
    pub vol_height: f64,
}

impl Default for AreaState {
    fn default() -> Self {
        Self {
            width: 800.0,
            height: 400.0,
            pad_left: 60.0,
            pad_right: 16.0,
            pad_top: 12.0,
            pad_bottom: 24.0,
            vol_height: 50.0,
        }
    }
}

impl ChartInteraction {
    /// Create a new interaction state with default viewport.
    #[must_use]
    pub fn new() -> Self {
        Self {
            viewport: ViewportState::default(),
            mouse: MousePosition::default(),
            area: AreaState::default(),
        }
    }

    /// Create with custom area state.
    #[must_use]
    pub fn with_area(area: AreaState) -> Self {
        Self { area, ..Self::new() }
    }

    /// Handle scroll wheel event for zooming.
    ///
    /// `delta_y`: positive = zoom out (more candles), negative = zoom in (fewer candles).
    /// `total_data_points`: total number of data points available.
    pub fn on_wheel(&mut self, delta_y: f64, total_data_points: usize) {
        let step = if delta_y > 0.0 { 10 } else { -10 };
        let new_count = (self.viewport.count as i64 + step).max(10) as usize;
        let count = new_count.min(total_data_points.max(1));
        let start =
            (self.viewport.start + self.viewport.count.saturating_sub(count))
                .min(total_data_points.saturating_sub(count));
        self.viewport.start = start;
        self.viewport.count = count;
    }

    /// Handle mouse move event.
    ///
    /// `x`, `y`: mouse position in CSS pixels relative to the canvas element.
    /// `total_data_points`: total number of data points available.
    pub fn on_mouse_move(&mut self, x: f64, y: f64, _total_data_points: usize) {
        let price_area = self.price_area();
        if x >= price_area.x
            && x <= price_area.x + price_area.w
            && y >= price_area.y
            && y <= price_area.y + price_area.h + self.area.vol_height
        {
            self.mouse = MousePosition { x, y, in_chart: true };
        } else {
            self.mouse = MousePosition::default();
        }
    }

    /// Handle mouse leave event.
    pub fn on_mouse_leave(&mut self) {
        self.mouse = MousePosition::default();
    }

    /// Toggle log scale.
    pub fn toggle_log_scale(&mut self) {
        self.viewport.log_scale = !self.viewport.log_scale;
    }

    /// Reset viewport to show all data.
    pub fn reset_viewport(&mut self, total_data_points: usize) {
        self.viewport.start = 0;
        self.viewport.count = total_data_points;
    }

    /// Get the price chart area (excluding volume, padding).
    #[must_use]
    pub fn price_area(&self) -> plycore::ChartArea {
        plycore::ChartArea {
            x: self.area.pad_left,
            y: self.area.pad_top,
            w: self.area.width - self.area.pad_left - self.area.pad_right,
            h: self.area.height - self.area.pad_top - self.area.pad_bottom - self.area.vol_height,
        }
    }

    /// Get the volume chart area.
    #[must_use]
    pub fn vol_area(&self) -> plycore::ChartArea {
        let price = self.price_area();
        plycore::ChartArea {
            x: price.x,
            y: price.y + price.h + 8.0,
            w: price.w,
            h: self.area.vol_height - 8.0,
        }
    }

    /// Get the full chart area (price + volume + padding).
    #[must_use]
    pub fn full_area(&self) -> plycore::ChartArea {
        plycore::ChartArea {
            x: self.area.pad_left,
            y: self.area.pad_top,
            w: self.area.width - self.area.pad_left - self.area.pad_right,
            h: self.area.height - self.area.pad_top - self.area.pad_bottom,
        }
    }

    /// Convert data index to X pixel coordinate.
    #[must_use]
    pub fn index_to_x(&self, index: usize, visible_count: usize) -> f64 {
        let area = self.price_area();
        area.x + (index as f64 + 0.5) * (area.w / visible_count.max(1) as f64)
    }

    /// Convert price to Y pixel coordinate (linear scale).
    #[must_use]
    pub fn price_to_y_linear(&self, price: f64, min_p: f64, max_p: f64) -> f64 {
        let area = self.price_area();
        let range = (max_p - min_p).max(0.0001);
        area.y + area.h * (1.0 - (price - min_p) / range)
    }

    /// Convert price to Y pixel coordinate (log scale).
    #[must_use]
    pub fn price_to_y_log(&self, price: f64, min_p: f64, max_p: f64) -> f64 {
        let area = self.price_area();
        let lp = price.max(0.0001).ln();
        let lmin = min_p.max(0.0001).ln();
        let lmax = max_p.max(0.0001).ln();
        let range = (lmax - lmin).abs().max(0.0001);
        area.y + area.h * (1.0 - (lp - lmin) / range)
    }

    /// Convert price to Y using current scale mode.
    #[must_use]
    pub fn price_to_y(&self, price: f64, min_p: f64, max_p: f64) -> f64 {
        if self.viewport.log_scale {
            self.price_to_y_log(price, min_p, max_p)
        } else {
            self.price_to_y_linear(price, min_p, max_p)
        }
    }

    /// Get the visible candle index from mouse X position.
    #[must_use]
    pub fn candle_index_at_mouse(&self, visible_count: usize) -> Option<usize> {
        if !self.mouse.in_chart || visible_count == 0 {
            return None;
        }
        let area = self.price_area();
        let x = self.mouse.x - area.x;
        let idx = (x / area.w * visible_count as f64).round() as usize;
        Some(idx.min(visible_count.saturating_sub(1)))
    }

    /// Get visible slice of data given total data points.
    #[must_use]
    pub fn visible_range(&self, total: usize) -> (usize, usize) {
        let start = self.viewport.start.min(total.saturating_sub(1));
        let end = (start + self.viewport.count).min(total);
        (start, end)
    }
}

impl Default for ChartInteraction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interaction_new_defaults() {
        let i = ChartInteraction::new();
        assert_eq!(i.viewport.start, 0);
        assert_eq!(i.viewport.count, 100);
        assert!(!i.viewport.log_scale);
        assert!(!i.mouse.in_chart);
    }

    #[test]
    fn on_wheel_zoom_in() {
        let mut i = ChartInteraction::new();
        i.viewport.count = 100;
        i.on_wheel(-10.0, 500);
        assert_eq!(i.viewport.count, 90);
    }

    #[test]
    fn on_wheel_zoom_out() {
        let mut i = ChartInteraction::new();
        i.viewport.count = 100;
        i.on_wheel(10.0, 500);
        assert_eq!(i.viewport.count, 110);
    }

    #[test]
    fn on_wheel_min_count() {
        let mut i = ChartInteraction::new();
        i.viewport.count = 5;
        i.on_wheel(-10.0, 500);
        assert!(i.viewport.count >= 10);
    }

    #[test]
    fn on_wheel_max_count() {
        let mut i = ChartInteraction::new();
        i.viewport.count = 490;
        i.on_wheel(10.0, 500);
        assert_eq!(i.viewport.count, 500);
    }

    #[test]
    fn on_mouse_move_inside() {
        let mut i = ChartInteraction::new();
        i.on_mouse_move(100.0, 100.0, 200);
        assert!(i.mouse.in_chart);
    }

    #[test]
    fn on_mouse_leave() {
        let mut i = ChartInteraction::new();
        i.on_mouse_move(100.0, 100.0, 200);
        i.on_mouse_leave();
        assert!(!i.mouse.in_chart);
    }

    #[test]
    fn toggle_log_scale() {
        let mut i = ChartInteraction::new();
        assert!(!i.viewport.log_scale);
        i.toggle_log_scale();
        assert!(i.viewport.log_scale);
        i.toggle_log_scale();
        assert!(!i.viewport.log_scale);
    }

    #[test]
    fn reset_viewport() {
        let mut i = ChartInteraction::new();
        i.viewport.start = 200;
        i.viewport.count = 50;
        i.reset_viewport(500);
        assert_eq!(i.viewport.start, 0);
        assert_eq!(i.viewport.count, 500);
    }

    #[test]
    fn index_to_x() {
        let i = ChartInteraction::new();
        let x = i.index_to_x(0, 10);
        let area = i.price_area();
        assert!(x >= area.x);
        assert!(x <= area.x + area.w);
    }

    #[test]
    fn price_to_y_linear() {
        let i = ChartInteraction::new();
        let area = i.price_area();
        let y_min = i.price_to_y_linear(0.0, 0.0, 100.0);
        let y_max = i.price_to_y_linear(100.0, 0.0, 100.0);
        assert!(y_min > y_max);
        assert!((y_min - (area.y + area.h)).abs() < 1.0);
        assert!((y_max - area.y).abs() < 1.0);
    }

    #[test]
    fn candle_index_at_mouse_none_when_outside() {
        let i = ChartInteraction::new();
        assert!(i.candle_index_at_mouse(100).is_none());
    }

    #[test]
    fn visible_range() {
        let mut i = ChartInteraction::new();
        i.viewport.start = 10;
        i.viewport.count = 20;
        let (start, end) = i.visible_range(100);
        assert_eq!(start, 10);
        assert_eq!(end, 30);
    }

    #[test]
    fn visible_range_clamps_to_total() {
        let mut i = ChartInteraction::new();
        i.viewport.start = 90;
        i.viewport.count = 20;
        let (start, end) = i.visible_range(100);
        assert_eq!(start, 90);
        assert_eq!(end, 100);
    }

    #[test]
    fn price_area_dimensions() {
        let i = ChartInteraction::new();
        let a = i.price_area();
        assert!(a.w > 0.0);
        assert!(a.h > 0.0);
        assert_eq!(a.x, i.area.pad_left);
        assert_eq!(a.y, i.area.pad_top);
    }

    #[test]
    fn vol_area_adjacent_to_price_area() {
        let i = ChartInteraction::new();
        let p = i.price_area();
        let v = i.vol_area();
        assert!((v.y - (p.y + p.h + 8.0)).abs() < 1.0);
        assert_eq!(v.x, p.x);
        assert_eq!(v.w, p.w);
    }
}
