#!/usr/bin/env bash

BATCH_SIZE=1
BLANK_CHAR='.'
FULL_CHAR='#'

process_files() {
    local files=("$@")
    echo "Processing ${files[@]}"
    sleep 0.01
}

progress_bar() {
    local num_elements=$1
    local current=$2

    local perc_done=$((current * 100 / num_elements))

    local suffix=" $current/$num_elements ($perc_done%)"
    local max_columns=$((COLUMNS - ${#suffix} - 2))

    local num_fill=$((perc_done * max_columns / 100))

    local s="["
    local i
    for ((i = 0 ; i < num_fill; i++))
    do
        s+=$FULL_CHAR
    done

    for ((i = num_fill; i < max_columns; i++))
    do
        s+=$BLANK_CHAR
    done
    s+="]"
    s+=$suffix

    printf '\e7'                # save cursor
    printf '\e[%d;%dH' $LINES 0 # move to bottom
    printf '\e[2K'              # clear line
    echo -en "$s"               # print bar
    printf '\e8'                # restore cursor

}

init_term(){
    printf '\n'                             # space for progress bar
    printf '\e7'                            # save cursor
    printf '\e[%d;%dr' 0 "$((LINES - 1))"   # set scrollable region (https://ghostty.org/docs/vt/csi/decstbm)
    printf '\e8'                            # restore cursor
    printf '\e[1A'                          # move cursor up
}

deinit_term() {
    printf '\e7'                # save cursor
    printf '\e[%d;%dr' 0 $LINES # reset scrollable region
    printf '\e[%d;%dH' $LINES 0 # move cursor to bottom
    printf '\e[2K'              # clear line
    printf '\e8'                # reset cursor
}

fatal() {
    echo '[FATAL]' "$@" >&2
    exit 1
}

main() {
    local OPTARG OPTIND opt 
    while getopts 'b:c:f:t' opt
    do
        case "$opt" in 
            b) BATCH_SIZE=$OPTARG;;
            c) BLANK_CHAR=$OPTARG;;
            f) FULL_CHAR=$OPTARG;;
            t) echo "test"; exit 1;;
            *) fatal 'bad option';;
        esac
    done

    shopt -s globstar nullglob checkwinsize

    # necessary to be able to use LINES nad COLS from checkwinsize
    # for some reason, those global variables get set only 
    # when you call an external command
    (:)

    trap deinit_term exit
    trap init_term winch
    init_term

    local files=(./**/*cache)
    local len=${#files[@]}

    local i=0
    local file
    for ((i = 0; i < len; i += BATCH_SIZE))
    do
        progress_bar "$len" "$((i+1))"
        process_files "${files[@]:i:BATCH_SIZE}"
    done
    progress_bar "$len" "$len"
}

main "$@"