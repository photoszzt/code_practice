#!/bin/bash
num_columns=$(awk '{print NF}' file.txt | sort -nu | tail -n 1)
for i in `seq 1 ${num_columns}`; do
    awk -v i=$i '{ print $i }' file.txt | xargs echo
done
