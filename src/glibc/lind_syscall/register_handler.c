#include <stdint.h> // For uint64_t definition
#include <syscall-template.h> // For make_syscall definition

int register_handler(uint64_t targetcage, uint64_t targetcallnum, uint64_t handlefunc_index_in_this_grate, uint64_t this_grate_id, uint64_t arg1_datatype, uint64_t arg2_datatype, uint64_t arg3_datatype, uint64_t arg4_datatype, uint64_t arg5_datatype, uint64_t arg6_datatype) {
    // return MAKE_SYSCALL(400, "register_handler", targetcage, targetcallnum, handlefunc, handlefunccage, NOTUSED, NOTUSED);
    return REGISTER_HANDLER_SYSCALL(targetcage, targetcallnum, handlefunc_index_in_this_grate, this_grate_id, arg1_datatype, arg2_datatype, arg3_datatype, arg4_datatype, arg5_datatype, arg6_datatype);
}
