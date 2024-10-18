use std::time::Duration; // Needed for fonts

use egui::{Color32, Label, Pos2, RichText, Sense, Vec2};
use egui::{FontFamily, FontId};
use egui_plot::{Bar, BarChart, Line, Plot, PlotPoints};

use crate::data::{ButtonAction, CanvasState, Data};
//use crate::fonts;
use crate::logic::Logic;
use std::thread;

// Loading images as follows adds the image into the binary file
// See: https://docs.rs/egui/latest/egui/macro.include_image.html
const IMG_CAMERA: egui::ImageSource<'_> = egui::include_image!("../assets/pics/googlecamera.png");
const IMG_PROCESS: egui::ImageSource<'_> = egui::include_image!("../assets/pics/googlegrain.png");
const IMG_PLOT: egui::ImageSource<'_> = egui::include_image!("../assets/pics/googleplot.png");

const BUTTON_VERT_SPACING: f32 = 10.;

pub struct TemplateApp {
    // Example stuff:
    label: String,
    value: f32,
    run_once: bool,
    data: Data,
    logic: Logic,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            run_once: false,
            data: Default::default(),
            logic: Default::default(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        crate::fonts::setup_custom_fonts(&cc.egui_ctx);
        cc.egui_ctx.set_pixels_per_point(1.2);
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        /* ========================= Run once ======================== */
        if self.run_once == false {
            self.run_once = true;
            //self.button1 = self.button1.new(); // Initialize
            //self.button2 = self.button2.new(); // Initialize
            self.data = self.data.new();
            //println!("Button = {:?}", self.button1);
        }
        /* =========================================================== */

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_switch(ui);
            });
        });

        /* ======================================================================================= */

        egui::SidePanel::left("left_panel")
            .exact_width(64.)
            .resizable(false) // Stops highlight on hover
            .show(ctx, |ui| {
                ui.spinner();
            });

        /* ======================================================================================= */

        match self.data.canvas.get_canvas_state() {
            // Main canvas
            CanvasState::Main => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    // The central panel the region left after adding TopPanel's and SidePanel's
                    ui.heading("eframe template");
                });
            }

            // Process canvas
            CanvasState::Process => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Process");
                });
            }

            // Plot canvas
            CanvasState::Plot => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Plot");
                });
            }
        }

        // Update the state machine
        self.logic.update(&mut self.data);
    }
}
