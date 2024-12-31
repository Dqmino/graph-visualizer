use eframe::egui::{Checkbox, TopBottomPanel};
use eframe::{egui, App};
use egui::{CentralPanel, Context, Painter, Pos2, Rect, Slider};
use meval::Expr;
use std::str::FromStr;

struct GraphVisualizer {
    function: String,
    x_min: f32,
    x_max: f32,
    points: usize,
    show_axes: bool
}

impl Default for GraphVisualizer {
    fn default() -> Self {
        Self {
            function: "x * x".to_string(),
            x_min: -10.0,
            x_max: 10.0,
            points: 100,
            show_axes: false
        }
    }
}

impl App for GraphVisualizer {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.label("Function Graph Visualizer");

            ui.horizontal(|ui| {
                ui.label("f(x) = ");
                ui.text_edit_singleline(&mut self.function);
            });

            ui.add(Slider::new(&mut self.points, 10..=500).text("Points"));
            ui.add(Slider::new(&mut self.x_min, -100.0..=100.0).text("Minimum X value"));
            ui.add(Slider::new(&mut self.x_max, -100.0..=100.0).text("Maximum X value"));
            ui.add(Checkbox::new(&mut self.show_axes, "Show X and Y axes"));
            ui.separator();
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.label("Graph:");
            let graph_rect = ui.available_rect_before_wrap();
            let painter = ui.painter();
            self.draw_graph(&painter, graph_rect);
        });
    }
}

impl GraphVisualizer {
    fn draw_graph(&self, painter: &Painter, rect: Rect) {
        let GraphVisualizer {
            ref function,
            x_min,
            x_max,
            points,
            show_axes: show_axi
        } = *self;

        let eval_function = |x: f64| -> f64 {
            let eval_result = Expr::from_str(function);
            match eval_result {
                Ok(evalFnc) => match evalFnc.bind("x") {
                    Ok(final_eval_fnc) => final_eval_fnc(x),
                    Err(_) => 1.0,
                },
                Err(_) => 1.0,
            }
        };

        let width = rect.width();
        let height = rect.height();

        let to_screen = |x: f32, y: f32| -> Pos2 {
            let x_screen = rect.left() + (x - x_min) / (x_max - x_min) * width;
            let y_screen = rect.bottom() - (y + 1.0) / 2.0 * height;
            Pos2::new(x_screen, y_screen)
        };

        let step = (x_max - x_min) / (points as f32);
        let mut x = x_min;

        if show_axi {
            // performance implications here don't matter
            let x_axis_start = to_screen(x_min, 0.0);
            let x_axis_end = to_screen(x_max, 0.0);
            let y_axis_start = to_screen(0.0, -rect.height());
            let y_axis_end = to_screen(0.0, rect.height());

            painter.line_segment(
                [x_axis_start, x_axis_end],
                (1.0, egui::Color32::GRAY),
            );

            painter.line_segment(
                [y_axis_start, y_axis_end],
                (1.0, egui::Color32::GRAY),
            );
        }

        let mut prev_point = to_screen(x, eval_function(x as f64) as f32);
        for _ in 0..points {
            x += step;
            let y = eval_function(x as f64);
            let current_point = to_screen(x, y as f32);
            painter.line_segment([prev_point, current_point], (1.0, egui::Color32::WHITE));
            prev_point = current_point;
        }
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 768.0])
            .with_resizable(false),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "Function Graph Visualizer",
        options,
        Box::new(|_cc| {
            _cc.egui_ctx.set_zoom_factor(1.5);
            Ok(Box::new(GraphVisualizer::default()))
        }),
    ).unwrap();
}
