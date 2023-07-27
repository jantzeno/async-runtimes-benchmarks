#!/bin/bash

myarray=(`find ./json -maxdepth 1 -wholename "*.json"`)
if [ ${#myarray[@]} -gt 0 ]; then 
    for i in "${myarray[@]}"; do
        python processjson.py $i
    done 
else 
    echo "No json files found"
fi