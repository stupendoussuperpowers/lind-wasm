/*
This is an auto-generated file. Do not modify.
*/

#include <stdlib.h>
#include <unistd.h>
#include <copy_data_between_cages.h>
#include <register_handler.h>
#include <sys/stat.h>
#include <sys/utsname.h>

#define thiscage	getpid()

#define COPY(src, dst, len, type) \
    copy_data_between_cages(thiscage, (src).cage, \
                          (src).value, (src).cage, \
                          (dst).value, (dst).cage, \
                          (len), (type))

typedef struct lvar_t {
	uint64_t value;
	uint64_t cage;
} lvar_t;

struct generic_args {
	uint64_t cageid;
	lvar_t arg1;
	lvar_t arg2;
	lvar_t arg3;
	lvar_t arg4;
	lvar_t arg5;
	lvar_t arg6;
};

void grate_init(void);
lvar_t l_alloc(int);
void l_free(lvar_t);
void register_handlers(int, int);
int max(int, int);

/* Types get attached down below */
__attribute__((weak)) int open_syscall(int cageid, char *pathname, int flags,
                                       mode_t mode);
__attribute__((weak)) int close_syscall(int cageid, int fd);
__attribute__((weak)) int read_syscall(int cageid, int fd, void *buf,
                                       size_t count);
__attribute__((weak)) int write_syscall(int cageid, int fd, void *buf,
                                        size_t count);
__attribute__((weak)) int uname_syscall(int cageid, struct utsname *buf);
__attribute__((weak)) int fstat_syscall(int cageid, int fd,
                                        struct stat *statbuf);
__attribute__((weak)) int xstat_syscall(int cageid, char *pathname,
                                        struct stat *statbuf);
