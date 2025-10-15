/*
This is an auto-generated file. Do not modify.
*/

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <register_handler.h>
#include <sys/types.h>
#include <sys/wait.h>

#include <sys/stat.h>
#include <sys/utsname.h>

#include <lind_syscall_num.h>
#include "magic.h"

extern int grate_syscalls[];
extern size_t grate_syscalls_len;

lvar_t l_alloc(int size) {
	char *val = malloc(size);
	return (lvar_t) {
		.value = (uint64_t)val,
		.cage = thiscage,
	};
}

void l_free(lvar_t ptr) {
	free((char *)ptr.value);
}

void register_handlers(int cageid, int grateid) {
	int ret;
	for(int i=0; i < grate_syscalls_len; i++) {
		ret = register_handler(cageid, grate_syscalls[i], grateid, grateid);
		printf("Registered: %d\n", grate_syscalls[i]);
	}
}

int max(int a, int b) {
	return a > b ? a : b;
}

// Main function will always be same in all grates
int main(int argc, char *argv[]) {
    if (argc < 2) {
        fprintf(stderr, "Usage: %s <cage_file> <grate_file> <cage_file> [...]\n", argv[0]);
        exit(EXIT_FAILURE);
    }
    int grateid = getpid();
    printf("grateid: %d\n", grateid);
    for (int i = 1; i < (argc < 3 ? argc : 3); i++) {
        pid_t pid = fork();
        if (pid < 0) {
            perror("fork failed");
            exit(EXIT_FAILURE);
        } else if (pid == 0) {
            // will always be cage.
            if (i % 2 != 0) {
                int cageid = getpid();
               	printf("cageid: %d\n", cageid); 
		register_handlers(cageid, grateid);	
	    }

	    printf("Execing %s\n", argv[i]);
            if (execv(argv[i], &argv[i]) == -1) {
		printf("Execv failed!\n");
                exit(EXIT_FAILURE);
            }
	} else {
		grate_init();
	}
    }

    int status;
    while (wait(&status) > 0) {
        printf("[Grate | geteuid] terminated, status: %d\n", status);
    	grate_destroy();
    }

    return 0;
}


int open_grate(uint64_t cageid, uint64_t arg1, uint64_t arg1cage, uint64_t arg2,
	       uint64_t arg2cage, uint64_t arg3, uint64_t arg3cage,
	       uint64_t arg4, uint64_t arg4cage, uint64_t arg5,
	       uint64_t arg5cage, uint64_t arg6, uint64_t arg6cage) {
	if (!open_syscall) {
		return -1;
	}

	mode_t mode = arg3;

	int flags = arg2;

	char *pathname = malloc(256);

	if (pathname == NULL) {
		perror("malloc failed");
		exit(EXIT_FAILURE);
	}

	copy_data_between_cages(thiscage, arg1cage, arg1, arg1cage,
				(uint64_t)pathname, thiscage, 256, 1);

	int ret = open_syscall(cageid, pathname, flags, mode);

	free(pathname);

	return ret;
}


int close_grate(uint64_t cageid, uint64_t arg1, uint64_t arg1cage,
		uint64_t arg2, uint64_t arg2cage, uint64_t arg3,
		uint64_t arg3cage, uint64_t arg4, uint64_t arg4cage,
		uint64_t arg5, uint64_t arg5cage, uint64_t arg6,
		uint64_t arg6cage) {
	if (!close_syscall) {
		return -1;
	}

	int fd = arg1;

	int ret = close_syscall(cageid, fd);

	return ret;
}


int read_grate(uint64_t cageid, uint64_t arg1, uint64_t arg1cage, uint64_t arg2,
	       uint64_t arg2cage, uint64_t arg3, uint64_t arg3cage,
	       uint64_t arg4, uint64_t arg4cage, uint64_t arg5,
	       uint64_t arg5cage, uint64_t arg6, uint64_t arg6cage) {
	if (!read_syscall) {
		return -1;
	}

	size_t count = arg3;

	void *buf = malloc(count);

	if (buf == NULL) {
		perror("malloc failed");
		exit(EXIT_FAILURE);
	}

	copy_data_between_cages(thiscage, arg2cage, arg2, arg2cage,
				(uint64_t)buf, thiscage, count, 0);

	int fd = arg1;

	int ret = read_syscall(cageid, fd, buf, count);

	if (arg2 != 0) {
		copy_data_between_cages(thiscage, arg2cage, (uint64_t)buf,
					thiscage, arg2, arg2cage, count, 0);
	}

	free(buf);

	return ret;
}


int write_grate(uint64_t cageid, uint64_t arg1, uint64_t arg1cage,
		uint64_t arg2, uint64_t arg2cage, uint64_t arg3,
		uint64_t arg3cage, uint64_t arg4, uint64_t arg4cage,
		uint64_t arg5, uint64_t arg5cage, uint64_t arg6,
		uint64_t arg6cage) {
	if (!write_syscall) {
		return -1;
	}

	size_t count = arg3;

	void *buf = malloc(count);

	if (buf == NULL) {
		perror("malloc failed");
		exit(EXIT_FAILURE);
	}

	copy_data_between_cages(thiscage, arg2cage, arg2, arg2cage,
				(uint64_t)buf, thiscage, count, 0);

	int fd = arg1;

	int ret = write_syscall(cageid, fd, buf, count);

	free(buf);

	return ret;
}


int uname_grate(uint64_t cageid, uint64_t arg1, uint64_t arg1cage,
		uint64_t arg2, uint64_t arg2cage, uint64_t arg3,
		uint64_t arg3cage, uint64_t arg4, uint64_t arg4cage,
		uint64_t arg5, uint64_t arg5cage, uint64_t arg6,
		uint64_t arg6cage) {
	if (!uname_syscall) {
		return -1;
	}

	struct utsname *buf = malloc(sizeof(struct utsname));

	if (buf == NULL) {
		perror("malloc failed");
		exit(EXIT_FAILURE);
	}

	copy_data_between_cages(thiscage, arg1cage, arg1, arg1cage,
				(uint64_t)buf, thiscage, sizeof(struct utsname),
				0);

	int ret = uname_syscall(cageid, buf);

	if (arg1 != 0) {
		copy_data_between_cages(thiscage, arg1cage, (uint64_t)buf,
					thiscage, arg1, arg1cage,
					sizeof(struct utsname), 0);
	}

	free(buf);

	return ret;
}


int fstat_grate(uint64_t cageid, uint64_t arg1, uint64_t arg1cage,
		uint64_t arg2, uint64_t arg2cage, uint64_t arg3,
		uint64_t arg3cage, uint64_t arg4, uint64_t arg4cage,
		uint64_t arg5, uint64_t arg5cage, uint64_t arg6,
		uint64_t arg6cage) {
	if (!fstat_syscall) {
		return -1;
	}

	struct stat *statbuf = malloc(sizeof(struct stat));

	if (statbuf == NULL) {
		perror("malloc failed");
		exit(EXIT_FAILURE);
	}

	copy_data_between_cages(thiscage, arg2cage, arg2, arg2cage,
				(uint64_t)statbuf, thiscage,
				sizeof(struct stat), 0);

	int fd = arg1;

	int ret = fstat_syscall(cageid, fd, statbuf);

	if (arg2 != 0) {
		copy_data_between_cages(thiscage, arg2cage, (uint64_t)statbuf,
					thiscage, arg2, arg2cage,
					sizeof(struct stat), 0);
	}

	free(statbuf);

	return ret;
}


int xstat_grate(uint64_t cageid, uint64_t arg1, uint64_t arg1cage,
		uint64_t arg2, uint64_t arg2cage, uint64_t arg3,
		uint64_t arg3cage, uint64_t arg4, uint64_t arg4cage,
		uint64_t arg5, uint64_t arg5cage, uint64_t arg6,
		uint64_t arg6cage) {
	if (!xstat_syscall) {
		return -1;
	}

	struct stat *statbuf = malloc(sizeof(struct stat));

	if (statbuf == NULL) {
		perror("malloc failed");
		exit(EXIT_FAILURE);
	}

	copy_data_between_cages(thiscage, arg2cage, arg2, arg2cage,
				(uint64_t)statbuf, thiscage,
				sizeof(struct stat), 0);

	char *pathname = malloc(256);

	if (pathname == NULL) {
		perror("malloc failed");
		exit(EXIT_FAILURE);
	}

	copy_data_between_cages(thiscage, arg1cage, arg1, arg1cage,
				(uint64_t)pathname, thiscage, 256, 1);

	int ret = xstat_syscall(cageid, pathname, statbuf);

	if (arg2 != 0) {
		copy_data_between_cages(thiscage, arg2cage, (uint64_t)statbuf,
					thiscage, arg2, arg2cage,
					sizeof(struct stat), 0);
	}

	free(statbuf);
	free(pathname);

	return ret;
}

