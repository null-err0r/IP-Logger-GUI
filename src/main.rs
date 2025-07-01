use eframe::{egui, App, Frame};
use std::collections::HashMap;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::{Duration};
use reqwest;

struct AppState {
    logs: Arc<Mutex<String>>,
    #[allow(dead_code)] 
    last_ip: Arc<Mutex<Option<String>>>,
}

impl AppState {
    fn new() -> Self {
        let logs = Arc::new(Mutex::new(String::new()));
        let last_ip = Arc::new(Mutex::new(None));

        let logs_clone = logs.clone();
        let last_ip_clone = last_ip.clone();

        std::thread::spawn(move || loop {
            if let Ok(new_ip) = fetch_public_ip() {
                let time = get_timestamp();
                let conn_ips = get_connected_ips();
                let mut log = format!(
                    "[{}] Active IP: {}\nConnected IPs:\n{}\n{}\n",
                    time,
                    new_ip,
                    conn_ips,
                    "-".repeat(50)
                );

                let mut last_ip = last_ip_clone.lock().unwrap();
                if let Some(old_ip) = &*last_ip {
                    if old_ip != &new_ip {
                        log.insert_str(
                            0,
                            &format!(
                                "[{}] IP Changed!\nOld: {}\nNew: {}\n{}\n",
                                time,
                                old_ip,
                                new_ip,
                                "-".repeat(50)
                            ),
                        );
                    }
                }
                *last_ip = Some(new_ip);

                let mut logs = logs_clone.lock().unwrap();
                logs.push_str(&log);
            } else {
                let mut logs = logs_clone.lock().unwrap();
                logs.push_str(&format!("[{}] Network error.\n{}\n", get_timestamp(), "-".repeat(50)));
            }
            std::thread::sleep(Duration::from_secs(30));
        });

        AppState {
            logs,
            last_ip,
        }
    }
}

impl App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📝 IP Logger 📝");

            if ui.button("Clear Log").clicked() {
                self.logs.lock().unwrap().clear();
            }

            if ui.button("Save Log").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .set_file_name("log.txt")
                    .save_file()
                {
                    std::fs::write(path, self.logs.lock().unwrap().as_str()).ok();
                }
            }

            egui::ScrollArea::vertical()
                .max_height(500.0)
                .show(ui, |ui| {
                    let log_content = self.logs.lock().unwrap().clone();
                    ui.add_sized([
                        ui.available_width(),
                        480.0
                    ], egui::TextEdit::multiline(&mut log_content.clone())
                        .font(egui::TextStyle::Monospace)
                        .desired_rows(30)
                        .code_editor()
                        .cursor_at_end(true)
                        .interactive(false));
                });
        });
    }
}

fn get_timestamp() -> String {
    let now = chrono::Utc::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn fetch_public_ip() -> Result<String, ()> {
    let res = reqwest::blocking::get("https://api64.ipify.org?format=json")
        .map_err(|_| ())?
        .text()
        .map_err(|_| ())?;
    let json: serde_json::Value = serde_json::from_str(&res).map_err(|_| ())?;
    Ok(json["ip"].as_str().unwrap_or("?").to_string())
}

fn get_connected_ips() -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("netstat").arg("-n").output()
    } else {
        Command::new("netstat").arg("-tn").output()
    };

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let mut map = HashMap::new();
        for line in stdout.lines() {
            if cfg!(target_os = "windows") && line.contains("TCP") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let ip = parts[2].split(':').next().unwrap_or("");
                    *map.entry(ip.to_string()).or_insert(0) += 1;
                }
            } else if !cfg!(target_os = "windows") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 5 {
                    let ip = parts[4].split(':').next().unwrap_or("");
                    *map.entry(ip.to_string()).or_insert(0) += 1;
                }
            }
        }
        let mut pairs: Vec<(String, u32)> = map.into_iter().collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1));
        pairs
            .into_iter()
            .map(|(ip, count)| format!("{} -> {}", count, ip))
            .collect::<Vec<String>>()
            .join("\n")
    } else {
        "[!] Unable to fetch netstat info".to_string()
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "IP Logger GUI",
        options,
        Box::new(|_cc| Box::new(AppState::new())),
    )
}

