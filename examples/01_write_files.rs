use hstrace::prelude::*;

fn main() {
    let mut tracer = HStraceBuilder::new()
        .program("touch")
        .arg("/tmp/HStrace_example.txt")
        .build();

    tracer.start().unwrap();

    for call in tracer.iter_as_syscall() {
        match call.kind {
            SyscallKind::Openat(o) => {
                if o.flags.contains(call::OpenatMode::O_WRONLY) {
                    println!("File {} opened in write-mode ({:?})", o.pathname, o.flags);
                }
            }
            _ => (),
        }
    }
}
