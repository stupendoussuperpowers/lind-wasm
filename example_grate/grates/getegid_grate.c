#include <stdio.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <string.h>

#include <lind_syscall_num.h>
#include "magic.h"


int grate_syscalls[] = {GETEGID_SYSCALL};
int grate_syscalls_len = 1;

void grate_init() {
	printf("EGID init'd\n");
}

void grate_destroy() {
	printf("EGID exiting.\n");
}

int getegid_syscall(int cageid) {
	return 556;
}

