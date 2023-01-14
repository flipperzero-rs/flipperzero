#!/usr/bin/env python3
# Helper script for building/installing all examples.

import argparse
import logging
import os
from pathlib import Path, PurePosixPath
from subprocess import run

FLIPPERZERO_FIRMWARE = Path(os.environ.get('FLIPPERZERO_FIRMWARE', '../../flipperzero-firmware'))
PYTHON = 'python'
STORAGE_SCRIPT = FLIPPERZERO_FIRMWARE / 'scripts' / 'storage.py'
INSTALL_PATH = PurePosixPath('/ext/apps/Examples')


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('-i', '--install', action='store_true', help='Copy built projects to device')

    return parser.parse_args()


def main():
    args = parse_args()

    logging.basicConfig(level=logging.INFO)

    for path in Path.cwd().iterdir():
        if not path.is_dir() or not path.joinpath('Cargo.toml').exists():
            continue

        logging.info('Building %s', path.name)
        run(['cargo', 'build', '--release'], cwd=path, check=True)

        if args.install:
            # Assume that the binary has the name as the 
            filename = f'{path.name}.fap'
            binary = path / 'target' / 'thumbv7em-none-eabihf' / 'release' / filename
            target = INSTALL_PATH / filename

            logging.info('Copying %s to %s', binary, target)
            run([PYTHON, STORAGE_SCRIPT, 'send', os.fspath(binary), os.fspath(target)], check=True)


if __name__ == '__main__':
    main()
