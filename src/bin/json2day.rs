use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;

use serde_this_or_that::{as_f64, as_u64};

use plotters::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input JSON file
    #[arg(short, long, env = "INPUT", value_name = "INPUT")]
    input: std::path::PathBuf,

    /// Output image
    #[arg(short, long, env = "OUTPUT", value_name = "OUTPUT")]
    output: std::path::PathBuf,

    /// Transformed output JSON file
    #[arg(short, long, env = "TRANSFORMED", value_name = "TRANSFORMED")]
    transformed: Option<std::path::PathBuf>,

    /// Label
    #[arg(short, long, env = "LABEL", value_name = "LABEL")]
    label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum NumOrStr {
    Str(String),
    #[serde(deserialize_with = "as_f64")]
    Num(f64),
}

impl NumOrStr {
    pub fn as_num(&self) -> Self {
        match self {
            NumOrStr::Str(ref s) => {
                let parts: Vec<&str> = s.split(|c| ":,".contains(c)).collect();
                let h = parts[0].parse::<f64>().unwrap();
                let m = parts[1].parse::<f64>().unwrap();
                NumOrStr::Num(h + m / 60.0)
            }
            _ => self.clone(),
        }
    }

    pub fn as_str(&self) -> Self {
        match self {
            NumOrStr::Num(n) => {
                NumOrStr::Str(format!("{}:{}", (*n as u64), ((n.fract() * 60.0) as u64)))
            }
            _ => self.clone(),
        }
    }

    pub fn get_num(&self) -> f64 {
        match self {
            NumOrStr::Num(n) => *n,
            _ => self.as_num().get_num(),
        }
    }
}

impl std::fmt::Display for NumOrStr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NumOrStr::Str(s) => write!(f, "{}", s),
            NumOrStr::Num(n) => write!(f, "{}:{}", (*n as u64), ((n.fract() * 60.0) as u64)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Day {
    #[serde(deserialize_with = "as_u64")]
    y: u64,
    #[serde(deserialize_with = "as_u64")]
    m: u64,
    #[serde(deserialize_with = "as_u64")]
    d: u64,
    #[serde(deserialize_with = "as_u64")]
    yday: u64,
    srise: Option<NumOrStr>,
    sset: Option<NumOrStr>,
    solnoon: Option<NumOrStr>,
    daylen: Option<NumOrStr>,
}

#[derive(Serialize, Debug)]
struct XDay {
    #[serde(rename(serialize = "Day Number"))]
    yday: u64,
    #[serde(
        rename(serialize = "Sunrise Hour"),
        skip_serializing_if = "Option::is_none"
    )]
    srise: Option<NumOrStr>,
    #[serde(
        rename(serialize = "Sunset Hour"),
        skip_serializing_if = "Option::is_none"
    )]
    sset: Option<NumOrStr>,
    #[serde(
        rename(serialize = "Daylight Length"),
        skip_serializing_if = "Option::is_none"
    )]
    daylen: Option<NumOrStr>,
    #[serde(
        rename(serialize = "Solar Noon Time"),
        skip_serializing_if = "Option::is_none"
    )]
    solnoon: Option<NumOrStr>,
}

fn main() {
    let args = Args::parse();

    let input = args.input;

    let input = fs::read_to_string(&input).expect("Unable to read file");
    let mut data: Vec<Day> = serde_json::from_str(&input).expect("Unable to parse JSON");

    for day in &mut data {
        day.srise = Some(day.srise.as_ref().unwrap().as_num());
        day.sset = Some(day.sset.as_ref().unwrap().as_num());
        day.solnoon = Some(day.solnoon.as_ref().unwrap().as_num());
        day.daylen = match (&day.srise, &day.sset) {
            (Some(NumOrStr::Num(srise)), Some(NumOrStr::Num(sset))) => {
                Some(NumOrStr::Num(sset - srise))
            }
            _ => None,
        };
    }

    //let output = serde_json::to_string_pretty(&data).expect("Unable to serialize JSON");
    //println!("{}", output);

    let root = SVGBackend::new(&args.output, (1024, 768)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(
            args.label.unwrap_or(String::from("Sun Rise/Set/Noon")),
            ("sans-serif", 50).into_font(),
        )
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..365, 0.0..24.0)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(28)
        .y_labels(5)
        .x_desc("Day")
        .y_desc("Time/Duration (h)")
        .draw()
        .unwrap();

    // series
    let mut plot_series = |accessor: fn(&Day) -> &NumOrStr, color: RGBColor, label: &str| {
        chart
            .draw_series(
                LineSeries::new(
                    data.iter()
                        .map(|day| (day.yday as i32, accessor(day).get_num())),
                    &color,
                )
                .point_size(5),
            )
            .unwrap()
            .label(label)
            .legend({
                let color = color;
                move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color)
            });
    };

    plot_series(
        |day| day.srise.as_ref().unwrap(),
        RGBColor(255, 0, 0),
        "Sunrise",
    );

    plot_series(
        |day| day.sset.as_ref().unwrap(),
        RGBColor(0, 255, 0),
        "Sunset",
    );

    plot_series(
        |day| day.solnoon.as_ref().unwrap(),
        RGBColor(0, 0, 255),
        "Solar Noon",
    );

    plot_series(
        |day| &day.daylen.as_ref().unwrap(),
        RGBColor(0, 0, 0),
        "Day Length",
    );

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .position(SeriesLabelPosition::UpperRight)
        .draw()
        .unwrap();

    #[derive(Debug)]
    struct SineFitment {
        period_mul_2pi: f64,
        a: f64,
        b: f64,
        c: f64,
        d: f64,
    }

    impl SineFitment {
        pub fn new() -> Self {
            SineFitment {
                period_mul_2pi: 0.0,
                a: 0.0,
                b: 0.0,
                c: 0.0,
                d: 0.0,
            }
        }
    }

    struct SineFitter {
        min: f64,
        min_day: u64,
        max: f64,
        max_day: u64,
    }

    impl SineFitter {
        fn new() -> Self {
            SineFitter {
                min: f64::MAX,
                min_day: 0,
                max: f64::MIN,
                max_day: 0,
            }
        }

        fn update(&mut self, value: f64, yday: u64) {
            if value < self.min {
                self.min = value;
                self.min_day = yday;
            }
            if value > self.max {
                self.max = value;
                self.max_day = yday;
            }
        }

        fn get_midline(&self) -> f64 {
            (self.min + self.max) / 2.0
        }

        fn fit(&self, data: &Vec<(f64, u64)>, period_mul_2pi: f64) -> SineFitment {
            let mut fitment = SineFitment::new();

            if data.is_empty() {
                return fitment;
            }

            //{
            //    a: 1.0,
            //    b: 2.0 * std::f64::consts::PI / 365.0,
            //    c: 0.0,
            //    d: self.get_midline(),
            //};

            fitment.period_mul_2pi = period_mul_2pi;

            let mag = &mut fitment.a;
            let period = &mut fitment.b;
            let shift = &mut fitment.c;
            let midline = &mut fitment.d;

            *mag = (self.max - self.min) / 2.0;
            *midline = self.get_midline();
            *period = period_mul_2pi * 2.0 * std::f64::consts::PI / 365.0;

            // find ydar closest to midline
            let mut closest = data[0];
            let mut closest_last_pt = data[0];
            let mut closest_deriv = 0.0;
            for d in data {
                let deriv = (d.0 - closest_last_pt.0) / (d.1 - closest_last_pt.1) as f64;
                closest_last_pt = *d;
                if (d.0 - fitment.d).abs() < (closest.0 - fitment.d).abs() {
                    closest = *d;
                    closest_deriv = deriv;
                }
            }

            *shift = closest.1 as f64;
            if (*shift > (182.5)) && (*shift < (365.0)) {
                //*shift = *shift - 365.0;
                //*shift = 365.0 - *shift;
                if closest_deriv > 0.00 {
                    *shift = *shift - 365.0 / (2.0 * period_mul_2pi);
                } else {
                    *shift = 365.0 / (2.0 * period_mul_2pi) - *shift;
                }
            }

            eprintln!(
                "Closest: {:?}, Shift: {}; deriv: {}",
                closest, *shift, closest_deriv
            );

            *shift = *shift * if closest_deriv >= 0.01 { -1.0 } else { 1.0 };

            fitment
        }
    }

    if let Some(transformed) = args.transformed {
        let mut xdata = Vec::new();

        let mut mmsrise = SineFitter::new();
        let mut mmsset = SineFitter::new();
        let mut mmsolnoon = SineFitter::new();
        let mut mmdaylen = SineFitter::new();

        for dtp in &data {
            xdata.push(XDay {
                yday: dtp.yday,
                srise: dtp.srise.clone(),
                sset: dtp.sset.clone(),
                solnoon: dtp.solnoon.clone(),
                daylen: dtp.daylen.clone(),
            });
            mmsrise.update(dtp.srise.as_ref().unwrap().get_num(), dtp.yday);
            mmsset.update(dtp.sset.as_ref().unwrap().get_num(), dtp.yday);
            mmsolnoon.update(dtp.solnoon.as_ref().unwrap().get_num(), dtp.yday);
            mmdaylen.update(dtp.daylen.as_ref().unwrap().get_num(), dtp.yday);
        }

        let output = serde_json::to_string_pretty(&xdata).expect("Unable to serialize JSON");

        fs::write(&transformed, output).expect("Unable to write file");

        println!(
            "Min/Max Sunrise: {} (day {}) - {} (day {})",
            mmsrise.min, mmsrise.min_day, mmsrise.max, mmsrise.max_day
        );
        println!(
            "Min/Max Sunset: {} (day {}) - {} (day {})",
            mmsset.min, mmsset.min_day, mmsset.max, mmsset.max_day
        );
        println!(
            "Min/Max Solar Noon: {} (day {}) - {} (day {})",
            mmsolnoon.min, mmsolnoon.min_day, mmsolnoon.max, mmsolnoon.max_day
        );
        println!(
            "Min/Max Day Length: {} (day {}) - {} (day {})",
            mmdaylen.min, mmdaylen.min_day, mmdaylen.max, mmdaylen.max_day
        );

        let fit_srise = mmsrise.fit(
            &data
                .iter()
                .map(|d| (d.srise.as_ref().unwrap().get_num(), d.yday))
                .collect(),
            1.0,
        );
        let fit_sset = mmsset.fit(
            &data
                .iter()
                .map(|d| (d.sset.as_ref().unwrap().get_num(), d.yday))
                .collect(),
            1.0,
        );
        let fit_solnoon = mmsolnoon.fit(
            &data
                .iter()
                .map(|d| (d.solnoon.as_ref().unwrap().get_num(), d.yday))
                .collect(),
            2.0,
        );
        let fit_daylen = mmdaylen.fit(
            &data
                .iter()
                .map(|d| (d.daylen.as_ref().unwrap().get_num(), d.yday))
                .collect(),
            1.0,
        );

        println!("Fitment Sunrise: {:?}", fit_srise);
        println!("Fitment Sunset: {:?}", fit_sset);
        println!("Fitment Solar Noon: {:?}", fit_solnoon);
        println!("Fitment Day Length: {:?}", fit_daylen);

        chart
            .draw_series(LineSeries::new(
                (0..365).map(|x| {
                    (
                        x,
                        fit_srise.a
                            * (fit_srise.b * x as f64
                                + fit_srise.c * 2.0 * std::f64::consts::PI / 365.0)
                                .sin()
                            + fit_srise.d,
                    )
                }),
                &RGBColor(255, 0, 0),
            ))
            .unwrap()
            .label("Sunrise Fitment");

        chart
            .draw_series(LineSeries::new(
                (0..365).map(|x| {
                    (
                        x,
                        fit_sset.a
                            * (fit_sset.b * x as f64
                                + fit_sset.c * 2.0 * std::f64::consts::PI / 365.0)
                                .sin()
                            + fit_sset.d,
                    )
                }),
                &RGBColor(0, 255, 0),
            ))
            .unwrap()
            .label("Sunset Fitment");

        chart
            .draw_series(LineSeries::new(
                (0..365).map(|x| {
                    (
                        x,
                        fit_solnoon.a
                            * (fit_solnoon.b * x as f64
                                + fit_solnoon.c * 4.0 * std::f64::consts::PI / 365.0)
                                .sin()
                            + fit_solnoon.d,
                    )
                }),
                &RGBColor(0, 0, 255),
            ))
            .unwrap()
            .label("Solar Noon Fitment");

        chart
            .draw_series(LineSeries::new(
                (0..365).map(|x| {
                    (
                        x,
                        fit_daylen.a
                            * (fit_daylen.b * x as f64
                                + fit_daylen.c * 2.0 * std::f64::consts::PI / 365.0)
                                .sin()
                            + fit_daylen.d,
                    )
                }),
                &RGBColor(0, 0, 0),
            ))
            .unwrap()
            .label("Day Length Fitment");
    }

    root.present().unwrap();
}
