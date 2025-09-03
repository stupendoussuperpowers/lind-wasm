## Running TCC with an IMFS Grate

This folder contains all the files required to run tcc using an in-memory file system grate. 

### Files 

- `imfs.*` : IMFS implementaiton. 
- `open_grate.c` : Grate file
- `preloads` : List of files that need to be loaded into memory in order for tcc to be able to compile C code. This includes headers, object files and archives. All the files listed here must be present in `LIND_FS_ROOT` (defaults to `src/RawPOSIX/tmp`). 

### Prerequisites 

- Build glibc and wasmtime. 
- Build `tcc.wasm` using [these instructions](https://github.com/Lind-Project/TriSeal/issues/9) and place it in `LIND_FS_ROOT`. 
- Place all the files listed in `preloads`. 
- Place `hello.c` into `LIND_FS_ROOT`. This is a C source file that can use `stdio.h` as an include. 
- You can extract [this tar](https://github.com/stupendoussuperpowers/lind-wasm/releases/download/imfs_tcc/sys-tcc-imfs.tar.gz) into `LIND_FS_ROOT` which contains the aforementioned files. 

### Building & Running 

- Build using `./compile_open.sh`
- Run using `./run_imfs_tcc.sh`

### Notes 

- Ths `stdout` and `stderr` for `tcc` are redirected to `LIND_FS_ROOT/host_write`. 
- If you run into any issues while running this code, please also attach `LIND_FS_ROOT/failed_opens.log` and `LIND_FS_ROOT/preloads.log` while reporting it. Most tcc errors are caused by a file not being properly loaded into memory and these two files log information about that. 

### Changes to wasmtime/rawposix 

This branch contains a few changes to wasmtime and rawposix which are required to run this grate. 

- Add support for stat, and fstat syscall. 
- Bypass a pointer dereference bug by rounding up setjmp/longjmp arguments, see [commit](https://github.com/stupendoussuperpowers/lind-wasm/commit/2ebfa632b568716a18a7c922c0e6186f0bc51c11) and [issue]()
