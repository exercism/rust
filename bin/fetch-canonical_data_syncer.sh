#!/usr/bin/env bash

set -eo pipefail

readonly FINAL_RELEASE='https://api.github.com/repos/exercism/canonical-data-syncer/releases/33439231' # v0.17.0

case "$(uname)" in
    (Darwin*)   OS='mac'     ;;
    (Linux*)    OS='linux'   ;;
    (Windows*)  OS='windows' ;;
    (MINGW*)    OS='windows' ;;
    (MSYS_NT-*) OS='windows' ;;
    (*)         OS='linux'   ;;
esac

case "$OS" in
    (windows*) EXT='zip' ;;
    (*)        EXT='tgz' ;;
esac

case "$(uname -m)" in
    (*64*)  ARCH='64bit' ;;
    (*686*) ARCH='32bit' ;;
    (*386*) ARCH='32bit' ;;
    (*)     ARCH='64bit' ;;
esac

if [ -z "${GITHUB_TOKEN}" ]
then
    HEADER=''
else
    HEADER="authorization: Bearer ${GITHUB_TOKEN}"
fi

FILENAME="canonical_data_syncer-${OS}-${ARCH}.${EXT}"

get_url () {
    curl --header "$HEADER" -s --location "${FINAL_RELEASE}" |
        awk -v filename=$FILENAME '$1 ~ /browser_download_url/ && $2 ~ filename { print $2 }' |
        tr -d '"'
}

URL=$(get_url)

case "$EXT" in
    (*zip)
        curl --header "$HEADER" -s --location "$URL" -o bin/latest-canonical_data_syncer.zip
        unzip bin/latest-canonical_data_syncer.zip -d bin/
        rm bin/latest-canonical_data_syncer.zip
        ;;
    (*) curl --header "$HEADER" -s --location "$URL" | tar xz -C bin/ ;;
esac
