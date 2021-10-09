use crate::app::{App, SchemeType, TopMenuTab};
use egui::{vec2, Slider, Ui};
use egui::{CollapsingHeader, ComboBox, Window};

impl App {
    pub fn hues(
        &mut self,
        ctx: &egui::CtxRef,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        if let Some(TopMenuTab::Hues) = self.current_tab {
            let mut is_open = true;
            Window::new("Hues")
                .collapsible(false)
                .scroll(true)
                .open(&mut is_open)
                .show(ctx, |ui| {
                    let color = &self.picker.current_color;
                    let hues = color.hues(self.hues_window.num_of_hues, self.hues_window.hues_step);
                    ui.add(
                        Slider::new(&mut self.hues_window.hues_step, 0.01..=0.1)
                            .clamp_to_range(true)
                            .text("step"),
                    );
                    let max_hues = (0.5 / self.hues_window.hues_step).round() as u8;
                    if self.hues_window.num_of_hues > max_hues {
                        self.hues_window.num_of_hues = max_hues;
                    }
                    ui.add(
                        Slider::new(&mut self.hues_window.num_of_hues, u8::MIN..=max_hues)
                            .clamp_to_range(true)
                            .text("# of hues"),
                    );
                    ui.add(
                        Slider::new(&mut self.hues_window.hue_color_size, 20.0..=200.)
                            .clamp_to_range(true)
                            .text("color size"),
                    );

                    let size = vec2(
                        self.hues_window.hue_color_size,
                        self.hues_window.hue_color_size,
                    );
                    hues.iter().for_each(|hue| {
                        self.color_box_label_side(hue, size, ui, tex_allocator);
                    });
                });

            if !is_open {
                self.current_tab = None;
            }
        }
    }

    pub fn tints(
        &mut self,
        ctx: &egui::CtxRef,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        if let Some(TopMenuTab::Tints) = self.current_tab {
            let mut is_open = true;
            Window::new("Tints")
                .collapsible(false)
                .scroll(true)
                .open(&mut is_open)
                .show(ctx, |ui| {
                    let color = &self.picker.current_color;
                    let tints = color.tints(self.tints_window.num_of_tints);
                    ui.add(
                        Slider::new(&mut self.tints_window.num_of_tints, u8::MIN..=50)
                            .clamp_to_range(true)
                            .text("# of tints"),
                    );
                    ui.add(
                        Slider::new(&mut self.tints_window.tint_color_size, 20.0..=200.)
                            .clamp_to_range(true)
                            .text("color size"),
                    );

                    let size = vec2(
                        self.tints_window.tint_color_size,
                        self.tints_window.tint_color_size,
                    );
                    tints.iter().for_each(|tint| {
                        self.color_box_label_side(tint, size, ui, tex_allocator);
                    });
                });

            if !is_open {
                self.current_tab = None;
            }
        }
    }

    pub fn shades(
        &mut self,
        ctx: &egui::CtxRef,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        if let Some(TopMenuTab::Shades) = self.current_tab {
            let mut is_open = true;
            Window::new("Shades")
                .collapsible(false)
                .scroll(true)
                .open(&mut is_open)
                .show(ctx, |ui| {
                    let color = self.picker.current_color;
                    let shades = color.shades(self.shades_window.num_of_shades);
                    ui.add(
                        Slider::new(&mut self.shades_window.num_of_shades, u8::MIN..=50)
                            .clamp_to_range(true)
                            .text("# of shades"),
                    );
                    ui.add(
                        Slider::new(&mut self.shades_window.shade_color_size, 20.0..=200.)
                            .clamp_to_range(true)
                            .text("color size"),
                    );

                    let size = vec2(
                        self.shades_window.shade_color_size,
                        self.shades_window.shade_color_size,
                    );
                    shades.iter().for_each(|shade| {
                        self.color_box_label_side(shade, size, ui, tex_allocator);
                    });
                });

            if !is_open {
                self.current_tab = None;
            }
        }
    }

    pub fn schemes(
        &mut self,
        ui: &mut Ui,
        tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    ) {
        CollapsingHeader::new("Schemes")
            .default_open(true)
            .show(ui, |ui| {
                let size = vec2(self.picker.scheme_color_size, self.picker.scheme_color_size);
                let double_size = vec2(
                    (self.picker.scheme_color_size + ui.spacing().item_spacing.x) * 2.,
                    self.picker.scheme_color_size,
                );

                macro_rules! cb {
                    ($color:ident, $size:expr, $ui:ident) => {
                        $ui.scope(|mut ui| {
                            self.color_box_label_under(&$color, $size, &mut ui, tex_allocator);
                        });
                    };
                    ($color:ident, $ui:ident) => {
                        cb!($color, size, $ui)
                    };
                }

                let color = self.picker.current_color;
                ComboBox::from_label("Choose a type")
                    .selected_text(self.picker.scheme_type.as_ref())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.picker.scheme_type,
                            SchemeType::Complementary,
                            SchemeType::Complementary.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.picker.scheme_type,
                            SchemeType::Triadic,
                            SchemeType::Triadic.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.picker.scheme_type,
                            SchemeType::Tetradic,
                            SchemeType::Tetradic.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.picker.scheme_type,
                            SchemeType::Analogous,
                            SchemeType::Analogous.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.picker.scheme_type,
                            SchemeType::SplitComplementary,
                            SchemeType::SplitComplementary.as_ref(),
                        );
                        ui.selectable_value(
                            &mut self.picker.scheme_type,
                            SchemeType::Square,
                            SchemeType::Square.as_ref(),
                        );
                    });
                ui.add(
                    Slider::new(&mut self.picker.scheme_color_size, 100.0..=250.)
                        .clamp_to_range(true)
                        .text("color size"),
                );
                match self.picker.scheme_type {
                    SchemeType::Complementary => {
                        let compl = color.complementary();
                        ui.horizontal(|ui| {
                            cb!(color, ui);
                            cb!(compl, ui);
                        });
                    }
                    SchemeType::Triadic => {
                        let tri = color.triadic();
                        ui.vertical(|ui| {
                            let c1 = tri.0;
                            let c2 = tri.1;
                            cb!(color, double_size, ui);
                            ui.horizontal(|ui| {
                                cb!(c1, ui);
                                cb!(c2, ui);
                            });
                        });
                    }
                    SchemeType::Tetradic => {
                        let tetr = color.tetradic();
                        ui.vertical(|ui| {
                            let c1 = &tetr.0;
                            let c2 = &tetr.1;
                            let c3 = &tetr.2;
                            ui.horizontal(|ui| {
                                cb!(color, ui);
                                cb!(c1, ui);
                            });
                            ui.horizontal(|ui| {
                                cb!(c2, ui);
                                cb!(c3, ui);
                            });
                        });
                    }
                    SchemeType::Analogous => {
                        let an = color.analogous();
                        ui.vertical(|ui| {
                            let c1 = an.0;
                            let c2 = an.1;
                            cb!(color, double_size, ui);
                            ui.horizontal(|ui| {
                                cb!(c1, ui);
                                cb!(c2, ui);
                            });
                        });
                    }
                    SchemeType::SplitComplementary => {
                        let sc = color.split_complementary();
                        ui.vertical(|ui| {
                            let c1 = sc.0;
                            let c2 = sc.1;
                            cb!(color, double_size, ui);
                            ui.horizontal(|ui| {
                                cb!(c1, ui);
                                cb!(c2, ui);
                            });
                        });
                    }
                    SchemeType::Square => {
                        let s = color.square();
                        ui.vertical(|ui| {
                            let c1 = s.0;
                            let c2 = s.1;
                            let c3 = s.2;
                            ui.horizontal(|ui| {
                                cb!(color, ui);
                                cb!(c1, ui);
                            });
                            ui.horizontal(|ui| {
                                cb!(c2, ui);
                                cb!(c3, ui);
                            });
                        });
                    }
                }
            });
    }
}
