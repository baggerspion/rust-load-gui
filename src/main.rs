extern crate cursive;
extern crate procinfo;

use cursive::XY;
use cursive::Cursive;
use cursive::vec::Vec2;
use cursive::view::*;
use cursive::views::{Dialog};
use cursive::{Printer};
use procinfo::loadavg;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    // create the Cursive root
    let mut app = Cursive::default();

    // a callback channel
    let cb_sink = app.cb_sink().clone();

    // A channel will communicate data from our running task to the UI.
    let (tx, rx) = mpsc::channel();

    // Generate data in a separate thread.
    thread::spawn(move || {
        get_load_avg(&tx, cb_sink);
    });

    // And sets the view to read from the other end of the channel.
    app.add_layer(Dialog::around(AVGView::new(rx))
                    .title("Load average")
                    .button("Quit", |s| s.quit())
                );
    
    app.run();   
}

fn get_load_avg(tx: &mpsc::Sender<String>, cb_sink: cursive::CbSink){
    // infinite loop
    loop {
        // get current load average
        let current_avg = get_load();

        // send data to the channel
        if tx.send(current_avg).is_err() {
            return;
        }
        
        // re draw the view
        cb_sink.send(Box::new(Cursive::noop)).unwrap();

        // wait 5 seconds before getting new data
        thread::sleep(Duration::from_secs(2));
    }
}

// Let's define a view, that shows the last lines from a stream.
struct AVGView {
    buffer: String,
    // Receiving end of the stream
    rx: mpsc::Receiver<String>,
}

impl AVGView {
    // Creates a new view
    fn new(rx: mpsc::Receiver<String>) -> Self {
        let buffer = String::new();
        AVGView {
            rx: rx,
            buffer: buffer,
        }
    }

    // Reads available data from the stream into the buffer
    fn update(&mut self) {
        // Add each available line to the end of the buffer.
        while let Ok(line) = self.rx.try_recv() {
            self.buffer = line;
        }
    }
}

// AVGView is a View Interface
impl View for AVGView {
    fn layout(&mut self, _: Vec2) {
        // Before drawing, we'll want to update the buffer
        self.update();
    }

    fn draw(&self, printer: &Printer) {
        // Print the last value
        printer.print((0,0),  &self.buffer)
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        XY {x: 30, y: 1}
    }
}

fn get_load() -> String {
    // return the load average in a formated string
    let load_avg = loadavg();
    match load_avg {
        Ok(avg) => {
            format!("{:.2}, {:.2}, {:.2}", 
                    avg.load_avg_1_min,
                    avg.load_avg_5_min,
                    avg.load_avg_10_min)
            },
        Err(e) => {
            format!("Error: {}", e)
        }            
    }
}

