#include <linux/seccomp.h>
#include <linux/filter.h>
#include <sys/prctl.h>

void enable_seccomp() {
    prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);

    struct sock_filter filter[] = {
        BPF_STMT(BPF_LD | BPF_W | BPF_ABS,
                 offsetof(struct seccomp_data, nr)),
        BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, __NR_read, 0, 5),
        BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, __NR_write, 0, 4),
        BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, __NR_exit, 0, 3),
        BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, __NR_exit_group, 0, 2),
        BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, __NR_fstat, 0, 1),
        BPF_STMT(BPF_RET | BPF_K, SECCOMP_RET_ALLOW),
        BPF_STMT(BPF_RET | BPF_K, SECCOMP_RET_KILL)
    };

    struct sock_fprog prog {
        .len = (unsigned short)(sizeof(filter)/sizeof(filter[0])),
        .filter = filter
    };

    prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, &prog);
}
