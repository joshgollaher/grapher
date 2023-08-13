#![cfg_attr(not(debug_assertions), windows_subsystem="windows")]

use eframe::{egui, Frame};
use egui::{Context, Vec2};
use egui::plot::{Line, Plot, PlotPoints};
use std::default::Default;
use std::error::Error;
use eframe::glow::POINT;

const POINTS_PER_UNIT: usize = 100;

fn integer_edit_field(ui: &mut egui::Ui, value: &mut i16) -> egui::Response {
    let mut tmp_value = format!("{}", value);
    let res = ui.text_edit_singleline(&mut tmp_value);
    if let Ok(result) = tmp_value.parse() {
        *value = result;
    }
    res
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        vsync: true,
        resizable: true,
        fullscreen: false,
        maximized: false,
        initial_window_size: Some(Vec2::new(1200., 700.)),
        ..Default::default()
    };
    eframe::run_native("Grapher", options, Box::new(|_cc| Box::new(App::new())))
}

struct App {
    equation: String,
    start: i16,
    end: i16
}

impl App {
    fn new() -> App {
        App {
            equation: "".to_owned(),
            start: -10,
            end: 10
        }
    }

    fn points(&mut self) -> Result<PlotPoints, Box<dyn Error>> {

        let expr: meval::Expr = self.equation.parse()?;
        let func = expr.bind("x")?;

        let vals: PlotPoints = ((POINTS_PER_UNIT as i32 * self.start as i32)..(POINTS_PER_UNIT as i32 * self.end as i32)).map(|v| {
            let x = v as f64 * (1f64 / POINTS_PER_UNIT as f64);
            [x, func(x)]
        }).collect();

        Ok(vals)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Grapher");
            ui.horizontal(|ui| {
                ui.label("Equation: y=");
                ui.text_edit_singleline(&mut self.equation);
                ui.label("from x=");
                integer_edit_field(ui, &mut self.start);
                ui.label("to x=");
                integer_edit_field(ui, &mut self.end);
            });

            Plot::new("Plot").view_aspect(2.0).show(ui, |plot_ui| {
                let points = self.points().unwrap_or(PlotPoints::new(vec![]));

                plot_ui.line(Line::new(points));
            });
        });
    }
}
