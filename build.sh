#!/bin/bash

FTP=false
RELEASE=false

for arg in "$@"; do
    if [[ $arg == "--ftp" ]]; then
        FTP=true
    elif [[ $arg == "--release" ]]; then
        RELEASE=true
    fi
done

if $RELEASE; then
    echo "Building in release mode..."
    cargo clean
    cargo psp --release
else
    echo "Building in debug mode..."
    cargo clean
    cargo psp
fi

if $FTP; then
    echo "Connecting to FTP server 192.168.0.133..."
    ftp -inv 192.168.0.133 <<EOF
anonymous
delete PSP/GAME/Ruty/EBOOT.PBP
put target/mipsel-sony-psp/debug/EBOOT.PBP PSP/GAME/Ruty/EBOOT.PBP
bye
EOF
    echo "Upload completed."
else
    echo "Option --ftp not specified. Skipping FTP upload."
fi
