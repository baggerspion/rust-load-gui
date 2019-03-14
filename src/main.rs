extern crate cursive;
extern crate procinfo;

use cursive::Cursive;
use cursive::view::*;
use cursive::views::{Dialog, LinearLayout, TextView};
use procinfo::loadavg;
use std::{thread, time};

fn main() {
    let mut app = Cursive::default();

    app.add_layer(Dialog::around(
            LinearLayout::vertical()
                  .child(TextView::new("")
                         .content(get_load())
                         .with_id("txt_view"))).button("Quit", |q| q.quit()));

    app.add_global_callback('u', |u| {
        u.call_on_id("txt_view", |view: &mut TextView| {
            view.set_content(get_load());
        });
    });
    
    app.run();
}

fn get_load() -> String {
    // Get the load and display it
    let load_avg = loadavg();
    match load_avg {
        Ok(avg) => {
            format!("Load Average: {}, {}, {}", 
                    avg.load_avg_1_min,
                    avg.load_avg_5_min,
                    avg.load_avg_10_min)
            },
        Err(e) => {
            format!("Error: {}", e)
        }            
    }
}
