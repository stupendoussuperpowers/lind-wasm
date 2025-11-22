#include <errno.h>
#include <register_handler.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

// Dispatcher function
int pass_fptr_to_wt(uint64_t fn_ptr_uint, uint64_t cageid, uint64_t arg1,
                    uint64_t arg1cage, uint64_t arg2, uint64_t arg2cage,
                    uint64_t arg3, uint64_t arg3cage, uint64_t arg4,
                    uint64_t arg4cage, uint64_t arg5, uint64_t arg5cage,
                    uint64_t arg6, uint64_t arg6cage) {
  if (fn_ptr_uint == 0) {
    return -1;
  }

  int (*fn)(uint64_t) = (int (*)(uint64_t))(uintptr_t)fn_ptr_uint;

  return fn(cageid);
}

// Function ptr and signatures of this grate
int geteuid_grate(uint64_t);

int geteuid_grate(uint64_t cageid) {
	return 10;
}

// Main function will always be same in all grates
int main(int argc, char *argv[]) {
  // Should be at least two inputs (at least one grate file and one cage file)
  if (argc < 2) {
    fprintf(stderr, "Usage: %s <cage_file> <grate_file> <cage_file> [...]\n",
            argv[0]);
    exit(EXIT_FAILURE);
  }

  int grateid = getpid();

  pid_t pid = fork();
  if (pid < 0) {
    perror("fork failed");
    exit(EXIT_FAILURE);
  } else if (pid == 0) {
    int cageid = getpid();
    
    uint64_t fn_ptr_addr = (uint64_t)(uintptr_t)&geteuid_grate;
    int ret = register_handler(cageid, 107, 1, grateid, fn_ptr_addr);

    if (execv(argv[1], &argv[1]) == -1) {
      perror("execv failed");
      exit(EXIT_FAILURE);
    }
  }

  int status;
  while (wait(&status) > 0) {
  }

  return 0;
}

/*
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>

#include "magic.h"
#include <lind_syscall_num.h>

int grate_syscalls[] = {GETEUID_SYSCALL};
int grate_syscalls_len = 1;

void grate_init() {
        printf("EUID initing...\n");
}

void grate_destroy() {
        printf("EUID exiting.\n");
}

int geteuid_syscall(int cageid) {
        return 546;
}
*/
