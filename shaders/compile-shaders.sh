#!/bin/sh

compile() {
    glslc -g -O $1 -o $1.spv
}

compile unlit.frag
