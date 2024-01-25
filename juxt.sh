#!/usr/bin/env bash

#
# Desc:
#   change windows path style to linux style, eg: C:\path\to\any\place -> /c/path/to/any/place
#
# Usage:
#   path_style_win2linux 'path'
# 
function path_style_win2linux() {
    local path=${1:-''}
    echo $path | sed 's#\\#/#g' | sed -E 's#^([a-zA-Z]):#/\L\1#g'
}

#
# Desc:
#   change linux path style to windows style, eg: /c/path/to/any/place -> c:/path/to/any/place
#
# Usage:
#   path_style_linux2win 'path'
# 
function path_style_linux2win() {
    local path=${1:-''}
    echo $path | sed 's#\\#/#g' | sed -E 's#^/([a-zA-Z])(.*)#\1:\2#'
}

#
# Desc:
#   echo msg with color
#
# Usage:
#   color_msg r 'msg'
#
function color_msg() {
    local color=${1:?'(r)ed or (g)reen (b)lue (y)ellow (p)urple (c)yan'}

    if (( 2 > $# )); then
        return
    fi

    if [[ 'r' == $color ]]; then
        echo -e '\e[31m'${@:2}'\e[0m' # red
    elif [[ 'g' == $color ]]; then
        echo -e '\e[32m'${@:2}'\e[0m' # green
    elif [[ 'b' == $color ]]; then
        echo -e '\e[34m'${@:2}'\e[0m' # blue
    elif [[ 'y' == $color ]]; then
        echo -e '\e[33m'${@:2}'\e[0m' # yellow
    elif [[ 'p' == $color ]]; then
        echo -e '\e[35m'${@:2}'\e[0m' # purple
    elif [[ 'c' == $color ]]; then
        echo -e '\e[36m'${@:2}'\e[0m' # cyan
    else
        echo -e '\e[37m'${@:2}'\e[0m' # white
    fi
}

function color_ps3() {
    local prompt=${1:-'select '}
    local color=${2:-'g'}

    PS3=$(color_msg $color $prompt)
}

function input_info() {
    local retval=${1:?''}
    local prompt=${2:-'input: '}

    while true; do
        read -p "$prompt" info
        eval $retval='$info'
        break
    done
}

function select_option() {
    local retval=${1:?''}
    local prompt=${2:?''}
    shift 2
    local options=()
    until [[ $# -eq 0 ]]; do
        options+=("$1")
        shift
    done

    color_ps3 "$prompt(1-${#options[@]}):"
    select option in "${options[@]}"; do
        local index=$(($REPLY-1))
        if (( 0 <= $index && $index < ${#options[@]} )); then
            eval $retval='$index'
            break
        else
            color_msg r "illegal selection: $REPLY" 1>&2
            exit 1
        fi
    done
}

function select_x_proj_name() {
    local retval=${1:?''}
    local x_projs=(`ls $x_proj_root`)
    select_option retval_x_proj_index 'select x proj' "${x_projs[@]}"
    local x_proj_index=$retval_x_proj_index
    local x_proj_name=${x_projs[$x_proj_index]}
     eval $retval='$x_proj_name'
}

function select_x_crate_name() {
    local retval=${1:?''}
    select_x_proj_name retval_x_proj_name
    local x_proj_name=$retval_x_proj_name
    local x_crate_name=$x_proj_prefix$x_proj_name
    eval $retval='$x_crate_name'
}

function cargo_test_crate() {
    local crate_name=${1:?'need a crate name'}
    cargo test -p $crate_name
}

function cargo_test() {
    select_x_crate_name retval_x_crate_name
    local x_crate_name=$retval_x_crate_name
    cargo_test_crate $x_crate_name
}

function cargo_publish_crate() {
    local crate_name=${1:?'need a crate name'}
    cargo publish -n --registry crates-io -p $crate_name
    if (( 0 != $? )); then
        exit $?
    fi
}

function cargo_publish() {
    select_x_crate_name retval_x_crate_name
    local x_crate_name=$retval_x_crate_name
    cargo_publish_crate $x_crate_name
}

function cargo_new() {
    input_info retval_x_proj_name 'input x proj name: '
    local x_proj_name=$retval_x_proj_name
    local x_crate_name=$x_proj_prefix$x_proj_name
    local x_crate_path=$x_proj_root/$x_proj_name
    if [[ -d $x_crate_path ]]; then
        color_msg r "$x_crate_path existed"
        exit 0
    fi

    local crate_types=(bin lib)
    select_option retval_crate_type_index 'select crate type' "${crate_types[@]}"
    local crate_type_index=$retval_crate_type_index
    cargo new --vcs none --${crate_types[$crate_type_index]} $x_crate_path
    sed -i -E "s#(name\s+=\s+\")(.+?)(\")#\1$x_crate_name\3#g" $x_crate_path/Cargo.toml
    sed -i -E "s#(name\s+=\s+".+?")#\1\ndescription = \"just $x_proj_name, nothing else\"#g" $x_crate_path/Cargo.toml
    sed -i -E 's#(edition\s+=\s+".+?")#\1\nlicense-file.workspace = true#g' $x_crate_path/Cargo.toml
    sed -i -E "s#(edition\s+=\s+".+?")#\1\ndocumentation = \"https://docs.rs/$x_crate_name\"#g" $x_crate_path/Cargo.toml
    sed -i -E "s#(edition\s+=\s+".+?")#\1\nrepository = \"https://github.com/wolfired/juxt/tree/main/x/$x_proj_name\"#g" $x_crate_path/Cargo.toml
    sed -i -E "s#(edition\s+=\s+".+?")#\1\nhomepage = \"https://github.com/wolfired/juxt/tree/main/x/$x_proj_name\"#g" $x_crate_path/Cargo.toml

cat <<EOF > $x_crate_path/README.md
$x_crate_name
================

[![Crates.io Version](https://img.shields.io/crates/v/$x_crate_name?style=flat)](https://crates.io/crates/$x_crate_name)
[![docs.rs](https://img.shields.io/docsrs/$x_crate_name?style=flat&logo=docsdotrs)](https://docs.rs/$x_crate_name/latest/$x_crate_name/)
<!-- [![Codecov](https://img.shields.io/codecov/c/gh/wolfired/$proj_name?token=95IHYGJI9H&style=flat&logo=codecov)](https://app.codecov.io/gh/wolfired/$proj_name) -->

just $x_proj_name, nothing else
EOF

}

function code_coverage() {
    rm -rf $cov_dir/default_*.profraw
    rm -rf $profdata
    rm -rf $lcovdata

    RUSTFLAGS="-C instrument-coverage" \
    RUSTDOCFLAGS="-C instrument-coverage -Z unstable-options --persist-doctests target/debug/doctestbins" \
    LLVM_PROFILE_FILE=$llvm_profile_file cargo test

    if (( 0 != $? )); then
      exit $?
    fi

    $llvm_profdata merge --instr --sparse $cov_dir/default_*.profraw -o $profdata

    $llvm_cov show \
    -format=html \
    -output-dir=$cov_dir \
    -Xdemangler=rustfilt \
    $( \
      for file in \
        $( \
          RUSTFLAGS="-C instrument-coverage" \
          RUSTDOCFLAGS="-C instrument-coverage -Z unstable-options --persist-doctests target/debug/doctestbins" \
            cargo test --no-run --message-format=json \
              | gojq -r "select(.profile.test == true) | .filenames[]" \
              | grep -v dSYM - \
        ) \
        target/debug/doctestbins/*/rust_out*; \
      do \
        [[ -x $file ]] && printf "%s %s " -object $file; \
      done \
    ) \
    --instr-profile=$profdata \
    -show-line-counts-or-regions \
    -show-instantiations

    $llvm_cov export \
    -format=lcov \
    $( \
      for file in \
        $( \
          RUSTFLAGS="-C instrument-coverage" \
          RUSTDOCFLAGS="-C instrument-coverage -Z unstable-options --persist-doctests target/debug/doctestbins" \
            cargo test --no-run --message-format=json \
              | gojq -r "select(.profile.test == true) | .filenames[]" \
              | grep -v dSYM - \
        ) \
        target/debug/doctestbins/*/rust_out*; \
      do \
        [[ -x $file ]] && printf "%s %s " -object $file; \
      done \
    ) \
    -instr-profile=$profdata > $lcovdata
}

function codecov_upload() {
  local count=`git status --porcelain | grep -coP '^.+$'`
	local hashl=`git rev-parse @`
	local hashr=`git rev-parse @{u}`

    if (( 0 < $count )) || [[ $hashl != $hashr ]]; then
        color_msg r 'you need commit and push at first'
        return
    fi

    if [[ ! -n $CODECOV_TOKEN ]]; then
        color_msg r 'you need setup CODECOV_TOKEN env var'
        return
    fi

    if [[ -x `type codecov | grep -oP '[^\s]+$'` ]]; then
        codecov -t $CODECOV_TOKEN -f $lcovdata
    else
        bash <(curl -s https://codecov.io/bash) -t $CODECOV_TOKEN -f $lcovdata
    fi

    if (( 0 != $? )); then
        exit $?
    fi
}

function exit0() {
    exit 0
}

function select_action() {
    local actions=(
        cargo_test
        cargo_publish
        cargo_new
        code_coverage
        codecov_upload
        exit0
    )
    local action_labels=(
        'cargo test'
        'cargo publish'
        'cargo new'
        'code coverage'
        'codecov upload'
        'exit'
    )
    select_option retval_action_label_index 'select action' "${action_labels[@]}"
    local action_label_index=$retval_action_label_index
    ${actions[$action_label_index]}
}

function main() {
    proj_path=${proj_path:-$(dirname $(realpath $0))}
    proj_name=$(basename $proj_path)

    commit_hash=`rustc -vV | grep -oP '[0-9a-z]{40}'`
    # printf '%23s %s\n' $(color_msg y commit_hash:) $(color_msg g $commit_hash)

    sysroot=`rustc --print sysroot`
    # printf '%23s %s\n' $(color_msg y sysroot:) $(color_msg g $(path_style_win2linux $sysroot))

    target_triple=`basename $sysroot | grep -oP '(?<=-).*'`
    # printf '%23s %s\n' $(color_msg y target_triple:) $(color_msg g $target_triple)

    llvm_profdata=`path_style_win2linux $sysroot`/lib/rustlib/$target_triple/bin/llvm-profdata
    llvm_cov=`path_style_win2linux $sysroot`/lib/rustlib/$target_triple/bin/llvm-cov

    cov_dir=$proj_path/target/cov
    llvm_profile_file=$cov_dir/default_%m_%p.profraw
    profdata=$cov_dir/default_`basename $proj_path`.profdata
    lcovdata=$cov_dir/default_`basename $proj_path`.lcovdata

    x_proj_root=$proj_path/x
    x_proj_prefix=${proj_name}_

    if (( 0 == $# )); then
        select_action
    else
        $1
    fi
}
main $@
