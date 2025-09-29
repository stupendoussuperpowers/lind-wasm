#!/usr/bin/python3

import sys

top = """
"""

type_wrapper = """

struct {name}_args {{
        uint64_t cageid;

        {variables}
        {pad}
}};

"""

member_wrapper = ".{name} = {{ .value = arg{idx}, .cage = arg{idx}cage }}"

function_wrapper = """
int {name}_grate(uint64_t cageid, uint64_t arg1, uint64_t arg1cage, uint64_t arg2, uint64_t arg2cage, uint64_t arg3, uint64_t arg3cage, uint64_t arg4, uint64_t arg4cage, uint64_t arg5, uint64_t arg5cage, uint64_t arg6, uint64_t arg6cage){{

        struct {name}_args args = (struct {name}_args) {{
                .cageid = cageid,
                {members}
        }};

        return _{name}_grate(args);
}}
"""

function_dec = "int _{name}_grate(struct {name}_args);"


def generate_type(name, lvars):
    variables = ";\n\t".join([f"lvar_t {i}" for i in lvars]) + ";\n"

    num_pad = 6 - len(lvars)

    pad = "uint64_t " + \
        ", ".join([f"u{6-i}, uc{6-i}" for i in range(num_pad)]) + ";"

    return type_wrapper.format(name=name, variables=variables, pad=pad)


def generate_function(name, lvars):
    members = ",\n\t\t".join([member_wrapper.format(
        name=i, idx=idx+1) for idx, i in enumerate(lvars)])

    return function_wrapper.format(name=name, members=members)


def parse_descs(syscall_descs, header):
    ret_arr = []
    for i in syscall_descs.strip().split("\n"):
        i = i.replace(" ", "")
        tokens = i.split("=")
        ret_arr.append((tokens[0], tokens[1].split(",")))

    ret = "" if header else top

    for call in ret_arr:
        ret += generate_type(call[0], call[1]
                             ) if header else generate_function(call[0], call[1])
        ret += function_dec.format(name=call[0]) if header else ""

    return ret


if __name__ == "__main__":
    with open("syscall_descs", "r") as file:
        syscall_descs = file.read()
    if sys.argv[1] == "0":
        print(parse_descs(syscall_descs, True))
    else:
        print(parse_descs(syscall_descs, False))
