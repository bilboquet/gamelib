#!/bin/bash

NODE_SHAPE=circle
NODE_SHAPE=point


for file in *.dot; do
    filename="${file%.*}"
    echo $filename
    # dot -Tsvg -o "$filename.svg" "$file"
    dot -Tpng -O "$file" -Nshape=$NODE_SHAPE -Nwidth=".3" -Glabel="$filename" -Glabelloc=b

    #eog master.png

done
