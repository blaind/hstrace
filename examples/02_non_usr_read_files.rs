use hstrace::prelude::*;

fn main() {
    let mut tracer = HStraceBuilder::new()
        .program("ssh")
        .args(vec!["localhost:0".to_owned()])
        .build();

    tracer.start().unwrap();

    for call in tracer.iter_as_syscall() {
        match call.kind {
            SyscallKind::Openat(o) => {
                if o.pathname.starts_with("/home") {
                    println!(
                        "File {} opened from home-folder ({:?})",
                        o.pathname, o.flags
                    );
                }
            }
            _ => (),
        }
    }
}
