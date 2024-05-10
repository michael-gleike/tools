#!/bin/bash

target=aquascope-book
source=examples

# clear assets and md files to mdbook directory
cd $target
rm -f src/*md

printf "# Summary\n\n" > src/SUMMARY.md

# Create an empty array to store directory names
declare -a dir_names
for entry in "../$source"/*/
do
    if [ -d "$entry" ]; then
        dir_names+=($(basename "$entry"))
    fi
done

printf "Generating visualizations for the following examples: \n"
for dir_name in "${dir_names[@]}"; do
    printf "building %s..." $dir_name
    cp "../$source/$dir_name/$dir_name.md" "./src/$dir_name.md"
    echo "- [$dir_name](./$dir_name.md)" >> src/SUMMARY.md
    echo "done"
done

#mdbook build
echo use http://localhost:8080/
mdbook serve -n 0.0.0.0 -p 8080

# Option 1: Using tail -f (for most cases)
tail -f /dev/null

# Option 2: Infinite loop (alternative)
# while true; do sleep 1; done