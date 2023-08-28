#!/usr/bin/env python3
# Helper script for linking and post-processing FAP binaries.

import os
import sys
from subprocess import run

TOOLS_PATH = '../tools'


def main():
    args = sys.argv[1:]
    print(args)

    # Run the linker with the given arguments.
    result = run(
        [
            'cargo',
            'run',
            '--quiet',
            '--release',
            '--bin',
            'fap-lld',
            '--',
        ] + args,
        cwd=os.path.join(os.path.dirname(__file__), TOOLS_PATH),
    )
    sys.exit(result.returncode)


if __name__ == '__main__':
    main()
