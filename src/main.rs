#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use dotenv::dotenv;
use eframe::egui;
use eframe::egui::Widget;
use egui_extras::DatePickerButton;
use reqwest;
use serde_json;

fn get_moon_data(
    key: String,
    loc: String,
    date: String,
) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!(
        "http://api.weatherapi.com/v1/astronomy.json?key={}&q={}&dt={}",
        key, loc, date
    );
    let response = reqwest::blocking::get(&url)?;
    let data: serde_json::Value = response.json()?;

    Ok(data)
}

fn main() -> Result<(), eframe::Error> {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY has to be set in your .env file.");
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(450.0, 500.0)),
        ..Default::default()
    };

    let mut city = "Paris".to_owned();
    let mut selected_date = chrono::Local::today().naive_local();

    let mut moon_phase: String = "".to_string();
    let mut moon_rise: String = "".to_string();
    let mut moon_set: String = "".to_string();
    let mut moon_up_text: String = "".to_string();

    let mut sun_rise: String = "".to_string();
    let mut sun_set: String = "".to_string();
    let mut sun_up_text: String = "".to_string();

    let mut country: String = "".to_string();
    let mut region: String = "".to_string();
    let mut local_time: String = "".to_string();

    let mut error: String = "".to_string();

    let label_background_color = egui::Color32::from_rgb(79, 79, 79);
    let label_background_color2 = egui::Color32::from_rgb(61, 61, 61);

    eframe::run_simple_native("Lunar Insight", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(1.2);
            ui.heading("Lunar Insight");
            ui.horizontal(|ui| {
                let name_label = ui.label("City: ");
                ui.text_edit_singleline(&mut city)
                    .labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                ui.label("Date: ");

                DatePickerButton::new(&mut selected_date)
                    .calendar(true)
                    .ui(ui);
            });
            ui.label("");
            let submit_button = ui.add(egui::Button::new("Get data"));

            ui.label("");
            egui::Grid::new("basic_info")
                .striped(true)
                .with_row_color(move |row_index, _style| {
                    Option::from({
                        if row_index % 2 == 0 {
                            label_background_color
                        } else {
                            label_background_color2
                        }
                    })
                })
                .show(ui, |ui| {
                    ui.label("Country: ");
                    ui.label(format!("{}", country));
                    ui.end_row();

                    ui.label("Region: ");
                    ui.label(format!("{}", region));
                    ui.end_row();

                    ui.label("Local time: ");
                    ui.label(format!("{}", local_time));
                    ui.end_row();
                });

            ui.label("");
            egui::Grid::new("moon_info")
                .striped(true)
                .with_row_color(move |row_index, _style| {
                    Option::from({
                        if row_index % 2 == 0 {
                            label_background_color
                        } else {
                            label_background_color2
                        }
                    })
                })
                .show(ui, |ui| {
                    ui.label("Moon phase:");
                    ui.label(format!("{}", moon_phase));
                    ui.end_row();

                    ui.label("Moon rise");
                    ui.label(format!("{}", moon_rise));
                    ui.end_row();

                    ui.label("Moon set");
                    ui.label(format!("{}", moon_set));
                    ui.end_row();
                    ui.label("Moon visible?");
                    ui.label(format!("{}", moon_up_text));
                    ui.end_row();
                });
            ui.label("");
            egui::Grid::new("sun_info")
                .striped(true)
                .with_row_color(move |row_index, _style| {
                    Option::from({
                        if row_index % 2 == 0 {
                            label_background_color
                        } else {
                            label_background_color2
                        }
                    })
                })
                .show(ui, |ui| {
                    ui.label("Sun rise: :");
                    ui.label(format!("{}", sun_rise));
                    ui.end_row();

                    ui.label("Sun set");
                    ui.label(format!("{}", sun_set));
                    ui.end_row();

                    ui.label("Sun visible?");
                    ui.label(format!("{}", sun_up_text));
                    ui.end_row();
                });

            ui.horizontal(|ui| {
                let error_label = ui.label("Errors: ");
                ui.label(format!("{}", error)).labelled_by(error_label.id);
            });
            if submit_button.clicked() {
                let data = get_moon_data(
                    api_key.clone(),
                    city.clone(),
                    selected_date.to_string().clone(),
                );
                let error_status = data.is_err();
                if error_status {
                    error = data.err().unwrap().to_string()
                } else {
                    if data.as_ref().unwrap()["error"].is_null() {
                        country = data.as_ref().unwrap()["location"]["country"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();

                        region = data.as_ref().unwrap()["location"]["region"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();

                        local_time = data.as_ref().unwrap()["location"]["localtime"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();

                        moon_phase = data.as_ref().unwrap()["astronomy"]["astro"]["moon_phase"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();

                        moon_rise = data.as_ref().unwrap()["astronomy"]["astro"]["moonrise"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();
                        moon_set = data.as_ref().unwrap()["astronomy"]["astro"]["moonset"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();
                        if data.as_ref().unwrap()["astronomy"]["astro"]["is_moon_up"] == 1 {
                            moon_up_text = "Yes".to_string()
                        } else {
                            moon_up_text = "No".to_string()
                        }

                        sun_rise = data.as_ref().unwrap()["astronomy"]["astro"]["sunrise"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();
                        sun_set = data.as_ref().unwrap()["astronomy"]["astro"]["sunset"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();
                        if data.as_ref().unwrap()["astronomy"]["astro"]["is_sun_up"] == 1 {
                            sun_up_text = "Yes".to_string()
                        } else {
                            sun_up_text = "No".to_string()
                        }

                        error = "None".to_string();
                    } else {
                        error = data.as_ref().unwrap()["error"]["message"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();
                    }
                    println!("{}", data.as_ref().unwrap());
                }
            }
        });
    })
}
