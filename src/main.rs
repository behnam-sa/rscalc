#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    // tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<CalcApp>::default()),
    )
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl Op {
    fn apply(self, a: f64, b: f64) -> f64 {
        match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => a / b,
            Op::Mod => a % b,
        }
    }
}

impl ToString for Op {
    fn to_string(&self) -> String {
        match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
            Op::Mod => "%",
        }
        .to_owned()
    }
}

#[derive(Debug)]
struct CalcApp {
    result: Option<f64>,
    op: Option<Op>,
    current: String,
}

impl Default for CalcApp {
    fn default() -> Self {
        Self {
            result: None,
            op: None,
            current: "".into(),
        }
    }
}

impl CalcApp {
    fn do_op(&mut self, op: Op) {
        self.evaluate();
        if self.result.is_some() {
            self.op = Some(op);
        }
    }

    fn add_digit(&mut self, digit: char) {
        if self.op.is_none() {
            self.result = None;
        }

        if self.current.len() > 10 {
            return;
        }

        self.current.push(digit);
    }

    fn add_point(&mut self) {
        if self.op.is_none() {
            self.result = None;
        }

        if !self.current.contains('.') {
            self.current.push('.');
        }
    }

    fn negate(&mut self) {
        if self.op.is_none() {
            self.result = None;
        }

        if self.current.starts_with('-') {
            self.current.remove(0);
        } else {
            self.current.insert(0, '-');
        }
    }

    fn evaluate(&mut self) {
        self.apply_op();
        self.op = None;
        self.current.clear();
    }

    fn apply_op(&mut self) {
        if self.current.is_empty() {
            return;
        }

        let Ok(current) = self.current.parse() else {
            todo!();
        };

        let Some(result) = self.result else {
            if self.op.is_some() {
                unreachable!();
            }

            self.result = Some(current);
            return;
        };

        let Some(op) = self.op else {
            unreachable!();
        };

        self.result = Some(op.apply(result, current));
    }
}

impl eframe::App for CalcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Calculator");

            let result = self.result.map(|v| v.to_string()).unwrap_or_default();
            let op = self.op.map(|op| op.to_string()).unwrap_or_default();

            ui.label(result + " " + op.as_str());
            ui.label(&self.current);

            egui::Grid::new("grid").show(ui, |ui| {
                if ui.button("CE").clicked() {
                    self.current.clear();
                }
                if ui.button("C").clicked() {
                    self.result = None;
                    self.op = None;
                    self.current.clear();
                }
                if ui.button("%").clicked() {
                    self.do_op(Op::Mod)
                }
                if ui.button("÷").clicked() {
                    self.do_op(Op::Div)
                }
                ui.end_row();

                if ui.button("7").clicked() {
                    self.add_digit('7');
                }
                if ui.button("8").clicked() {
                    self.add_digit('8');
                }
                if ui.button("9").clicked() {
                    self.add_digit('9');
                }
                if ui.button("×").clicked() {
                    self.do_op(Op::Mul);
                }
                ui.end_row();

                if ui.button("4").clicked() {
                    self.add_digit('4');
                }
                if ui.button("5").clicked() {
                    self.add_digit('5');
                }
                if ui.button("6").clicked() {
                    self.add_digit('6');
                }
                if ui.button("−").clicked() {
                    self.do_op(Op::Sub);
                }
                ui.end_row();

                if ui.button("1").clicked() {
                    self.add_digit('1');
                }
                if ui.button("2").clicked() {
                    self.add_digit('2');
                }
                if ui.button("3").clicked() {
                    self.add_digit('3');
                }
                if ui.button("+").clicked() {
                    self.do_op(Op::Add)
                }
                ui.end_row();

                if ui.button("+/−").clicked() {
                    self.negate();
                }
                if ui.button("0").clicked() {
                    self.add_digit('0');
                }
                if ui.button(".").clicked() {
                    self.add_point();
                }
                if ui.button("=").clicked() {
                    self.evaluate();

                    if let Some(result) = self.result {
                        self.current = result.to_string();
                        self.result = None;
                    }
                }
                ui.end_row();
            });
        });
    }
}
