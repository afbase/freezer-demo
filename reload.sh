#!/bin/sh

FILE_SERVER="run/server/server"
FILE_CLIENT="run/client/client"
if [ -f "$FILE_SERVER" ]; then
    rm $FILE_SERVER
    echo "old $FILE_SERVER removed."
fi

if [ -f "$FILE_CLIENT" ]; then
    rm $FILE_CLIENT
    echo "old $FILE_CLIENT removed."
fi

BUILD_SERVER="target/release/examples/server"
BUILD_CLIENT="target/release/examples/client"

if [ -f "$BUILD_SERVER" ]; then
    cp $BUILD_SERVER run/server
    echo "new server moved"
fi

if [ -f "$BUILD_CLIENT" ]; then
    cp $BUILD_CLIENT run/client
    echo "new client moved"
fi