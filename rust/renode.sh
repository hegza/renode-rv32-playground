#!/bin/sh
set -x #echo on

# Override platform here
PLATFORM=rv32imac-ibex

BASEDIR=$(dirname "$0")
BIN=${BIN=$1}

PLATFORM_PATH=$BASEDIR/../platforms/${PLATFORM=rv32imac-vexriscv}.repl

renode --console -e "set bin @$BIN; set platform_path @$PLATFORM_PATH; include @$BASEDIR/../scripts/start.resc"
