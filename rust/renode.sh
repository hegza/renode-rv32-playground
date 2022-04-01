#!/bin/sh
set -x #echo on

BASEDIR=$(dirname "$0")
BINARY=$1
renode --console -e "include @$BASEDIR/../scripts/start-rv32imac.resc; set bin @$BINARY; runMacro \$reset; start;"
