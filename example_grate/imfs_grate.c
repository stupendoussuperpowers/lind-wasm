#include <stdio.h>
#include <sys/types.h>
#include <lind_syscall_num.h>
#include "magic.h"

#include "imfs.h"

int grate_syscalls[] = {OPEN_SYSCALL, CLOSE_SYSCALL, READ_SYSCALL, WRITE_SYSCALL};
int grate_syscalls_len = 4;

void grate_init() {
	imfs_init();
}

int _open_grate(struct open_args args) {
	uint64_t cageid = args.cageid;

	lvar_t pathname = args.pathname;
	lvar_t flags = args.flags;
	lvar_t mode = args.mode;
	
	lvar_t _path = l_alloc(256);

	// src, dst, len, type
	COPY(pathname, _path, 256, 1);

	int ifd = imfs_open(cageid, (char *)_path.value, flags.value, mode.value);

	l_free(_path);

	return ifd;
}

int _close_grate(struct close_args args) {
	uint64_t cageid = args.cageid;

	lvar_t fd = args.fd;

	return imfs_close(cageid, fd.value);
}

int _read_grate(struct read_args args) {
	uint64_t cageid = args.cageid;
	
	lvar_t fd = args.fd;
	lvar_t count = args.count;
	lvar_t buffer = args.buffer;

	lvar_t _buffer = l_alloc(count.value);
	
	int ret = imfs_read(cageid, fd.value, (char *)_buffer.value, count.value);

	if (buffer.value != 0) {
		COPY(_buffer, buffer, ret, 0);
	}

	l_free(_buffer);

	return ret;
}

int _write_grate(struct write_args args) {
	uint64_t cageid = args.cageid;

	lvar_t fd = args.fd;
	lvar_t count = args.count;
	lvar_t buffer = args.buffer;

	lvar_t _buffer = l_alloc(count.value);

	COPY(buffer, _buffer, count.value, 0);

	int ret;
	if (fd.value < 3) {
		ret = write(fd.value, (char *) _buffer.value, count.value);
	
	} else {
		ret = imfs_write(cageid, fd.value, (char *)_buffer.value, count.value);
	}

	l_free(_buffer);

	return ret;
}

