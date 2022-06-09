#!/bin/bash

VERSION=$(echo $1 | sed 's/v//')
shift

function retry {
    for i in {0..10}; do
        $@
        if [ $? -eq 0 ]; then
            exit 0
        fi
        sleep 3
    done
    exit 1
}


declare -a PROJECTS=(
    revonslang_codegen
    revonslang_base
    revonslang_parser
    revonslang_check
    revonslang_completion
    revonslang_vm
    revonslang_format
    revonslang
    revonslang_c-api
    revonslang_doc
    revonslang_repl
)

for PROJECT in "${PROJECTS[@]}"
do
    PROJECT_PATH=$(echo "$PROJECT" | sed 's/revonslang_//' | sed 's/revonslang/./')

    if ! (./scripts/sync_publish.sh "$(pwd)/${PROJECT_PATH}" -f "$@"); then
        echo "Failed to publish $PROJECT_PATH"
        exit 1
    fi
done
