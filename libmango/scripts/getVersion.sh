#! /usr/bin/env bash

# get the text of the cargo manifest
cat Cargo.toml | 
    # get the version field
    grep version | 
        # split the line with the version at the = into 2 
        tr "=" "\n" |
            # only keep the second line
            sed -n 2p |
                # we should now have something like ' "1.0.0"'
                # remove the whitespace and "
                sed 's/ \|"//g'
