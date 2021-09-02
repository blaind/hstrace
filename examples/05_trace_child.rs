use hstrace::prelude::*;

fn main() {
    // build traced binary
    std::process::Command::new("make")
        .args(&["compile_main"])
        .current_dir("data/c_code")
        .output()
        .expect("Build test C binary");


    let mut tracer = HStraceBuilder::new()
        .program("data/c_code/main")
        .arg("sched")
        .build();

    tracer.start().unwrap();

    for call in tracer.iter() {
        println!("TRACE: pid={}, call={} vars={:?} out={:?}", call.pid, call.ident, call.variables, call.out);
    }
}
