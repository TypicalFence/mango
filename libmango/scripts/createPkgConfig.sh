#! /usr/bin/env bash

version=$(./scripts/getVersion.sh $1)

echo "prefix=$1"  > libmango.pc

cat template.pc | sed "s/VERSION/$version/g" >> libmango.pc

