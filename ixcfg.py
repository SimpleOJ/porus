import os
import os.path
import re
from ix import has_to_recompile as ix_has_to_recompile, compile_file
from ix.utils import index_of, replace_ext
import subprocess

COVERAGE= os.environ.get('COVERAGE', '0').lower() in ('1', 'on')
BUILD_PATH = os.path.join(ROOTDIR, 'target/cov/build' if COVERAGE else 'target')
RUSTC = os.path.join(BUILD_PATH, 'rustc-shim.bat') if COVERAGE else 'rustc'

if COVERAGE:
    os.environ["CARGO_INCREMENTAL"]="0"
    os.environ["COV_PROFILER_LIB_NAME"]="@native"
    os.environ["COV_PROFILER_LIB_PATH"]="@native"
    os.environ["COV_RUSTC"]="rustc"
    os.environ["COV_BUILD_PATH"]=BUILD_PATH

DEBUG_EXTERNS = {
    "porus": os.path.join(BUILD_PATH, "debug/libporus.rlib"),
    "porus_macros": os.path.join(BUILD_PATH, "debug/libporus_macros.so"),
}

SOLUTION_PATTERN = r'^(?:[^/]+)/(?P<oj>\w+)(?:/.*)?/(?P<problem>[A-Za-z0-9_\-]+)\.rs(?:\.c)?$'

PRELUDE = b'''#![feature(proc_macro_non_items)]
#![feature(main)]
#![cfg_attr(not(debug_assertions), no_std)]
'''


def extern(externs):
    return sum([['--extern', '{}={}'.format(k,v)] for k,v in externs.items()], [])


def has_to_recompile(source, target, rlibs=DEBUG_EXTERNS):
    if ix_has_to_recompile(source, target):
        return True

    rlib = rlibs["porus"]
    if os.stat(rlib).st_mtime >= os.stat(target).st_mtime:
        return True

    return False


def get_rustc_argv(mode='debug', target=None):
    EXTERNS = {
        "porus": os.path.join(BUILD_PATH, "{}{}/libporus.rlib".format("" if target is None else target+"/", mode)),
        "porus_macros": os.path.join(BUILD_PATH, "{}/libporus_macros.so".format(mode)),
    }
    DEPS = ['-L', 'dependency='+os.path.join(BUILD_PATH, "{}/deps".format(mode))]
    if mode != 'release' and not COVERAGE:
        DEPS = ['-C' 'incremental='+os.path.join(BUILD_PATH, "{}/incremental".format(mode))] + DEPS

    VERBOSE_FLAG = '-v' if VERBOSE else '-q'
    MODE = [] if mode == 'debug' else ['--'+mode]
    TARGET = [] if target is None else ['--target', target]
    FEATURES = []
    if target is None:
        FEATURES.append("local-judge")
    if mode == 'release':
        FEATURES.append("online-judge")

    ARGV = ['cargo'] + (
        ['cov'] if COVERAGE else []
    ) + ['build', VERBOSE_FLAG, '--lib'] + MODE + TARGET + ["--features", ",".join(FEATURES)]

    if compile_file(ROOTDIR, ARGV, EXTERNS["porus"]) is None:
        return

    FEATURES = sum([["--cfg", 'feature="{}"'.format(f)] for f in FEATURES], [])
    FLAGS = os.environ.get("RUSTFLAGS", "-Z borrowck=mir -Z polonius").split(" ")
    DEBUG = ['-C', 'debuginfo=2'] if mode == 'debug' else []
    return [RUSTC, '-Z', 'external-macro-backtrace'] + DEBUG + FLAGS + TARGET + FEATURES + DEPS, EXTERNS


def read_source(filename):
    with open(filename, 'rb') as f:
        source = f.read()
    return PRELUDE + source


def get_compile_argv(filename):
    target = replace_ext(filename,"elf")

    if filename.endswith(".c"):
        return ['gcc', '-o', target, filename], target

    argv, externs = get_rustc_argv()
    if argv is None:
        raise Exception("failed to build library")

    return argv + extern(externs) + ['-o', target, "-"], target, read_source(filename)


def list_generated_files(filename):
    return [replace_ext(filename, ext) for ext in ["elf","bc","ll","s","rs.c","rs.elf","gcno","gcda"]]


def pick_env(envs):
    envs = [c for c in envs if c.lang == "C" and c.name in ("GCC", "MinGW")]
    envs.sort(key=lambda c: (index_of(['Linux','Windows'], c.os), index_of(['x86_64','x86'], c.arch)))
    if envs:
        return envs[0]


def get_llvm_target(env):
    return ( ({"x86": "i686", "x86_64": "x86_64"}[env.arch])
             + "-" +
             ({"Windows": "pc-windows", "Linux": "unknown-linux"}[env.os]) + "-gnu")


def generate_submission(source, llvm_target):
    argv, externs = get_rustc_argv('release', llvm_target)
    target = replace_ext(source, "s")

    argv = argv + extern(externs) + [
        "--crate-type", "cdylib",
        "--emit", "asm",
        "-C", "llvm-args=-disable-debug-info-print",
        "-C", "lto=fat",
        "-C", "opt-level=s",
        "-C", "panic=abort",
        "-o", target, "-"]

    if has_to_recompile(source, target, externs):
        if compile_file(ROOTDIR, argv, target, read_source(source)) is None:
            return None

    return target


def prepare_submission(envs, filename):
    env = pick_env(envs)
    if not env:
        return None

    llvm_target = get_llvm_target(env)

    if filename.endswith(".c"):
        llvm_target = None
        filename = filename[:-2]

    asm = generate_submission(filename, llvm_target)
    if asm is None:
        return None

    with open(asm,'rb') as f:
        code = f.read()

    if llvm_target is None:
        from ix.escape import escape_source
        code1 = escape_source(code)
        with open(filename+".c",'wb') as f:
            f.write(code1)

    return env, code
