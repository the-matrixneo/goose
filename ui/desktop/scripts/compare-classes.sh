#!/bin/bash
# compare-classes.sh
# Usage: ./compare-classes.sh branch1 branch2

set -e

if [ $# -ne 2 ]; then
  echo "Usage: $0 <branch1> <branch2>"
  exit 1
fi

branch1=$1
branch2=$2
orig_branch=$(git rev-parse --abbrev-ref HEAD)

# Extract classes from branch1
git checkout $branch1 > /dev/null 2>&1
grep -rhoE "className=[\"'][^\"']*[\"']" src/ | sort | uniq > /tmp/classes-$branch1.txt

# Extract classes from branch2
git checkout $branch2 > /dev/null 2>&1
grep -rhoE "className=[\"'][^\"']*[\"']" src/ | sort | uniq > /tmp/classes-$branch2.txt

# Show the diff
diff -u /tmp/classes-$branch1.txt /tmp/classes-$branch2.txt | less

# Return to original branch
git checkout $orig_branch > /dev/null 2>&1

echo "\nComparison complete. Returned to $orig_branch." 