# yahv
yahv or yet another hex viewer is a small application to preview files in hexadecimal format.
It is meant to be used on embedded systems.

Currently, the project is in the WIP state.

# Usage
```
Usage:
         yahv [options] file
Options:
        -u              use upper case hex letters.
        -d              show offest in decimal.
        -g bytes        number of octets per group. Default 2.
        -h              print usage.
        -s seek         start at <seek>.
        -c cols         <cols> octets per line. Default 16.
        -l len          stop after <len> octets.
        -b              print octets in binary.
```

