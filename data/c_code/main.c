#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <string.h>
#include <stdbool.h>

#define __USE_GNU
#include <sched.h>

#include <sys/acct.h>
#include <sys/auxv.h>
#include <sys/bitypes.h>
#include <sys/cdefs.h>
#include <sys/debugreg.h>
#include <sys/dir.h>
#include <sys/epoll.h>
#include <sys/errno.h>
#include <sys/eventfd.h>
#include <sys/fanotify.h>
#include <sys/fcntl.h>
#include <sys/file.h>
#include <sys/fsuid.h>
#include <sys/gmon.h>
#include <sys/gmon_out.h>
#include <sys/inotify.h>
#include <sys/ioctl.h>
#include <sys/io.h>
#include <sys/ipc.h>
#include <sys/kd.h>
#include <sys/klog.h>
#include <sys/mman.h>
#include <sys/mount.h>
#include <sys/msg.h>
#include <sys/mtio.h>
#include <sys/param.h>
#include <sys/pci.h>
#include <sys/perm.h>
#include <sys/personality.h>
#include <sys/poll.h>
#include <sys/prctl.h>
#include <sys/procfs.h>
#include <sys/profil.h>
#include <sys/ptrace.h>
#include <sys/queue.h>
#include <sys/quota.h>
#include <sys/random.h>
#include <sys/raw.h>
#include <sys/reboot.h>
#include <sys/reg.h>
#include <sys/resource.h>
#include <sys/select.h>
#include <sys/sem.h>
#include <sys/sendfile.h>
#include <sys/shm.h>
#include <sys/signalfd.h>
#include <sys/signal.h>
#include <sys/socket.h>
#include <sys/socketvar.h>
#include <sys/soundcard.h>
#include <sys/statfs.h>
#include <sys/stat.h>
#include <sys/statvfs.h>
#include <sys/swap.h>
#include <sys/syscall.h>
#include <sys/sysinfo.h>
#include <sys/syslog.h>
#include <sys/sysmacros.h>
#include <sys/termios.h>
#include <sys/timeb.h>
#include <sys/time.h>
#include <sys/timerfd.h>
#include <sys/times.h>
#include <sys/timex.h>
#include <sys/ttychars.h>
#include <sys/ttydefaults.h>
#include <sys/types.h>
#include <sys/ucontext.h>
#include <sys/uio.h>
#include <sys/un.h>
#include <sys/unistd.h>
#include <sys/user.h>
#include <sys/utsname.h>
#include <sys/vfs.h>
#include <sys/vlimit.h>
#include <sys/vt.h>
#include <sys/vtimes.h>
#include <sys/wait.h>
#include <sys/xattr.h>

int sched_child_2()
{
    char buf[256];
    readlink("/tmp/link_src_child_2", buf, 256);

    return 0;
}

#define STACK_SIZE (1024 * 1024)
int sched_child_1()
{
    char buf[256];
    readlink("/tmp/link_src_child_1", buf, 256);

    char *stack = malloc(STACK_SIZE);
    if (stack == NULL)
    {
        printf("ERROR: unable to allocate\n");
        return 255;
    }

    char *stackTop = stack + STACK_SIZE;
    int pid = clone(sched_child_2, stackTop, SIGCHLD, NULL);
    if (pid < 0)
    {
        printf("ERROR: Could not clone()\n");
        return 255;
    }

    wait(NULL);
    free(stack);

    return 0;
}

void sched_ops()
{
    char *stack = malloc(STACK_SIZE);
    if (stack == NULL)
    {
        printf("ERROR: unable to allocate\n");
        return;
    }

    char *stackTop = stack + STACK_SIZE;
    int pid = clone(sched_child_1, stackTop, SIGCHLD, NULL);
    if (pid < 0)
    {
        printf("ERROR: Could not clone()\n");
        return;
    }

    wait(NULL);
    free(stack);
}

void fncntl_ops()
{
    const char access_path[] = "/tmp/hstrace.test";
    // printf("O_WRONLY: %d O_APPEND: %d", O_WRONLY, O_APPEND);
    openat(0, access_path, O_WRONLY | O_APPEND);
}

//unistd.h : extern int access(const char *__name, int __type) __THROW __nonnull((1));
//unistd.h : extern int euidaccess(const char *__name, int __type)
//unistd.h : extern int eaccess(const char *__name, int __type)
//unistd.h : extern int faccessat(int __fd, const char *__file, int __type, int __flag)
//unistd.h : extern int close(int __fd);
//unistd.h : extern ssize_t read(int __fd, void *__buf, size_t __nbytes) __wur;
//unistd.h : extern ssize_t write(int __fd, const void *__buf, size_t __n) __wur;
//unistd.h:extern ssize_t pread (int __fd, void *__buf, size_t __nbytes,
//unistd.h:extern ssize_t pwrite (int __fd, const void *__buf, size_t __n,
//unistd.h:extern ssize_t __REDIRECT (pread, (int __fd, void *__buf, size_t __nbytes,
//unistd.h:extern ssize_t __REDIRECT (pwrite, (int __fd, const void *__buf,
//unistd.h:extern ssize_t pread64 (int __fd, void *__buf, size_t __nbytes,
//unistd.h:extern ssize_t pwrite64 (int __fd, const void *__buf, size_t __n,
//unistd.h:extern int pipe (int __pipedes[2]) __THROW __wur;
//unistd.h:extern int pipe2 (int __pipedes[2], int __flags) __THROW __wur;
//unistd.h:extern unsigned int alarm (unsigned int __seconds) __THROW;
//unistd.h:extern unsigned int sleep (unsigned int __seconds);
//unistd.h:extern int usleep (__useconds_t __useconds);
//unistd.h:extern int pause (void);
//unistd.h:extern int chown (const char *__file, __uid_t __owner, __gid_t __group)
//unistd.h:extern int fchown (int __fd, __uid_t __owner, __gid_t __group) __THROW __wur;
//unistd.h:extern int lchown (const char *__file, __uid_t __owner, __gid_t __group)
//unistd.h:extern int fchownat (int __fd, const char *__file, __uid_t __owner,
//unistd.h:extern int chdir (const char *__path) __THROW __nonnull ((1)) __wur;
//unistd.h:extern int fchdir (int __fd) __THROW __wur;
//unistd.h:extern char *getcwd (char *__buf, size_t __size) __THROW __wur;
//unistd.h:extern char *get_current_dir_name (void) __THROW;
//unistd.h:extern char *getwd (char *__buf)
//unistd.h:extern int dup (int __fd) __THROW __wur;
//unistd.h:extern int dup2 (int __fd, int __fd2) __THROW;
//unistd.h:extern int dup3 (int __fd, int __fd2, int __flags) __THROW;
//unistd.h:extern char **__environ;
//unistd.h:extern char **environ;
//unistd.h:extern int execve (const char *__path, char *const __argv[],
//unistd.h:extern int fexecve (int __fd, char *const __argv[], char *const __envp[])
//unistd.h:extern int execv (const char *__path, char *const __argv[])
//unistd.h:extern int execle (const char *__path, const char *__arg, ...)
//unistd.h:extern int execl (const char *__path, const char *__arg, ...)
//unistd.h:extern int execvp (const char *__file, char *const __argv[])
//unistd.h:extern int execlp (const char *__file, const char *__arg, ...)
//unistd.h:extern int execvpe (const char *__file, char *const __argv[],
//unistd.h:extern int nice (int __inc) __THROW __wur;
//unistd.h:extern void _exit (int __status) __attribute__ ((__noreturn__));
//unistd.h:extern long int pathconf (const char *__path, int __name)
//unistd.h:extern long int fpathconf (int __fd, int __name) __THROW;
//unistd.h:extern long int sysconf (int __name) __THROW;
//unistd.h:extern size_t confstr (int __name, char *__buf, size_t __len) __THROW;
//unistd.h:extern int setpgid (__pid_t __pid, __pid_t __pgid) __THROW;
//unistd.h:extern int setpgrp (void) __THROW;
//unistd.h:extern int getgroups (int __size, __gid_t __list[]) __THROW __wur;
//unistd.h:extern int group_member (__gid_t __gid) __THROW;
//unistd.h:extern int setuid (__uid_t __uid) __THROW __wur;
//unistd.h:extern int setreuid (__uid_t __ruid, __uid_t __euid) __THROW __wur;
//unistd.h:extern int seteuid (__uid_t __uid) __THROW __wur;
//unistd.h:extern int setgid (__gid_t __gid) __THROW __wur;
//unistd.h:extern int setregid (__gid_t __rgid, __gid_t __egid) __THROW __wur;
//unistd.h:extern int setegid (__gid_t __gid) __THROW __wur;
//unistd.h:extern int getresuid (__uid_t *__ruid, __uid_t *__euid, __uid_t *__suid)
//unistd.h:extern int getresgid (__gid_t *__rgid, __gid_t *__egid, __gid_t *__sgid)
//unistd.h:extern int setresuid (__uid_t __ruid, __uid_t __euid, __uid_t __suid)
//unistd.h:extern int setresgid (__gid_t __rgid, __gid_t __egid, __gid_t __sgid)
//unistd.h:extern char *ttyname (int __fd) __THROW;
//unistd.h:extern int ttyname_r (int __fd, char *__buf, size_t __buflen)
//unistd.h:extern int isatty (int __fd) __THROW;
//unistd.h:extern int ttyslot (void) __THROW;
//unistd.h:extern int link (const char *__from, const char *__to)
//unistd.h:extern int linkat (int __fromfd, const char *__from, int __tofd,
//unistd.h:extern int symlink (const char *__from, const char *__to)
//unistd.h:extern ssize_t readlink (const char *__restrict __path,
//unistd.h:extern int symlinkat (const char *__from, int __tofd,
//unistd.h:extern ssize_t readlinkat (int __fd, const char *__restrict __path,
//unistd.h:extern int unlink (const char *__name) __THROW __nonnull ((1));
//unistd.h:extern int unlinkat (int __fd, const char *__name, int __flag)
//unistd.h:extern int rmdir (const char *__path) __THROW __nonnull ((1));
//unistd.h:extern int tcsetpgrp (int __fd, __pid_t __pgrp_id) __THROW;
//unistd.h:extern char *getlogin (void);
//unistd.h:extern int getlogin_r (char *__name, size_t __name_len) __nonnull ((1));
//unistd.h:extern int setlogin (const char *__name) __THROW __nonnull ((1));
//unistd.h:extern int gethostname (char *__name, size_t __len) __THROW __nonnull ((1));
//unistd.h:extern int sethostname (const char *__name, size_t __len)
//unistd.h:extern int sethostid (long int __id) __THROW __wur;
//unistd.h:extern int getdomainname (char *__name, size_t __len)
//unistd.h:extern int setdomainname (const char *__name, size_t __len)
//unistd.h:extern int vhangup (void) __THROW;
//unistd.h:extern int revoke (const char *__file) __THROW __nonnull ((1)) __wur;
//unistd.h:extern int profil (unsigned short int *__sample_buffer, size_t __size,
//unistd.h:extern int acct (const char *__name) __THROW;
//unistd.h:extern char *getusershell (void) __THROW;
//unistd.h:extern void endusershell (void) __THROW; /* Discard cached info.  */
//unistd.h:extern void setusershell (void) __THROW; /* Rewind and re-read the file.  */
//unistd.h:extern int daemon (int __nochdir, int __noclose) __THROW __wur;
//unistd.h:extern int chroot (const char *__path) __THROW __nonnull ((1)) __wur;
//unistd.h:extern char *getpass (const char *__prompt) __nonnull ((1));
//unistd.h:extern int fsync (int __fd);
//unistd.h:extern int syncfs (int __fd) __THROW;
//unistd.h:extern long int gethostid (void);
//unistd.h:extern void sync (void) __THROW;
//unistd.h:extern int getpagesize (void)  __THROW __attribute__ ((__const__));
//unistd.h:extern int getdtablesize (void) __THROW;
//unistd.h:extern int truncate (const char *__file, __off_t __length)
//unistd.h:extern int __REDIRECT_NTH (truncate,
//unistd.h:extern int truncate64 (const char *__file, __off64_t __length)
//unistd.h:extern int ftruncate (int __fd, __off_t __length) __THROW __wur;
//unistd.h:extern int __REDIRECT_NTH (ftruncate, (int __fd, __off64_t __length),
//unistd.h:extern int ftruncate64 (int __fd, __off64_t __length) __THROW __wur;
//unistd.h:extern int brk (void *__addr) __THROW __wur;
//unistd.h:extern void *sbrk (intptr_t __delta) __THROW;
//unistd.h:extern long int syscall (long int __sysno, ...) __THROW;
//unistd.h:extern int lockf (int __fd, int __cmd, __off_t __len) __wur;
//unistd.h:extern int __REDIRECT (lockf, (int __fd, int __cmd, __off64_t __len),
//unistd.h:extern int lockf64 (int __fd, int __cmd, __off64_t __len) __wur;
//unistd.h:extern int fdatasync (int __fildes);
//unistd.h:extern char *crypt (const char *__key, const char *__salt)
//unistd.h:extern void swab (const void *__restrict __from, void *__restrict __to,
//unistd.h:extern char *ctermid (char *__s) __THROW;
//unistd.h:extern char *cuserid (char *__s);
//unistd.h:extern int pthread_atfork (void (*__prepare) (void),
void unistd_ops()
{
    const char access_path[] = "/tmp";

    access(access_path, F_OK | R_OK | W_OK);
    access(access_path, F_OK);

    char cwd[4096];
    getcwd(cwd, sizeof(cwd));

    readlink("/tmp/link_src", "/tmp/link_dst", 3);
}

//acct.h : extern int acct(const char *__filename) __THROW;
//auxv.h : extern unsigned long int getauxval(unsigned long int __type)

//epoll.h : extern int epoll_create(int __size) __THROW;
//epoll.h : extern int epoll_create1(int __flags) __THROW;
//epoll.h:extern int epoll_ctl (int __epfd, int __op, int __fd,
//epoll.h:extern int epoll_wait (int __epfd, struct epoll_event *__events,
//epoll.h:extern int epoll_pwait (int __epfd, struct epoll_event *__events,

//eventfd.h:extern int eventfd (unsigned int __count, int __flags) __THROW;
//eventfd.h:extern int eventfd_read (int __fd, eventfd_t *__value);
//eventfd.h:extern int eventfd_write (int __fd, eventfd_t __value);

//fanotify.h:extern int fanotify_init (unsigned int __flags, unsigned int __event_f_flags)
//fanotify.h:extern int fanotify_mark (int __fanotify_fd, unsigned int __flags,

//file.h:extern int flock (int __fd, int __operation) __THROW;

//fsuid.h:extern int setfsuid (__uid_t __uid) __THROW;
//fsuid.h:extern int setfsgid (__gid_t __gid) __THROW;

//gmon.h:extern struct __bb *__bb_head;
//gmon.h:extern void __monstartup (unsigned long __lowpc, unsigned long __highpc) __THROW;
//gmon.h:extern void monstartup (unsigned long __lowpc, unsigned long __highpc) __THROW;
//gmon.h:extern void _mcleanup (void) __THROW;

//inotify.h:extern int inotify_init (void) __THROW;
//inotify.h:extern int inotify_init1 (int __flags) __THROW;
//inotify.h:extern int inotify_add_watch (int __fd, const char *__name, uint32_t __mask)
//inotify.h:extern int inotify_rm_watch (int __fd, int __wd) __THROW;

//ioctl.h:extern int ioctl (int __fd, unsigned long int __request, ...) __THROW;

//io.h:extern int ioperm (unsigned long int __from, unsigned long int __num,
//io.h:extern int iopl (int __level) __THROW;

//ipc.h:extern key_t ftok (const char *__pathname, int __proj_id) __THROW;

//klog.h:extern int klogctl (int __type, char *__bufp, int __len) __THROW;

//mman.h:extern void *mmap (void *__addr, size_t __len, int __prot,
//mman.h:extern void * __REDIRECT_NTH (mmap,
//mman.h:extern void *mmap64 (void *__addr, size_t __len, int __prot,
//mman.h:extern int munmap (void *__addr, size_t __len) __THROW;
//mman.h:extern int mprotect (void *__addr, size_t __len, int __prot) __THROW;
//mman.h:extern int msync (void *__addr, size_t __len, int __flags);
//mman.h:extern int madvise (void *__addr, size_t __len, int __advice) __THROW;
//mman.h:extern int posix_madvise (void *__addr, size_t __len, int __advice) __THROW;
//mman.h:extern int mlock (const void *__addr, size_t __len) __THROW;
//mman.h:extern int munlock (const void *__addr, size_t __len) __THROW;
//mman.h:extern int mlockall (int __flags) __THROW;
//mman.h:extern int munlockall (void) __THROW;
//mman.h:extern int mincore (void *__start, size_t __len, unsigned char *__vec)
//mman.h:extern void *mremap (void *__addr, size_t __old_len, size_t __new_len,
//mman.h:extern int remap_file_pages (void *__start, size_t __size, int __prot,
//mman.h:extern int shm_open (const char *__name, int __oflag, mode_t __mode);
//mman.h:extern int shm_unlink (const char *__name);

//mount.h:extern int mount (const char *__special_file, const char *__dir,
//mount.h:extern int umount (const char *__special_file) __THROW;
//mount.h:extern int umount2 (const char *__special_file, int __flags) __THROW;

//msg.h:extern int msgctl (int __msqid, int __cmd, struct msqid_ds *__buf) __THROW;
//msg.h:extern int msgget (key_t __key, int __msgflg) __THROW;
//msg.h:extern ssize_t msgrcv (int __msqid, void *__msgp, size_t __msgsz,
//msg.h:extern int msgsnd (int __msqid, const void *__msgp, size_t __msgsz,

//perm.h:extern int ioperm (unsigned long int __from, unsigned long int __num,
//perm.h:extern int iopl (int __level) __THROW;

//personality.h:extern int personality (unsigned long int __persona) __THROW;

//poll.h:extern int poll (struct pollfd *__fds, nfds_t __nfds, int __timeout);
//poll.h:extern int ppoll (struct pollfd *__fds, nfds_t __nfds,

//prctl.h:extern int prctl (int __option, ...) __THROW;

//profil.h:extern int sprofil (struct prof *__profp, int __profcnt,

//quota.h:extern int quotactl (int __cmd, const char *__special, int __id,

//reboot.h:extern int reboot (int __howto) __THROW;

//resource.h:extern int getrlimit (__rlimit_resource_t __resource,
//resource.h:extern int __REDIRECT_NTH (getrlimit, (__rlimit_resource_t __resource,
//resource.h:extern int getrlimit64 (__rlimit_resource_t __resource,
//resource.h:extern int setrlimit (__rlimit_resource_t __resource,
//resource.h:extern int __REDIRECT_NTH (setrlimit, (__rlimit_resource_t __resource,
//resource.h:extern int setrlimit64 (__rlimit_resource_t __resource,
//resource.h:extern int getrusage (__rusage_who_t __who, struct rusage *__usage) __THROW;
//resource.h:extern int getpriority (__priority_which_t __which, id_t __who) __THROW;
//resource.h:extern int setpriority (__priority_which_t __which, id_t __who, int __prio)

//select.h:extern int select (int __nfds, fd_set *__restrict __readfds,
//select.h:extern int pselect (int __nfds, fd_set *__restrict __readfds,

//sem.h:extern int semctl (int __semid, int __semnum, int __cmd, ...) __THROW;
//sem.h:extern int semget (key_t __key, int __nsems, int __semflg) __THROW;
//sem.h:extern int semop (int __semid, struct sembuf *__sops, size_t __nsops) __THROW;
//sem.h:extern int semtimedop (int __semid, struct sembuf *__sops, size_t __nsops,

//sendfile.h:extern ssize_t sendfile (int __out_fd, int __in_fd, off_t *__offset,
//sendfile.h:extern ssize_t __REDIRECT_NTH (sendfile,
//sendfile.h:extern ssize_t sendfile64 (int __out_fd, int __in_fd, __off64_t *__offset,
void sendfile_ops()
{
    sendfile(5, 4, 0, 10);
    sendfile(6, 3, 0, 10);
}

//shm.h:extern int shmctl (int __shmid, int __cmd, struct shmid_ds *__buf) __THROW;
//shm.h:extern int shmget (key_t __key, size_t __size, int __shmflg) __THROW;
//shm.h:extern void *shmat (int __shmid, const void *__shmaddr, int __shmflg)
//shm.h:extern int shmdt (const void *__shmaddr) __THROW;
void shm_ops()
{
}

//signalfd.h:extern int signalfd (int __fd, const sigset_t *__mask, int __flags)

//socket.h:extern int socket (int __domain, int __type, int __protocol) __THROW;
//socket.h:extern int socketpair (int __domain, int __type, int __protocol,
//socket.h:extern int bind (int __fd, __CONST_SOCKADDR_ARG __addr, socklen_t __len)
//socket.h:extern int getsockname (int __fd, __SOCKADDR_ARG __addr,
//socket.h:extern int connect (int __fd, __CONST_SOCKADDR_ARG __addr, socklen_t __len);
//socket.h:extern int getpeername (int __fd, __SOCKADDR_ARG __addr,
//socket.h:extern ssize_t send (int __fd, const void *__buf, size_t __n, int __flags);
//socket.h:extern ssize_t recv (int __fd, void *__buf, size_t __n, int __flags);
//socket.h:extern ssize_t sendto (int __fd, const void *__buf, size_t __n,
//socket.h:extern ssize_t recvfrom (int __fd, void *__restrict __buf, size_t __n,
//socket.h:extern ssize_t sendmsg (int __fd, const struct msghdr *__message,
//socket.h:extern int sendmmsg (int __fd, struct mmsghdr *__vmessages,
//socket.h:extern ssize_t recvmsg (int __fd, struct msghdr *__message, int __flags);
//socket.h:extern int recvmmsg (int __fd, struct mmsghdr *__vmessages,
//socket.h:extern int getsockopt (int __fd, int __level, int __optname,
//socket.h:extern int setsockopt (int __fd, int __level, int __optname,
//socket.h:extern int listen (int __fd, int __n) __THROW;
//socket.h:extern int accept (int __fd, __SOCKADDR_ARG __addr,
//socket.h:extern int accept4 (int __fd, __SOCKADDR_ARG __addr,
//socket.h:extern int shutdown (int __fd, int __how) __THROW;
//socket.h:extern int sockatmark (int __fd) __THROW;
//socket.h:extern int isfdtype (int __fd, int __fdtype) __THROW;
void socket_ops()
{
    int server_fd = socket(AF_INET, SOCK_DGRAM, 0);

    int enable = 1;
    setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR, &enable, sizeof(int));

    struct sockaddr_in serv_addr;
    serv_addr.sin_family = AF_INET;
    serv_addr.sin_port = htons(12345);
    inet_pton(AF_INET, "127.10.0.1", &serv_addr.sin_addr);

    char *sendmsg = "Test!";

    sendto(server_fd, sendmsg, strlen(sendmsg), MSG_CONFIRM, (struct sockaddr *)&serv_addr, sizeof(serv_addr));
    connect(server_fd, (struct sockaddr *)&serv_addr, sizeof(serv_addr));

    send(server_fd, sendmsg, strlen(sendmsg), 0);
    close(server_fd);

    // ipv6
    server_fd = socket(AF_INET6, SOCK_DGRAM, 0);
    struct sockaddr_in6 v6;
    v6.sin6_family = AF_INET6;
    v6.sin6_port = htons(12345);
    inet_pton(AF_INET6, "::1", &v6.sin6_addr);
    connect(server_fd, (struct sockaddr *)&v6, sizeof(v6));

    // try too large size
    //connect(server_fd, (struct sockaddr *)&v6, 1024 * 1024 * 100);
}

//statfs.h:extern int statfs (const char *__file, struct statfs *__buf)
//statfs.h:extern int __REDIRECT_NTH (statfs,
//statfs.h:extern int statfs64 (const char *__file, struct statfs64 *__buf)
//statfs.h:extern int fstatfs (int __fildes, struct statfs *__buf)
//statfs.h:extern int __REDIRECT_NTH (fstatfs, (int __fildes, struct statfs *__buf),
//statfs.h:extern int fstatfs64 (int __fildes, struct statfs64 *__buf)
//stat.h:extern int stat (const char *__restrict __file,
//stat.h:extern int fstat (int __fd, struct stat *__buf) __THROW __nonnull ((2));
//stat.h:extern int __REDIRECT_NTH (stat, (const char *__restrict __file,
//stat.h:extern int __REDIRECT_NTH (fstat, (int __fd, struct stat *__buf), fstat64)
//stat.h:extern int stat64 (const char *__restrict __file,
//stat.h:extern int fstat64 (int __fd, struct stat64 *__buf) __THROW __nonnull ((2));
//stat.h:extern int fstatat (int __fd, const char *__restrict __file,
//stat.h:extern int __REDIRECT_NTH (fstatat, (int __fd, const char *__restrict __file,
//stat.h:extern int fstatat64 (int __fd, const char *__restrict __file,
//stat.h:extern int lstat (const char *__restrict __file,
//stat.h:extern int __REDIRECT_NTH (lstat,
//stat.h:extern int lstat64 (const char *__restrict __file,
//stat.h:extern int chmod (const char *__file, __mode_t __mode)
//stat.h:extern int lchmod (const char *__file, __mode_t __mode)
//stat.h:extern int fchmod (int __fd, __mode_t __mode) __THROW;
//stat.h:extern int fchmodat (int __fd, const char *__file, __mode_t __mode,
//stat.h:extern __mode_t umask (__mode_t __mask) __THROW;
//stat.h:extern __mode_t getumask (void) __THROW;
//stat.h:extern int mkdir (const char *__path, __mode_t __mode)
//stat.h:extern int mkdirat (int __fd, const char *__path, __mode_t __mode)
//stat.h:extern int mknod (const char *__path, __mode_t __mode, __dev_t __dev)
//stat.h:extern int mknodat (int __fd, const char *__path, __mode_t __mode,
//stat.h:extern int mkfifo (const char *__path, __mode_t __mode)
//stat.h:extern int mkfifoat (int __fd, const char *__path, __mode_t __mode)
//stat.h:extern int utimensat (int __fd, const char *__path,
//stat.h:extern int futimens (int __fd, const struct timespec __times[2]) __THROW;
//stat.h:extern int __fxstat (int __ver, int __fildes, struct stat *__stat_buf)
//stat.h:extern int __xstat (int __ver, const char *__filename,
//stat.h:extern int __lxstat (int __ver, const char *__filename,
//stat.h:extern int __fxstatat (int __ver, int __fildes, const char *__filename,
//stat.h:extern int __REDIRECT_NTH (__fxstat, (int __ver, int __fildes,
//stat.h:extern int __REDIRECT_NTH (__xstat, (int __ver, const char *__filename,
//stat.h:extern int __REDIRECT_NTH (__lxstat, (int __ver, const char *__filename,
//stat.h:extern int __REDIRECT_NTH (__fxstatat, (int __ver, int __fildes,
//stat.h:extern int __fxstat64 (int __ver, int __fildes, struct stat64 *__stat_buf)
//stat.h:extern int __xstat64 (int __ver, const char *__filename,
//stat.h:extern int __lxstat64 (int __ver, const char *__filename,
//stat.h:extern int __fxstatat64 (int __ver, int __fildes, const char *__filename,
//stat.h:extern int __xmknod (int __ver, const char *__path, __mode_t __mode,
//stat.h:extern int __xmknodat (int __ver, int __fd, const char *__path,
void stat_ops()
{
    const char stat_path[] = "/____nonexistant";
    struct stat statbuf;
    stat(stat_path, &statbuf);
}

//statvfs.h:extern int statvfs (const char *__restrict __file,
//statvfs.h:extern int __REDIRECT_NTH (statvfs,
//statvfs.h:extern int statvfs64 (const char *__restrict __file,
//statvfs.h:extern int fstatvfs (int __fildes, struct statvfs *__buf)
//statvfs.h:extern int __REDIRECT_NTH (fstatvfs, (int __fildes, struct statvfs *__buf),
//statvfs.h:extern int fstatvfs64 (int __fildes, struct statvfs64 *__buf)

void swap_ops() // total: 2
{
    const char swap_path[] = "/tmp/ptrace/swap";

    swapon(swap_path, SWAP_FLAG_DISCARD); // FIXME more flags
    swapoff(swap_path);
}

void sysinfo_ops()
{
    struct sysinfo info;
    int s0 = sysinfo(&info);
}

//syslog.h:extern void closelog (void);
//syslog.h:extern void openlog (const char *__ident, int __option, int __facility);
//syslog.h:extern int setlogmask (int __mask) __THROW;
//syslog.h:extern void syslog (int __pri, const char *__fmt, ...)
//syslog.h:extern void vsyslog (int __pri, const char *__fmt, __gnuc_va_list __ap)

//timeb.h:extern int ftime (struct timeb *__timebuf);

//time.h:extern int gettimeofday (struct timeval *__restrict __tv,
//time.h:extern int settimeofday (const struct timeval *__tv,
//time.h:extern int adjtime (const struct timeval *__delta,
//time.h:extern int getitimer (__itimer_which_t __which,
//time.h:extern int setitimer (__itimer_which_t __which,
//time.h:extern int utimes (const char *__file, const struct timeval __tvp[2])
//time.h:extern int lutimes (const char *__file, const struct timeval __tvp[2])
//time.h:extern int futimes (int __fd, const struct timeval __tvp[2]) __THROW;
//time.h:extern int futimesat (int __fd, const char *__file,

//timerfd.h:extern int timerfd_create (__clockid_t __clock_id, int __flags) __THROW;
//timerfd.h:extern int timerfd_settime (int __ufd, int __flags,
//timerfd.h:extern int timerfd_gettime (int __ufd, struct itimerspec *__otmr) __THROW;

//times.h:extern clock_t times (struct tms *__buffer) __THROW;
//timex.h:extern int __adjtimex (struct timex *__ntx) __THROW;
//timex.h:extern int adjtimex (struct timex *__ntx) __THROW;
//timex.h:extern int __REDIRECT_NTH (ntp_gettime, (struct ntptimeval *__ntv),
//timex.h:extern int ntp_gettimex (struct ntptimeval *__ntv) __THROW;
//timex.h:extern int ntp_adjtime (struct timex *__tntx) __THROW;

//uio.h:extern ssize_t readv (int __fd, const struct iovec *__iovec, int __count)
//uio.h:extern ssize_t writev (int __fd, const struct iovec *__iovec, int __count)
//uio.h:extern ssize_t preadv (int __fd, const struct iovec *__iovec, int __count,
//uio.h:extern ssize_t pwritev (int __fd, const struct iovec *__iovec, int __count,
//uio.h:extern ssize_t __REDIRECT (preadv, (int __fd, const struct iovec *__iovec,
//uio.h:extern ssize_t __REDIRECT (pwritev, (int __fd, const struct iovec *__iovec,
//uio.h:extern ssize_t preadv64 (int __fd, const struct iovec *__iovec, int __count,
//uio.h:extern ssize_t pwritev64 (int __fd, const struct iovec *__iovec, int __count,
//uio.h:extern ssize_t preadv2 (int __fp, const struct iovec *__iovec, int __count,
//uio.h:extern ssize_t pwritev2 (int __fd, const struct iovec *__iodev, int __count,
//uio.h:extern ssize_t __REDIRECT (pwritev2, (int __fd, const struct iovec *__iovec,
//uio.h:extern ssize_t __REDIRECT (preadv2, (int __fd, const struct iovec *__iovec,
//uio.h:extern ssize_t preadv64v2 (int __fp, const struct iovec *__iovec,
//uio.h:extern ssize_t pwritev64v2 (int __fd, const struct iovec *__iodev,

//utsname.h:extern int uname (struct utsname *__name) __THROW;
void utsname_ops()
{
    struct utsname buf;
    if (uname(&buf) < 0)
        printf("uname failed\n");
}

//vlimit.h:extern int vlimit (enum __vlimit_resource __resource, int __value) __THROW;
//vm86.h:extern int vm86 (unsigned long int __subfunction,
//vtimes.h:extern int vtimes (struct vtimes * __current, struct vtimes * __child) __THROW;
//wait.h:extern __pid_t wait (int *__stat_loc);
//wait.h:extern __pid_t waitpid (__pid_t __pid, int *__stat_loc, int __options);
//wait.h:extern int waitid (idtype_t __idtype, __id_t __id, siginfo_t *__infop,
//wait.h:extern __pid_t wait3 (int *__stat_loc, int __options,
//wait.h:extern __pid_t wait4 (__pid_t __pid, int *__stat_loc, int __options,
//xattr.h:extern int setxattr (const char *__path, const char *__name,
//xattr.h:extern int lsetxattr (const char *__path, const char *__name,
//xattr.h:extern int fsetxattr (int __fd, const char *__name, const void *__value,
//xattr.h:extern ssize_t getxattr (const char *__path, const char *__name,
//xattr.h:extern ssize_t lgetxattr (const char *__path, const char *__name,
//xattr.h:extern ssize_t fgetxattr (int __fd, const char *__name, void *__value,
//xattr.h:extern ssize_t listxattr (const char *__path, char *__list, size_t __size)
//xattr.h:extern ssize_t llistxattr (const char *__path, char *__list, size_t __size)
//xattr.h:extern ssize_t flistxattr (int __fd, char *__list, size_t __size)
//xattr.h:extern int removexattr (const char *__path, const char *__name) __THROW;
//xattr.h:extern int lremovexattr (const char *__path, const char *__name) __THROW;
//xattr.h:extern int fremovexattr (int __fd, const char *__name) __THROW;

int main(int argc, char *argv[])
{
    // swapoff is being used to determine the starting path
    const char swap_path[] = "/tmp/__nonexistant";
    swapoff(swap_path);

    if (argc > 1)
    {
        bool run_all = false;

        if (strcmp(argv[1], "unistd") == 0)
            unistd_ops();
        else if (strcmp(argv[1], "fncntl") == 0)
            fncntl_ops();
        else if (strcmp(argv[1], "utsname") == 0)
            utsname_ops();
        else if (strcmp(argv[1], "socket") == 0)
            socket_ops();
        else if (strcmp(argv[1], "sendfile") == 0)
            sendfile_ops();
        else if (strcmp(argv[1], "shm") == 0)
            shm_ops();
        else if (strcmp(argv[1], "swap") == 0)
            swap_ops();
        else if (strcmp(argv[1], "sched") == 0)
            sched_ops();
        else if (strcmp(argv[1], "stat") == 0)
            stat_ops();
        else
            run_all = true;

        if (!run_all)
        {
            return 0;
        }
    }

    unistd_ops();
    fncntl_ops();
    utsname_ops();
    socket_ops();
    swap_ops();
    sysinfo_ops();
    sched_ops();
    stat_ops();

    return 0;
}
