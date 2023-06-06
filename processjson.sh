#!/bin/bash

myarray=(`find ./ -maxdepth 1 -name "*.json"`)
if [ ${#myarray[@]} -gt 0 ]; then 
    for i in "${myarray[@]}"; do
        python processjson.py $i
    done 
else 
    echo "No json files found"
fi