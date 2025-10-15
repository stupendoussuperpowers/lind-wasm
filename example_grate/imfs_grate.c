#include <stdio.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <string.h>

#include <lind_syscall_num.h>
#include "magic.h"

#include "imfs.h"


int grate_syscalls[] = {OPEN_SYSCALL, XSTAT_SYSCALL, WRITE_SYSCALL, READ_SYSCALL, CLOSE_SYSCALL};
int grate_syscalls_len = 5;

void grate_init() {
	imfs_init();
}

int open_syscall(int cageid, char* pathname, int flags, mode_t mode) {
	int ret = imfs_open(cageid, pathname, flags, mode);
	fprintf(stderr, "\nOPEN(%s, %d, %d) = %d\n", pathname, flags, mode, ret);
	return ret;
}

int close_syscall(int cageid, int fd) {
	int ret = imfs_close(cageid, fd);
	fprintf(stderr, "\nCLOSE(%d) = %d\n", fd);
	return ret;
}

int write_syscall(int cageid, int fd, void *buf, size_t count) {
	if (fd < 3) {
		return write(2, buf, count);
	}
	int ret = imfs_write(cageid, fd, buf, count);
	fprintf(stderr, "\nWRITE(%d, %s, %d) = %d\n", fd, buf, count, ret);
	return ret;
}

int read_syscall(int cageid, int fd, void *buf, size_t count) {
	int ret = imfs_read(cageid, fd, buf, count);
	fprintf(stderr, "\nREAD(%d, %s, %d) = %d\n", fd, buf, count, ret);
	return ret;
}

int xstat_syscall(int cageid, char *pathname, struct stat *statbuf) {
	int ret = imfs_stat(cageid, pathname, statbuf);
	fprintf(stderr, "\nXSTAT(%s, %p) = %d\n", pathname, statbuf, ret);
       	return ret;	
}

