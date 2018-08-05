from typing import Mapping, Set

def extensions(f):
    exts: Mapping[str, Set[str]] = {}

    for line in f:
        line = line.strip()
        if '.' in line:
            *base, ext = line.split('.')
            if len(ext) > 3:
                continue
            base = '.'.join(base).rstrip('-0123456789')
            if ext not in exts:
                exts[ext] = set()
            exts[ext].add(base)

    return exts
