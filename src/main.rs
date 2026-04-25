#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    #[expect(clippy::print_stderr)]
    let (_guard, egui_tracing_collector) = match eframe_template::tracing::init_native() {
        Ok(guard) => guard,
        Err(err_msg) => {
            eprintln!("Failed to start tracing: {err_msg:?}");
            std::process::exit(84);
        }
    };

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 786.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(
                    &include_bytes!("../assets/favicon-512x512.png")[..],
                )
                .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| {
            Ok(Box::new(eframe_template::TemplateApp::new(
                cc,
                egui_tracing_collector,
            )))
        }),
    )
}

// When compiling to web using trunk
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;
    let egui_tracing_collector = match eframe_template::tracing::init_wasm() {
        Ok(collector) => collector,
        Err(err_msg) => panic!("Failed to start tracing: {err_msg:?}"),
    };

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window found")
            .document()
            .expect("No document found (No DOM)");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| {
                    Ok(Box::new(eframe_template::TemplateApp::new(
                        cc,
                        egui_tracing_collector,
                    )))
                }),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
