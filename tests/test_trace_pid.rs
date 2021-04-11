use hstrace::prelude::*;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn test_trace_invalid_pid() {
    init();

    let tracer = HStraceBuilder::new().pid(0).build().start();

    assert_eq!(
        tracer.unwrap_err(),
        hstrace::TraceError::NixError(nix::Error::Sys(nix::errno::Errno::ESRCH))
    );
}

#[test]
fn test_trace_pid() {
    init();

    let mut cmd = std::process::Command::new("dd")
        .args(&["if=/dev/zero", "bs=1M", "count=10000", "of=/dev/null"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();

    let mut tracer = HStraceBuilder::new().pid(cmd.id() as usize).build();
    tracer.start().unwrap();

    let x: Vec<hstrace::Syscall> = tracer.iter_as_syscall().take(5).collect();
    assert_eq!(x.len(), 5);

    cmd.kill().unwrap();
}
