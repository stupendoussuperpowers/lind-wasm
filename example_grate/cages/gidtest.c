#include <stdio.h>
#include <unistd.h>

int main(void) {
    int cageid = getpid();
    printf("--- GIDTEST %d ---\n", cageid);
    printf("getgid() = %d\n", (int)getgid());
    printf("--EXIT GIDTEST-\n");
    return 0;
}
