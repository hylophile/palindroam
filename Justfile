default:
    @just --list --unsorted

# run the given recipes in parallel and log their output prefixed with the recipe's name
[positional-arguments]
parallog +args:
    #!/usr/bin/env bash
    trap "kill 0" EXIT
    align=$((1 + `printf "%s\n" "$@" | wc -L`))
    while (("$#")); do
        color=$((31 + ("$#" % 6)))
        prefix=`printf "\033[${color};m%+${align}s\033[0m" "$1"`
        FORCE_COLOR=1 just $1 2>&1 | sed "s/^/${prefix} │ /;" &
        shift
    done
    wait -n

[private]
pandoc:
    fd md notes \
        --exec sh -c '\
            pandoc {} \
            --lua-filter=typ_to_html.lua \
            -o app/src/lib/notes/$(basename {}).html'   

[private]
backlinks:
    uv run --with bs4 gen_backlinks.py

watch_notes:
    watchexec --no-vcs-ignore --watch ./notes 'just pandoc && just backlinks'

web_app:
    pnpm --prefix app run dev -- --open

# watch_notes and web_app
run:
    just parallog watch_notes web_app

clean:
		rm app/src/lib/backlinks.json
		rm app/src/lib/notes/*

