#!/usr/bin/env bash

echo "stdout"
echo "stderr" >&2

echo -en "test"
sleep 1
echo -e "\rreplaced"

exit 0
