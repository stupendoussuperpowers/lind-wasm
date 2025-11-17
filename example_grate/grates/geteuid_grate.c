#include <stdio.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <string.h>

#include <lind_syscall_num.h>
#include "magic.h"

int grate_syscalls[] = {GETEUID_SYSCALL};
int grate_syscalls_len = 1;

void grate_init() { printf("EUID initing...\n"); }

void grate_destroy() { printf("EUID exiting.\n"); }

int geteuid_syscall(int cageid) { return 546; }
