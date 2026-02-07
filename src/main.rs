use eframe::egui;
use diskray::app::DiskRayApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([800.0, 600.0])
            .with_resizable(true)
            .with_title("DiskRay - Disk Space Analyzer"),
        ..Default::default()
    };

    eframe::run_native(
        "DiskRay",
        options,
        Box::new(|cc| {
            // Set dark theme by default
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            
            // Посмотрим все поля Interaction
            cc.egui_ctx.style_mut(|style| {
                println!("Interaction fields: {:?}", style.interaction);
                // Перечислим все поля по отдельности
                println!("interact_radius: {}", style.interaction.interact_radius);
                println!("resize_grab_radius_side: {}", style.interaction.resize_grab_radius_side);
                println!("resize_grab_radius_corner: {}", style.interaction.resize_grab_radius_corner);
                println!("show_tooltips_only_when_still: {}", style.interaction.show_tooltips_only_when_still);
                println!("tooltip_delay: {:?}", style.interaction.tooltip_delay);
                // И т.д. для остальных полей
            });

            Ok(Box::new(DiskRayApp::new()))
        }),
    )
}