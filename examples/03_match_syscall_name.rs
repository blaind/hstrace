use hstrace::prelude::*;

fn main() {
    let mut tracer = HStraceBuilder::new().program("ps").arg("uxaw").build();

    tracer.start().unwrap();

    for syscall in tracer.iter_as_syscall() {
        match syscall.name {
            hstrace::Ident::Openat | hstrace::Ident::Fstat | hstrace::Ident::Stat => {
                println!("File operation detected: {:?}", syscall);
            }

            hstrace::Ident::Socket | hstrace::Ident::Bind | hstrace::Ident::Connect => {
                println!("Network operation detected: {:?}", syscall);
            }

            _ => (),
        }
    }
}
