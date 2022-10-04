#!/bin/bash

export PORT=8080

PROCESSES=()

_term() {
    echo "Caught SIGTERM"
    for child in "${PROCESSES[@]}"; do
        kill -TERM "$child" 2>/dev/null
    done 
}

_int() {
    echo "Caught SIGINT"
    for child in "${PROCESSES[@]}"; do
        kill -TERM "$child" 2>/dev/null
    done 
}

trap _term SIGTERM
trap _int SIGINT

pushd .;
cargo watch -x "run" &
SERVER_PROCESS=$!
PROCESSES+=($SERVER_PROCESS)
popd;

pushd front-end;
trunk watch &
FRONT_END_PROCESS=$!
PROCESSES+=($FRONT_END_PROCESS)
popd;

pushd front-end;
trunk watch &
BACK_OFFICE_PROCESS=$!
PROCESSES+=($BACK_OFFICE_PROCESS)
popd;

wait $BACK_OFFICE_PROCESS
echo "Done running actix and yew, bye"
