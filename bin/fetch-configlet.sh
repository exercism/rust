#!/bin/bash

LATEST=https://github.com/exercism/configlet/releases/latest

OS=$(
case $(uname) in
    (Darwin*)
        echo "mac";;
    (Linux*)
        echo "linux";;
    (Windows*)
        echo "windows";;
    (*)
        echo "linux";;
esac)

ARCH=$(
case $(uname -m) in
    (*64*)
        echo 64bit;;
    (*686*)
        echo 32bit;;
    (*386*)
        echo 32bit;;
    (*)
        echo 64bit;;
esac)

VERSION="$(curl --head --silent $LATEST | awk -v FS=/ '/Location:/{print $NF}' | tr -d '\r')"
URL=https://github.com/exercism/configlet/releases/download/$VERSION/configlet-$OS-${ARCH}.tgz

curl -s --location $URL | tar xz -C bin/
