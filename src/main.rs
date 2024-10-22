use std::io::Write;

use clap::Parser;
use dotenv::dotenv;

use libtad_rs::models::astronomy;
use libtad_rs::models::time;
use libtad_rs::ServiceClient;

use serde::Serialize;
//use serde_json::json;
//
#[derive(Serialize)]
struct AstroInfo {
    country: String,
    state: String,
    city: String,
    lat: f32,
    lon: f32,
    lat_dir: String,
    lon_dir: String,
}

//#[derive(Serialize)]
//struct AstroDay {
//    date: time::DateTime,
//    sunrise: time::DateTime,
//    sunset: time::DateTime,
//    meridian: String,
//    antimeridian: String,
//}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// API key
    #[arg(short, long, env = "API_KEY", value_name = "API-KEY")]
    api: String,

    /// Secret key
    #[arg(short, long, env = "SECRET_KEY", value_name = "SECRET-KEY")]
    secret: String,

    /// Location
    #[arg(short, long, env = "LOCATION", value_name = "LOCATION")]
    location: String,

    /// Dump filename
    #[arg(short, long, env = "DUMP", value_name = "DUMP")]
    dump: Option<std::path::PathBuf>,
}

fn main() {
    dotenv().ok();

    let args = Args::parse();

    let api_key = args.api;
    let secret_key = args.secret;

    let client = ServiceClient::new(api_key, secret_key);

    let mut astro_info: Option<AstroInfo> = None;

    let req = libtad_rs::service::astronomy::AstroEventRequest::new()
        .with_object(astronomy::AstronomyObjectType::Sun)
        .with_placeid(args.location)
        //.set_startdt(libtad_rs::models::time::DateTime::from(
        //    "2024-10-17T00:00:00",
        //))
        //.set_enddt(libtad_rs::models::time::DateTime::from(
        //    "2024-10-19T23:59:59",
        //));
        .set_startdt(time::DateTime {
            year: 2024,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
        })
        .set_enddt(time::DateTime {
            year: 2024,
            month: 2,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
        })
        .with_type(astronomy::AstronomyEventClass::Meridian)
        .with_type(astronomy::AstronomyEventClass::SetRise)
        .set_lang("en");

    let res = client.get_astro_events(&req);

    if let Some(dump_path) = args.dump {
        match &res {
            Ok(astro_events) => {
                let mut dump_file = std::fs::File::create(dump_path).unwrap();
                let fmted = format!("{:?}", astro_events);
                dump_file.write(fmted.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    match &res {
        Ok(astro_events) => {
            astro_events.into_iter().for_each(|event| {
                for location in event.locations.iter() {
                    if astro_info.is_none() {
                        let country = &location.geo.country.name;
                        let state = location.geo.state.as_deref().unwrap_or_default();
                        let city = &location.geo.name;
                        let lat_lon: [f32; 2] = [
                            location.geo.latitude.unwrap_or_default(),
                            location.geo.longitude.unwrap_or_default(),
                        ];

                        //println!(
                        //    "location: {}, {}, {}, {}{} {}{}",
                        //    country,
                        //    state,
                        //    city,
                        //    lat_lon[0],
                        //    if lat_lon[0] > 0.0 { "N" } else { "S" },
                        //    lat_lon[1],
                        //    if lat_lon[1] > 0.0 { "E" } else { "W" }
                        //);

                        astro_info = Some(AstroInfo {
                            country: country.clone(),
                            state: state.to_owned(),
                            city: city.clone(),
                            lat: lat_lon[0],
                            lon: lat_lon[1],
                            lat_dir: if lat_lon[0] > 0.0 { "N" } else { "S" }.to_string(),
                            lon_dir: if lat_lon[1] > 0.0 { "E" } else { "W" }.to_string(),
                        });
                    }

                    for astro in location.astronomy.objects.iter() {
                        // println!("astro: {:?}", astro);
                        if let Some(current) = astro.current.as_ref() {
                            println!("current: {:?}", current);
                        }

                        if let Some(days) = astro.days.as_ref() {
                            println!("days: {}", days.len());

                            for day in days.iter() {
                                println!("day: {:?}", day);
                                let date = day.date.to_string();

                                let sunrise = day
                                    .events
                                    .iter()
                                    .find(|event| {
                                        let event_type = &event.r#type;
                                        if event_type == "rise" {
                                            true
                                        } else {
                                            false
                                        }
                                    })
                                    .unwrap();

                                let sunset = day
                                    .events
                                    .iter()
                                    .find(|event| {
                                        let event_type = &event.r#type;
                                        if event_type == "set" {
                                            true
                                        } else {
                                            false
                                        }
                                    })
                                    .unwrap();

                                //let meridian = day
                                //    .events
                                //    .iter()
                                //    .find(|event| {
                                //        let event_type = &event.r#type;
                                //        if event_type == "meridian" {
                                //            true
                                //        } else {
                                //            false
                                //        }
                                //    })
                                //    .unwrap();

                                // let antimeridian = day
                                //     .events
                                //     .iter()
                                //     .find(|event| {
                                //         let event_type = &event.r#type;
                                //         if event_type == "antimeridian" {
                                //             true
                                //         } else {
                                //             false
                                //         }
                                //     })
                                //     .unwrap();

                                println!(
                                    "date: {}, sunrise: {}:{}:{}, sunset: {}:{}:{}",
                                    date,
                                    sunrise.hour,
                                    sunrise.min,
                                    sunrise.sec,
                                    sunset.hour,
                                    sunset.min,
                                    sunset.sec
                                );
                            }
                        }
                    }
                }
            });
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
