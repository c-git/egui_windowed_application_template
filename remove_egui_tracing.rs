#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[dependencies]
anyhow = "1.0.102"
regex = "1.12.3"
version-control-clean-check = "0.1.4"
---

use anyhow::Context as _;
use std::io::Write as _;

fn main() -> anyhow::Result<()> {
    // Check for clean repo
    let mut opts = version_control_clean_check::CheckOptions::new();
    opts.allow_staged = true;
    let check_result = version_control_clean_check::check_version_control(
        std::env::current_dir().context("failed to get current working directory")?,
        &opts,
    );

    if let Err(err_msg) = check_result
        && !confirm_user_wants_to_proceed_anyway(&err_msg)
    {
        // Repo not clean and user didn't approve going ahead anyway
        std::process::exit(1);
    }

    do_replace("Cargo.toml", &[(r#"egui_tracing = "\d+\.\d+\.\d+"\n"#, "")])?;
    do_replace(
        "src/app.rs",
        &[
            (
                r#"egui_tracing_collector: egui_tracing::EventCollector,"#,
                "",
            ),
            (
                r#"let mut result = if let Some\(storage\)"#,
                "if let Some(storage)",
            ),
            (
                r#"(?s)Self::default().+result\.data.+result"#,
                "Self::default()}",
            ),
            (
                r#"(?s)UiPage::ui_menu_page_btn::<pages::UiLogViewer>.+?\);"#,
                "",
            ),
        ],
    )?;
    do_replace(
        "src/consts.rs",
        &[(r#"pub const EVENT_COLLECTOR_MAX_EVENTS.+?;\n"#, "")],
    )?;
    do_replace(
        "src/data_shared.rs",
        &[
            (r#"(?s)ScreenLockInfo,.+?Collector"#, "ScreenLockInfo"),
            (r#"egui_tracing_collector.+?,"#, ""),
        ],
    )?;
    do_replace(
        "src/main.rs",
        &[
            (r#"\(_guard, egui_tracing_collector\)"#, "_guard"),
            (r#"let egui_tracing_collector = "#, ""),
            (r#"Ok\(collector\) => collector,"#, "Ok(()) => {}"),
            (r#"egui_tracing_collector,\n"#, ""),
            (r#"Ok(collector) => collector,"#, "Ok(()) = {},"),
        ],
    )?;
    do_replace(
        "src/pages.rs",
        &[
            (r#"mod log_viewer;\n"#, ""),
            (r#"LogViewer\(UiLogViewer\),"#, ""),
            (r#"UiPage::LogViewer\(\$page\) => \$body,\n"#, ""),
            (r#"(?s)Self::LogViewer\(.+?\).+?}\n"#, ""),
            (r#"log_viewer::UiLogViewer,"#, ""),
        ],
    )?;
    do_replace(
        "src/tracing.rs",
        &[
            (
                r#"anyhow::Result<egui_tracing::EventCollector>"#,
                "anyhow::Result<()>",
            ),
            (
                r#"let egui_tracing_event_collector = get_egui_tracing_event_collector\(\);\n"#,
                "",
            ),
            (r#"\.with\(egui_tracing_event_collector.clone\(\)\)\n"#, ""),
            (r#"(?s)subscriber"\)\?;.+?MAX_EVENTS\)"#, "subscriber\")"),
        ],
    )?;
    do_replace(
        "src/tracing/native_only.rs",
        &[
            (
                r#"(?s)<\(.+?tracing_appender::non_blocking::WorkerGuard.+?egui_tracing::EventCollector,.+?\)"#,
                "<tracing_appender::non_blocking::WorkerGuard",
            ),
            (
                r#"let egui_tracing_event_collector = super::get_egui_tracing_event_collector\(\);\n"#,
                "",
            ),
            (r#"egui_tracing_event_collector.clone\(\),"#, ""),
            (r#"let guard = "#, ""),
            (
                r#"(?s)guard, subscriber\)\?;.+?collector\)\)"#,
                "guard, subscriber)",
            ),
            (
                r#"egui_tracing_event_collector: egui_tracing::EventCollector,"#,
                "",
            ),
            (r#"\.with\(egui_tracing_event_collector\)\n"#, ""),
        ],
    )?;

    std::fs::remove_file("src/pages/log_viewer.rs").context("failed to remove log_viewer.rs")?;

    // Clean up code with rustfmt
    let status = std::process::Command::new("cargo")
        .arg("fmt")
        .status() // Waits for the command to finish
        .expect("failed to execute cargo fmt");

    if !status.success() {
        anyhow::bail!("Cargo fmt exited with an error.");
    }

    println!("Completed");
    Ok(())
}

fn do_replace<P: std::fmt::Debug + AsRef<std::path::Path>>(
    path: P,
    pairs_pattern_replacement: &[(&str, &str)],
) -> anyhow::Result<()> {
    // Load file contents
    let mut contents = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read file contents of: {path:?}"))?;

    // Do replacements
    for &(pattern, replacement) in pairs_pattern_replacement {
        let re = regex::Regex::new(pattern).context("failed to compile regex")?;
        contents = re.replace_all(&contents, replacement).to_string();
    }

    // Save file
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .with_context(|| format!("failed to open file for writing: {path:?}"))?;
    file.write_all(contents.as_bytes())
        .with_context(|| format!("failed to write changes to: {path:?}"))?;
    Ok(())
}

fn confirm_user_wants_to_proceed_anyway(status: &version_control_clean_check::VCSError) -> bool {
    print!("Warning {status}\nProceed anyway? (y/N): ");
    std::io::stdout().flush().unwrap(); // Ensure prompt prints immediately

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() != "y" {
        println!("Aborting.");
        false
    } else {
        true
    }
}
