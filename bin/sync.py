import os
import os.path
import sys

# 1. 获取文件名，最后修改日期

# 将最新的文件同步到旧的文件中

# 两种文件1. lib.rs  problem.rs


# 2. 读取文件内容

# 格式不一样 lib.rs要有定义struct  problem.rs要有上下的注释文件

def read_file(file_path):
    
    with open(file_path, 'r', encoding="utf-8") as f:
        code_lines = f.readlines()
    return code_lines

def get_code_from_sub(file_lines):
    start = 0
    end = 0
    for i, line in enumerate(file_lines):
        if line == "// @lc code=start\n":
            start = i
        if line == "// @lc code=end\n":
            end = i
    print(start, end)
    code_lines = file_lines[start + 1: end]
    return code_lines

def get_code_from_lib(file_lines):
    start = 0
    for i, line in enumerate(file_lines):
        if line == "pub  struct Solution;\n":
            start = i
            break
    print(start)
    code_lines = file_lines[start + 1: ]
    return code_lines

def write_file_lib(file_path, code_lines):
    with open(file_path, 'r', encoding="utf-8") as f:
        while True:
            line = f.readline()
            code_lines.insert(0, line)
            if line == "pub struct Solution;\n":
                break
    with open(file_path, 'w', encoding="utf-8") as f:
        f.writelines(code_lines)

def write_file_submit(file_path, code_lines):
    with open(file_path, 'r', encoding="utf-8") as f:
        i = 0
        while True:
            line = f.readline()
            code_lines.insert(i, line)
            i+=1
            if line == "// @lc code=start\n":
                break
            
    code_lines.append("// @lc code=end\n")
    with open(file_path, 'w', encoding="utf-8") as f:
        f.writelines(code_lines)

if __name__ == '__main__':
    args = sys.argv[1:]
    submit_file_name = args[0]
    lib_filename = "./src/lib.rs"
    lib_file_lines = read_file(lib_filename)
    submit_file_lines = read_file(submit_file_name)
    lib_t = os.path.getmtime(lib_filename)
    sub_t = os.path.getmtime(submit_file_name)
    if lib_t > sub_t:
        print(f"syn {lib_filename} to {submit_file_name}")
        code_lines = get_code_from_lib(lib_file_lines)
        write_file_submit(submit_file_name, code_lines)
    else:
        print(f"syn {submit_file_name} to {lib_filename}")
        code_lines = get_code_from_sub(submit_file_lines)
        write_file_lib(lib_filename, code_lines)
    print("同步的结果为:")
    print("".join(code_lines))
    print("sync success")

    
    
    
    