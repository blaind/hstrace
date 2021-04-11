use hstrace::prelude::*;

fn main() {
    let mut tracer = HStraceBuilder::new().pid(50000).build();
    tracer.start().unwrap();

    for call in tracer.iter_as_syscall() {
        println!("TRACE: {:?}", call);
    }
}
