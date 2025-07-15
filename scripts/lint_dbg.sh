#!/bin/sh

if [ ! -z "$(grep -rnc 'dbg!' crates | grep -v ':0$')" ]
then
    echo "Fatal: found lingering dbg! statements!"
    exit 1
fi
