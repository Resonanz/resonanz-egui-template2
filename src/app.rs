use std::time::Instant;

use egui::{Color32, Frame, Margin, RichText};
use egui::{FontFamily, FontId};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    slider_x: u32,
    slider_y: u32,
    slider_x_prev: u32,
    slider_y_prev: u32,
    time_avg: f32,
    time_max: f32,
    time_min: f32,
    avg_cnt: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            slider_x: 1,
            slider_y: 1,
            slider_x_prev: 1,
            slider_y_prev: 1,
            time_avg: 0.,
            time_max: 0.,
            time_min: std::f32::INFINITY,
            avg_cnt: 0.,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        crate::fonts::setup_custom_fonts(&cc.egui_ctx);
        cc.egui_ctx.set_pixels_per_point(1.);
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_switch(ui);
            });
        });

        /* =============================== LEFT SIDEPANEL ======================================== */

        // let style = ctx.style().visuals.panel_fill; // Get the current style

        egui::SidePanel::left("left_panel")
            .exact_width(180.)
            .frame(Frame {
                inner_margin: Margin {
                    left: 1.,
                    right: 1.,
                    top: 0.,
                    bottom: 1.,
                },
                fill: ctx.style().visuals.panel_fill,
                .. Default::default()
                // outer_margin: Default::default(),
                // rounding: Default::default(),
                // shadow: Default::default(),
                // fill: Default::default(),
                // stroke: Default::default(),
            })
            .resizable(false) // Stops highlight on hover
            .show(ctx, |ui| {
                Frame::none()
                    // .inner_margin(Margin::symmetric(42.0, 0.0))
                    .inner_margin(Margin {
                        left: 10.,
                        right: 0.,
                        top: 0.,
                        bottom: 0.,
                    })
                    // .stroke(Stroke {
                    //     width: 10.,
                    //     color: Color32::LIGHT_RED,
                    // })
                    // .rounding(8.0)
                    // .stroke(Stroke::new(1.0, Color32::GOLD))
                    .show(ui, |ui| {
                        let w = ui.available_width();
                        println!("{w}");
                        //ui.spacing_mut().item_spacing.x = 0.;
                        let w = ui.available_width();
                        println!("{w}");

                        ui.add_space(10.);
                        ui.label("Horizontal");
                        ui.add_space(4.);
                        ui.add(egui::Slider::new(&mut self.slider_x, 1..=30));
                        ui.add_space(10.);
                        ui.label("Vertical");
                        ui.add_space(4.);
                        let slider_response = ui.add(egui::Slider::new(&mut self.slider_y, 1..=20));
                        ui.add_space(10.);

                        let slider_width = slider_response.rect.width();
                        println!("slider_width = {slider_width}");
                    });
                Frame::none()
                    .inner_margin(Margin {
                        left: 0.,
                        right: 0.,
                        top: 0.,
                        bottom: 0.,
                    })
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::BOTTOM), |ui| {
                            let response = ui.add(
                                egui::Image::new(egui::include_image!(
                                    "../assets/pics/WR512x512px.png"
                                ))
                                // egui::Layout::centered_and_justified doesn't allow
                                // the image to align with the bottom (it's broken).
                                // Need to create image with large black border to force centering.
                                .max_width(140.)
                                .max_height(140.),
                            );
                            response.on_hover_text("This is a WHOLE lots of response hover text!!! \n and some more...");
                        });
                    });
            });

        /* ============================== CENTRAL PANEL ========================================== */

        //let size = egui::Vec2 { x: 180., y: 50. };
        let top_down_centered_layout = egui::Layout::top_down(egui::Align::Center);

        // egui::Frame::stroke(
        //     self,
        //     Stroke {
        //         width: 2.0,
        //         color: Color32::LIGHT_RED,
        //     },
        // );

        // The central panel the region left after adding TopPanel's and SidePanel's
        egui::CentralPanel::default().show(ctx, |ui| {

            let w = ui.available_width();
            // Grab the frame inner margin
            let ml = egui::Frame::group(ui.style()).inner_margin.left;
            let mr = egui::Frame::group(ui.style()).inner_margin.right;
            let s = egui::Frame::group(ui.style()).stroke.width;
            let is = ui.spacing().item_spacing.x;
            let iss = ui.style().spacing.item_spacing.x;

            println!("{w} and {ml} and {mr} and {s} and {is} and {iss}");
            // let size = egui::Vec2 {x: (ui.available_width() - 12.)/4. - (3. * 2.), y:50.}; // 12 is correct, but why 12? 2+2+4+4? 6+6?
            // Sorting spacing is super confusing.
            // Consider the following:
            //   x: (ui.available_width() - 12. - ((4-1). * 8.))/4
            // available_width is the width available inside the CentralPanel
            // -- the CentralPanel has a default inner_margin of 8 around all sides
            // -- in this case, 1020 - 8 - 8 = 1004 available
            // -- I think -12 means margin of 3 on the inner and outer of the group thus 3+3+3+3
            // ((4.-1.) * 8.))/4 means we have 4 blocks and 3 gaps between them, each size 8
            let size = egui::Vec2 {x: (ui.available_width() - 12. - ((4.-1.) * 8.))/4., y:50.}; // 12 is correct, but why 12? 2+2+4+4? 6+6?
            println!("size = {size}"); // <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
            ui.group(|ui| {
                //ui.spacing_mut().item_spacing.x = 0.;
                ui.horizontal_top(|ui| {
                    ui.allocate_ui_with_layout(size, top_down_centered_layout, |ui| {
                        ui.label(egui::RichText::new("Counter").size(16.));
                        ui.label(egui::RichText::new("[egui draw count]").size(14.));
                        ui.add_space(10.);
                        ui.label(
                            RichText::new(self.avg_cnt.to_string())
                                .color(Color32::RED)
                                .font(FontId {
                                    size: 40.0,
                                    family: FontFamily::Name("Segment7Standard".into()),
                                }),
                        );
                    });

                    ui.allocate_ui_with_layout(size, top_down_centered_layout, |ui| {
                        ui.label(egui::RichText::new("Computation time").size(16.));
                        ui.label(egui::RichText::new("[average, microseconds]").size(14.));
                        ui.add_space(10.);
                        ui.label(
                            RichText::new(self.time_avg.to_string())
                                .color(Color32::BROWN)
                                .font(FontId {
                                    size: 40.0,
                                    family: FontFamily::Name("Segment7Standard".into()),
                                }),
                        );
                    });

                    ui.allocate_ui_with_layout(size, top_down_centered_layout, |ui| {
                        ui.label(egui::RichText::new("Computation time").size(16.));
                        ui.label(egui::RichText::new("[maximum, microseconds]").size(14.));
                        ui.add_space(10.);
                        ui.label(
                            RichText::new(self.time_max.to_string())
                                .color(Color32::GREEN)
                                .font(FontId {
                                    size: 40.0,
                                    family: FontFamily::Name("Segment7Standard".into()),
                                }),
                        );
                    });

                    ui.allocate_ui_with_layout(size, top_down_centered_layout, |ui| {
                        ui.label(egui::RichText::new("Computation time").size(16.));
                        ui.label(egui::RichText::new("[minimum, microseconds]").size(14.));
                        ui.add_space(10.);
                        ui.label(
                            RichText::new(self.time_min.to_string())
                                .color(Color32::BLUE)
                                .font(FontId {
                                    size: 40.0,
                                    family: FontFamily::Name("Segment7Standard".into()),
                                }),
                        );
                    });
                });
            });

            ui.add_space(10.);

            /* ============================== CENTRAL PANEL ========================================== */

            // Check if either slider has had its value changed
            // If either slider value has changed, reset stuff
            if (self.slider_x != self.slider_x_prev) || (self.slider_y != self.slider_y_prev) {
                // Update stored slider values
                self.slider_x_prev = self.slider_x;
                self.slider_y_prev = self.slider_y;

                // Reset all measurement values
                self.time_avg = 0.;
                self.time_max = 0.;
                self.time_min = std::f32::INFINITY;
                self.avg_cnt = 0.;
            }

            // Start the duration timer to measure spinner draw time
            let start = Instant::now();

            // Draw the grid of spinner(s)
            ui.vertical(|ui| {
                for _ in 0..self.slider_y {
                    ui.horizontal(|ui| {
                        for _ in 0..self.slider_x {
                            ui.spinner();
                        }
                    });
                }
            });

            // Stop the duration timer
            let duration = start.elapsed().as_micros() as f32;

            // Update stored max and min duration values
            if duration < self.time_min {
                self.time_min = duration.round();
            } else if duration > self.time_max {
                self.time_max = duration.round();
            }

            self.time_avg =
                (((self.time_avg * self.avg_cnt) + duration) / (self.avg_cnt + 1.)).round();

            self.avg_cnt += 1.;
        });
    }
}
