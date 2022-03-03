// Tracing
#include <sys/ptrace.h>

// From strace
# if defined HAVE_STRUCT_PTRACE_SYSCALL_INFO
typedef struct ptrace_syscall_info struct_ptrace_syscall_info;
# elif defined HAVE_STRUCT___PTRACE_SYSCALL_INFO
typedef struct __ptrace_syscall_info struct_ptrace_syscall_info;
# else
#include <stdint.h>
struct ptrace_syscall_info {
	uint8_t op;
	uint8_t pad[3];
	uint32_t arch;
	uint64_t instruction_pointer;
	uint64_t stack_pointer;
	union {
		struct {
			uint64_t nr;
			uint64_t args[6];
		} entry;
		struct {
			int64_t rval;
			uint8_t is_error;
		} exit;
		struct {
			uint64_t nr;
			uint64_t args[6];
			uint32_t ret_data;
		} seccomp;
	};
};
typedef struct ptrace_syscall_info struct_ptrace_syscall_info;
# endif

// Syscalls
#include <sys/utsname.h>
#include <sys/stat.h>
#include <sys/sysinfo.h>
