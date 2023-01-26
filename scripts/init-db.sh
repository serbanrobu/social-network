#!/usr/bin/env bash

set -x # All executed commands are printed to the terminal
set -e # Set the shell to exit as soon as the first error is encountered
set -o pipefail

if ! [ -x "$(command -v sqlx)" ]; then
	echo >&2 "Error: sqlx is not installed."
	exit 1
fi

sqlx database create

sqlx migrate run
