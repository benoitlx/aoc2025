[private]
default:
    @just --list --justfile {{justfile()}}

# Examples:
#   just build day5          # builds day5/secret_entrance.ml
#   just build day5 2        # builds day5/secret_entrance_part2.ml
build day part="":
    #!/usr/bin/env bash
    mkdir -p bins
    if [ -z "{{part}}" ]; then
    ocamlc -o bins/{{day}} {{day}}/*.ml
    else
    ocamlc -o bins/{{day}} {{day}}/*_part{{part}}.ml
    fi

# Run a given day and optional part
run day part="":
    just build {{day}} {{part}}
    ./bins/{{day}}


# remove binaries
clean:
    rm -r bins/
