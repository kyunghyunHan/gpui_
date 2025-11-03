use gpui::{
    canvas, div, fill, outline, point, prelude::*, px, rgb, size, App, Application, BorderStyle,
    Bounds, Context, FontWeight, PathBuilder, Pixels, Render, SharedString, Window, WindowBounds,
    WindowOptions,
};

#[derive(Clone, Copy)]
struct Candle {
    open: f32,
    high: f32,
    low: f32,
    close: f32,
}

#[derive(Clone)]
struct ChartState {
    candles: Vec<Candle>,
    min_low: f32,
    max_high: f32,
}

struct StockChart {
    symbol: SharedString,
    candles: Vec<Candle>,
}

impl StockChart {
    fn new() -> Self {
        let candles = vec![
            Candle {
                open: 128.2,
                high: 131.8,
                low: 126.4,
                close: 130.3,
            },
            Candle {
                open: 130.1,
                high: 132.9,
                low: 129.2,
                close: 132.5,
            },
            Candle {
                open: 132.7,
                high: 134.0,
                low: 131.5,
                close: 131.9,
            },
            Candle {
                open: 131.5,
                high: 132.6,
                low: 129.4,
                close: 130.1,
            },
            Candle {
                open: 129.8,
                high: 130.4,
                low: 127.2,
                close: 128.0,
            },
            Candle {
                open: 128.2,
                high: 129.7,
                low: 125.9,
                close: 126.4,
            },
            Candle {
                open: 126.4,
                high: 127.3,
                low: 124.0,
                close: 125.2,
            },
            Candle {
                open: 125.0,
                high: 128.9,
                low: 124.8,
                close: 128.2,
            },
            Candle {
                open: 128.8,
                high: 131.1,
                low: 128.1,
                close: 130.7,
            },
            Candle {
                open: 130.9,
                high: 134.5,
                low: 130.8,
                close: 134.0,
            },
            Candle {
                open: 134.3,
                high: 136.0,
                low: 133.8,
                close: 135.4,
            },
            Candle {
                open: 135.6,
                high: 137.6,
                low: 135.0,
                close: 136.2,
            },
        ];

        Self {
            symbol: "ACME".into(),
            candles,
        }
    }
}

impl Render for StockChart {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut min_low = f32::MAX;
        let mut max_high = f32::MIN;

        for candle in &self.candles {
            min_low = min_low.min(candle.low);
            max_high = max_high.max(candle.high);
        }

        if !self.candles.is_empty() {
            let padding = (max_high - min_low) * 0.05;
            min_low -= padding;
            max_high += padding;
        } else {
            min_low = 0.0;
            max_high = 1.0;
        }

        let state = ChartState {
            candles: self.candles.clone(),
            min_low,
            max_high,
        };

        let state_for_canvas = state.clone();

        div()
            .bg(rgb(0x111827))
            .size(px(760.0))
            .rounded_lg()
            .shadow_lg()
            .p_6()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .text_color(rgb(0xf8fafc))
                    .text_lg()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child(format!("{} • Sample Candlestick", self.symbol)),
            )
            .child(
                canvas(
                    move |_, _, _| state_for_canvas.clone(),
                    move |bounds, state, window, _| draw_stock_canvas(bounds, state, window),
                )
                .size_full()
                .min_h(px(360.0))
                .bg(rgb(0x0f172a))
                .rounded_md()
                .border_1()
                .border_color(rgb(0x1f2937)),
            )
    }
}

fn draw_stock_canvas(bounds: Bounds<Pixels>, state: ChartState, window: &mut Window) {
    window.paint_quad(fill(bounds.clone(), rgb(0x0f172a)));

    if state.candles.is_empty() {
        return;
    }

    let width: f32 = bounds.size.width.into();
    let height: f32 = bounds.size.height.into();
    let margin = 24.0_f32;
    let chart_width = (width - margin * 2.0).max(1.0);
    let chart_height = (height - margin * 2.0).max(1.0);
    let candle_count = state.candles.len() as f32;
    let step = chart_width / candle_count.max(1.0);
    let body_width = (step * 0.6).max(2.0);

    let min_low = state.min_low;
    let max_high = state.max_high;
    let range = (max_high - min_low).max(0.0001);

    let chart_bounds = Bounds::new(
        point(px(margin), px(margin)),
        size(px(chart_width), px(chart_height)),
    );

    window.paint_quad(fill(chart_bounds.clone(), rgb(0x111c2d)));
    window.paint_quad(outline(
        chart_bounds.clone(),
        rgb(0x1f2937),
        BorderStyle::default(),
    ));

    // Draw horizontal grid lines for reference.
    let grid_steps = 4;
    let grid_color = rgb(0x1f2937);
    for step_index in 0..=grid_steps {
        let fraction = step_index as f32 / grid_steps as f32;
        let y = margin + fraction * chart_height;

        let mut builder = PathBuilder::stroke(px(1.0));
        builder.move_to(point(px(margin), px(y)));
        builder.line_to(point(px(margin + chart_width), px(y)));

        if let Ok(path) = builder.build() {
            window.paint_path(path, grid_color);
        }
    }

    let wick_color = rgb(0xe2e8f0);
    for (index, candle) in state.candles.iter().enumerate() {
        let center_x = margin + (index as f32 + 0.5) * step;
        let high_y = margin + (max_high - candle.high) / range * chart_height;
        let low_y = margin + (max_high - candle.low) / range * chart_height;

        let mut wick = PathBuilder::stroke(px(1.2));
        wick.move_to(point(px(center_x), px(high_y)));
        wick.line_to(point(px(center_x), px(low_y)));

        if let Ok(path) = wick.build() {
            window.paint_path(path, wick_color);
        }

        let open_y = margin + (max_high - candle.open) / range * chart_height;
        let close_y = margin + (max_high - candle.close) / range * chart_height;

        let body_top = open_y.min(close_y);
        let body_bottom = open_y.max(close_y);
        let body_height = (body_bottom - body_top).max(1.0);

        let body_bounds = Bounds::new(
            point(px(center_x - body_width / 2.0), px(body_top)),
            size(px(body_width), px(body_height)),
        );

        let (body_color, border_color) = if candle.close >= candle.open {
            (rgb(0x34d399), rgb(0x10b981))
        } else {
            (rgb(0xf87171), rgb(0xef4444))
        };

        window.paint_quad(fill(body_bounds.clone(), body_color));
        window.paint_quad(outline(
            body_bounds,
            border_color,
            BorderStyle::default(),
        ));
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(780.0), px(520.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| StockChart::new()),
        )
        .unwrap();
    });
}
