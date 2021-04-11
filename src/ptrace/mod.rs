mod mock_ptrace;
pub use mock_ptrace::*;

mod syscall_ptrace;
pub use syscall_ptrace::*;

use crate::from_c;
use crate::TraceError;

pub(crate) const MAX_MEMORY_READ_SIZE: usize = 1024 * 1024;

pub trait Tracer {
    #[inline]
    fn assert_memory_len(&self, len: usize) -> Result<(), TraceError> {
        if len > MAX_MEMORY_READ_SIZE {
            return Err(TraceError::TooLargeMemoryReadRequested(
                len,
                MAX_MEMORY_READ_SIZE,
            ));
        }

        Ok(())
    }

    fn initialize(&mut self) -> Result<(), TraceError>;

    /// Before-hook
    #[inline]
    fn before_data(&mut self) -> Result<(), TraceError> {
        Ok(())
    }

    fn prepare_next(&mut self) -> Result<bool, TraceError>;

    /// Get information on latest syscall
    ///
    /// * data_ptr (out) will be set into `ptrace_syscall_info` struct
    fn get_ptrace(
        &mut self,
        data_ptr: *mut from_c::ptrace_syscall_info,
    ) -> Result<GetPtraceInfo, TraceError>;

    fn finalize(&mut self) -> Result<(), TraceError>;

    fn read_memory_to_destination<T>(
        &mut self,
        pid: usize,
        address: usize,
        dest: *mut T,
    ) -> Result<(), TraceError>;

    fn find_string_from_memory(&mut self, pid: usize, address: usize)
        -> Result<String, TraceError>;

    fn read_memory_to_vec(
        &mut self,
        pid: usize,
        address: usize,
        len: usize,
    ) -> Result<Vec<u8>, TraceError>;

    /// Return a current process identified
    fn get_pid(&self) -> usize;
}

pub struct GetPtraceInfo {
    pub has_more: bool,
    pub pid: usize,
}
