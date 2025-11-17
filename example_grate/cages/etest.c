#include <stdio.h>
#include <unistd.h>

int main(void) {
    int cageid = getpid();
    printf("-------ETEST %d ------\n", cageid);
    printf("getegid() = %d\n", (int)getegid());
    printf("geteuid() = %d\n", (int)geteuid());
    printf("----(EXIT) ETEST----\n");
    return 0;
}
