use sanguine_lib::resources::{exclusion, layout::{self, Parameters, LayoutType, VoronoiType}};
use svg::Document;

use std::env;
use chrono::Local;

mod work;
use work::{star_burst, voronoi};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let path = "exclusion.svg";
    let mut content = String::new();


    // voronoi::form_group();

    if let Some(exclusion) = exclusion::Exclusion::make_exclusion(path, &mut content) {
        for i in 0..5 {
            // let parameters = Parameters{ height: 1200, width: 600, margin: 2, rows: 0, columns: 0, layout_type: LayoutType::VoronoiBased(VoronoiType::Uniform(50)) };
            let parameters = Parameters{ height: 2400, width: 1200, margin: 2, rows: 20, columns: 10, layout_type: LayoutType::GridBased(20, 10) };
            
            let work = layout::Work::new(parameters);
            
            
            let document = Document::new()
                .set("viewBox", (0, 0, work.0.get_width(), work.0.get_height()))
                .add(work.background())
                .add(star_burst::form_group(&work));

            let local_time = Local::now();
            let path = format!("nr_{:03}_{}.svg", i, local_time.format("%Y%m%d_%H%M%S"));
            svg::save(path, &document).unwrap();
        }
    };
    println!("None");
}
