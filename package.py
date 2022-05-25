import enum
import json
import os
import platform
import subprocess
import shutil
import sys

from pathlib import Path
from typing import Dict, List, Set

PROJECT_ROOT = Path(os.path.dirname(os.path.abspath(__file__)))
BIN_DIR = PROJECT_ROOT / "bin"


class Os(enum.Enum):

    WINDOWS = "win"
    MACOS = "macos"
    LINUX = "linux"

    @staticmethod
    def get() -> 'Os':
        ps = platform.system().lower()
        if ps == "darwin":
            return Os.MACOS
        if ps == "windows":
            return Os.WINDOWS
        if ps == "linux":
            return Os.LINUX
        sys.exit(f"unknown platform: {ps}")

    @staticmethod
    def arch() -> str:
        arch = platform.machine()
        if arch == "x86_64" or arch == "AMD64":
            return "x64"
        sys.exit(f'unknown arch: {arch}')


class Build(enum.Enum):

    BLAS = "blas"
    UMFPACK = "umfpack"

    def package(self) -> str:
        ops = Os.get()
        arc = Os.arch()
        return f"olca-native-{self.value}-{ops.value}-{arc}"

    def target(self) -> Path:
        return PROJECT_ROOT / self.package() /\
            "src/main/resources/org/openlca/nativelib"

    def lib_name(self) -> str:
        name = "olcar" if self == Build.BLAS else "olcar_withumf"
        _os = Os.get()
        prefix = ""
        if _os != Os.WINDOWS:
            if not name.startswith("lib"):
                prefix = "lib"
        extension = "so"
        if _os == Os.MACOS:
            extension = "dylib"
        elif _os == Os.WINDOWS:
            extension = "dll"
        return f'{prefix}{name}.{extension}'

    def lib(self) -> Path:
        return BIN_DIR / self.lib_name()


class Node:
    """A node in a library-dependency graph."""

    def __init__(self, path: Path):
        self.path = path
        self.deps: List[Node] = []

    @property
    def name(self):
        return self.path.name


def get_julia_libdir() -> Path:
    """Read the Julia library path from the config file."""
    _os = Os.get()
    libdir = None
    config = PROJECT_ROOT / "config"
    with open(config, "r", encoding="utf-8") as f:
        libdir_key = _os.value + "-julia-lib-dir"
        for line in f.readlines():
            parts = line.split("=")
            if len(parts) < 2:
                continue
            key = parts[0].strip()
            if key != libdir_key:
                continue
            libdir = parts[1].strip()
            break
    if libdir is None:
        sys.exit(f"no Julia lib-folder defined for OS={_os} in config")
    path = Path(libdir)
    if not path.exists():
        sys.exit(f"the defined Julia library folder {path} does not exist")
    return path


def get_deps(lib_path: Path, libs: List[str]) -> List[str]:
    _os = Os.get()
    cmd = None
    path_str = str(lib_path.absolute())
    if _os == Os.MACOS:
        cmd = ["otool", "-L", path_str]
    if _os == Os.WINDOWS:
        cmd = ["Dependencies.exe", "-imports", path_str]
    if _os == Os.LINUX:
        cmd = ["ldd", path_str]
    if cmd is None:
        sys.exit("no deps command for os " + _os)

    # in Python 3.7 we have capture_output and text flags
    # but we make this compatible with Python 3.6 here
    proc = subprocess.run(cmd, stdout=subprocess.PIPE,
                          stderr=subprocess.PIPE)
    out = None
    if proc.stdout is not None:
        out = proc.stdout.decode(sys.stdout.encoding)
    elif proc.stderr is not None:
        out = proc.stderr.decode(sys.stderr.encoding)
    if out is None:
        return []
    deps = set()
    for line in out.splitlines():
        for lib in libs:
            if lib == lib_path.name:
                continue
            if lib not in line:
                continue
            # make sure that the name of the
            # library is not a part of another
            # library name that is also contained
            # in the line (e.g. `libcamd.so` and
            # `libcamd.so.2`)
            dep = lib
            for other in libs:
                if other == dep:
                    continue
                if dep not in other:
                    continue
                if other not in line:
                    continue
                dep = other
            deps.add(dep)
    return list(deps)


def get_dep_dag(root_lib: Path) -> Node:
    """Create the directed acyclic graph (DAG) of the dependencies. """
    libdir = get_julia_libdir()
    libs = os.listdir(libdir)
    handled: Set[str] = set()
    root = Node(root_lib)
    queue: List[Node] = [root]
    while len(queue) != 0:
        n: Node = queue.pop(0)
        for dep in get_deps(n.path, libs):
            dep_node = Node(libdir / dep)
            n.deps.append(dep_node)
            if dep in handled:
                continue
            handled.add(dep)
            queue.append(dep_node)
    return root


def topo_sort(dag: Node) -> List[str]:
    """Creates a topological order of the dependency graph in increasing
       dependency order."""

    # create dependency maps
    in_degrees: Dict[str, int] = {}
    dependents: Dict[str, List[str]] = {}
    queue: List[Node] = [dag]
    handled = set()
    while len(queue) != 0:
        n = queue.pop(0)    # type: Node
        if n.name in handled:
            continue
        handled.add(n.name)
        if n.name not in in_degrees:
            in_degrees[n.name] = 0
        for dep in n.deps:
            queue.append(dep)
            if dep.name not in in_degrees:
                in_degrees[dep.name] = 0
            depl = dependents.get(dep.name)
            if depl is None:
                depl = []
                dependents[dep.name] = depl
            depl.append(n.name)
            in_degrees[n.name] = in_degrees[n.name] + 1

    ordered = []
    while len(in_degrees) != 0:

        lib = None
        for _lib, _indeg in in_degrees.items():
            if _indeg == 0:
                lib = _lib
                break
        if lib is None:
            sys.exit("could not calculate dependency order;"
                     + " are there cycles in the dependencies?")

        ordered.append(lib)
        in_degrees.pop(lib)
        depl = dependents.pop(lib, None)
        if depl is None:
            continue
        for dependent in depl:
            in_degrees[dependent] -= 1  # in_degrees[dependent] - 1

    return ordered


def viz():
    dag = get_dep_dag(Build.UMFPACK.lib())
    print("digraph g {")
    queue = [dag]
    while len(queue) != 0:
        n = queue.pop(0)
        for dep in n.deps:
            print('  "%s" -> "%s";' % (n.name, dep.name))
            queue.append(dep)
    print("}")


def collect() -> List[str]:
    """Collect all library dependecies and sync them with the 'bin' folder."""
    dag = get_dep_dag(Build.UMFPACK.lib())
    libs = topo_sort(dag).copy()
    for lib in topo_sort(get_dep_dag(Build.UMFPACK.lib())):
        if lib not in libs:
            libs.append(lib)
    julia_dir = get_julia_libdir()
    for lib in libs:
        target = BIN_DIR / lib
        if target.exists():
            continue
        source = julia_dir / lib
        if not source.exists():
            print(f"ERROR: {source} does not exist")
            continue
        shutil.copyfile(source, target)
        print(f"  copied bin/{lib}")
    return libs


def make() -> list:
    print("create Maven resources")
    collect()

    def package(build: Build):
        print(f"  to package {build.package()}")

        # copy libraries
        libs = topo_sort(get_dep_dag(build.lib()))
        target = build.target()
        target.mkdir(exist_ok=True, parents=True)
        for lib in libs:
            shutil.copyfile(BIN_DIR / lib, target / lib)

        # write the index
        mods = ["blas"] if build == Build.BLAS else ["blas", "umfpack"]
        obj = {"modules": mods, "libraries": libs}
        with open(target / 'olca-native.json', 'w', encoding='utf-8') as out:
            json.dump(obj, out, indent='  ')

        # copy licenses
        lics = ["LICENSE_OPENBLAS"]
        if build == Build.UMFPACK:
            lics.append("LICENSE_UMFPACK")
        for lic in lics:
            shutil.copyfile(PROJECT_ROOT / lic, target / lic)

    package(Build.UMFPACK)
    package(Build.BLAS)


def clean():
    dirs = [BIN_DIR, Build.BLAS.target(), Build.UMFPACK.target()]
    for folder in dirs:
        if folder.exists():
            print(f'  clear folder {folder}')
            shutil.rmtree(folder, ignore_errors=True)
        folder.mkdir(exist_ok=True, parents=True)


def main():
    args = sys.argv
    cmd = "make" if len(args) < 2 else args[1]

    if cmd == "clean":
        print('delete build artifacts')
        clean()
        return

    # check if we need a build
    needs_build = False
    for build in Build:
        if not build.lib().exists():
            needs_build = True
            break
    if needs_build:
        print('compile native libraries')
        ext = "bat" if Os.get() == Os.WINDOWS else "sh"
        subprocess.call(PROJECT_ROOT / f"build.{ext}")
        for build in Build:
            if not build.lib().exists():
                sys.exit(f'failed to build library {build.lib_name()}')

    if cmd == "viz":
        print("create dependency visualization; "
              "e.g. render it on http://webgraphviz.com/")
        viz()
    elif cmd == "make":
        make()
    else:
        print(f'Error: unknown command {cmd}')


if __name__ == '__main__':
    main()
