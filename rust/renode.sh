#!/bin/sh
set -x #echo on

BINARY=$1
renode --console -e "include @../scripts/start-rv32imac.resc; set bin @$BINARY; runMacro \$reset; start;"
