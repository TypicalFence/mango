#!/usr/bin/env bash
convert -size 320x120 xc:magenta \
        -draw "fill red  circle 250,30 310,30 \
               fill green  circle 55,75 15,80 \
               font DejaVu-Sans font-size 72  decorate UnderLine \
               fill blue  stroke navy  stroke-width 2 \
               translate 10,110 rotate -15 text 0,0 ' TEST '" \
        -resize 1000 \
        test.jpg

