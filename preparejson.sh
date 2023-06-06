#!/bin/bash

myarray=(`find ./ -maxdepth 1 -name "*.json"`)
if [ ${#myarray[@]} -gt 0 ]; then 
    for i in "${myarray[@]}"; do
       # Replace first line with [{
       sed -i "1 s/.*/[{/" $i
       # Replace last line with }]
       sed -i "$ s/.*/}]/" $i
    done 
else 
    echo "No json files found"
fi



