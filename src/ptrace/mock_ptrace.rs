use std::mem;

use super::{GetPtraceInfo, Tracer};
use crate::from_c;
use crate::TraceError;

/// Mock implementation of trace, used in fuzzing to provide syscall data
#[derive(Debug)]
pub struct MockPtrace {
    receiver: crossbeam_channel::Receiver<Vec<u8>>,
    fuzz_data: Vec<u8>,
    iteration: usize,
}

impl MockPtrace {
    pub fn new(receiver: crossbeam_channel::Receiver<Vec<u8>>) -> Self {
        MockPtrace {
            receiver,
            fuzz_data: Vec::new(),
            iteration: 0,
        }
    }

    pub fn seed(&mut self) {}
}

impl Tracer for MockPtrace {
    fn initialize(&mut self) -> Result<(), TraceError> {
        Ok(())
    }

    fn before_data(&mut self) -> Result<(), TraceError> {
        match self.receiver.try_recv() {
            Err(_) => (),
            Ok(data) => self.fuzz_data = data,
        };

        self.iteration += 1;

        Ok(())
    }

    fn prepare_next(&mut self) -> Result<bool, TraceError> {
        Ok(true)
    }

    fn get_ptrace(
        &mut self,
        data_ptr: *mut from_c::struct_ptrace_syscall_info,
    ) -> Result<GetPtraceInfo, TraceError> {
        // safety: mock_ptrace is used only in fuzz-testing of the library, not at production build
        let slice = unsafe {
            std::slice::from_raw_parts_mut(
                data_ptr as *mut u8,
                mem::size_of::<from_c::struct_ptrace_syscall_info>(),
            )
        };

        let copy_len = if self.fuzz_data.len() > slice.len() {
            slice.len()
        } else {
            self.fuzz_data.len()
        };

        slice[..copy_len].copy_from_slice(&self.fuzz_data[..copy_len]);

        self.fuzz_data = self.fuzz_data.split_off(copy_len);

        Ok(GetPtraceInfo {
            has_more: true,
            pid: 0,
        })
    }

    fn finalize(&mut self) -> Result<(), TraceError> {
        Ok(())
    }

    fn read_memory_to_destination<T>(
        &mut self,
        _pid: usize,
        _address: usize,
        _dest: *mut T,
    ) -> Result<(), TraceError> {
        Ok(())
    }

    fn find_string_from_memory(
        &mut self,
        _pid: usize,
        _address: usize,
    ) -> Result<String, TraceError> {
        Ok(String::from(""))
    }

    fn read_memory_to_vec(
        &mut self,
        _pid: usize,
        _address: usize,
        len: usize,
    ) -> Result<Vec<u8>, TraceError> {
        self.assert_memory_len(len)?;

        let data: Vec<u8> = if self.fuzz_data.len() < len {
            // take vec out
            let mut data = self.fuzz_data.split_off(0);
            data.resize(len, 0);
            data
        } else {
            self.fuzz_data.drain(0..len).collect()
        };

        assert_eq!(data.len(), len);

        Ok(data)
    }

    fn get_pid(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_mocked_syscall() {
        let (seed_sender, seed_receiver) = crossbeam_channel::bounded(5);
        let mut x = MockPtrace::new(seed_receiver);
        let mut data: from_c::ptrace_syscall_info = unsafe { std::mem::zeroed() };
        let data_ptr: *mut from_c::ptrace_syscall_info = &mut data;
        let ptrace_size = mem::size_of::<from_c::ptrace_syscall_info>();
        let mut inp = vec![0; ptrace_size * 2]; // for two iterations

        // op u8
        inp[0] = 24;

        // arch u32
        inp[4] = 255;
        inp[5] = 255;
        inp[6] = 255;
        inp[7] = 255;

        // op u8
        inp[ptrace_size + 0] = 22;

        // arch u32
        inp[ptrace_size + 4] = 255;
        inp[ptrace_size + 5] = 255;
        inp[ptrace_size + 6] = 255;
        inp[ptrace_size + 7] = 255;

        seed_sender.send(inp).unwrap();
        x.before_data().unwrap();
        let y = x.get_ptrace(data_ptr);
        assert!(y.is_ok());
        assert_eq!(data.op, 24);
        assert_eq!(data.arch, 4294967295);

        x.before_data().unwrap();
        let y = x.get_ptrace(data_ptr);
        assert!(y.is_ok());
        assert_eq!(data.op, 22);
        assert_eq!(data.arch, 4294967295);

        /*
        /// USE THIS SNIPPED TO GET THE BYTEARRAY
        let mut data: from_c::ptrace_syscall_info = unsafe { std::mem::zeroed() };
        data.op = 252;
        data.arch = 4294967295;

        let ptr = &data as *const _;
        let ptr = ptr as *const u8;

        let slice: &[u8] = unsafe {
            std::slice::from_raw_parts(ptr, mem::size_of::<from_c::ptrace_syscall_info>())
        };

        println!("ptrace_syscall_info as byte array {:?}", slice);
        */
    }
}
