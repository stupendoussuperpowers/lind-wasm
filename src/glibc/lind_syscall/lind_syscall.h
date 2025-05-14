int lind_syscall (unsigned int callnumber, unsigned long long callname, unsigned long long arg1, unsigned long long arg2, unsigned long long arg3, unsigned long long arg4, unsigned long long arg5, unsigned long long arg6);
int lind_register_syscall(uint64_t targetcage, 
    uint64_t targetcallnum, 
    uint64_t handlefunc_index_in_this_grate, 
    uint64_t this_grate_id, 
    uint64_t arg1_datatype, 
    uint64_t arg2_datatype, 
    uint64_t arg3_datatype, 
    uint64_t arg4_datatype, 
    uint64_t arg5_datatype, 
    uint64_t arg6_datatype);
