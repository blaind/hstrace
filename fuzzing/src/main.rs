#[macro_use]
extern crate afl;

use hstrace;
use hstrace::ptrace::Tracer;
use hstrace::value::MockPtrace;
use hstrace::TraceError;

use std::fs::File;
use std::io::prelude::*;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

fn main() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log.txt")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();

    log::info!("Starting!");

    fuzz!(|data: &[u8]| {
        let (seed_sender, seed_receiver) = crossbeam_channel::bounded(5);
        let mut ptrace = MockPtrace::new(seed_receiver);
        ptrace.initialize();

        let (sender, r) = crossbeam_channel::bounded(5);
        let mut tracer_thread =
            hstrace::TraceThread::new(ptrace, sender, hstrace::TraceOptions::default());

        match seed_sender.try_send(data.to_vec()) {
            Err(e) => {
                log::error!("Seeder data send failure!");
            }
            Ok(_) => (),
        }

        for i in 0..2 {
            match tracer_thread.iterate() {
                Err(e) => {
                    break;
                }
                Ok(_) => (),
            }
        }
        r.try_recv();

        match tracer_thread.finalize() {
            Err(e) => match e {
                TraceError::UnknownOp(_)
                | TraceError::DuplicateEntry
                | TraceError::NoMatchingEntryData
                | TraceError::TooLargeMemoryReadRequested(_, _) => (),
                _ => {
                    log::info!("Iterate failure: {:?}", e);
                }
            },
            Ok(_) => (),
        }
    });
}
