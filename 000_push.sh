#!/bin/bash

# Author        : Eshanized <m.eshanized@gmail.com>
# Author URI    : https://tonmoyinfrastructure.github.io/eshanized/

################################ NOTE ################################
#!!!!!!!!!!!!! Exexcute all the scripts at your own risk !!!!!!!!!!!!!
# I have written the push script for Arch Linux and other Arch Based #
# Linux Distribution. So this script will only work in ArchLinux and #
# Arch based Linux Distribution. You may customize it according to   #
# your Distribution.                                                 #
######################################################################

# ---> First We will check for commitizen is installed or not
check_commitizen() {
    if ! pacman -Qq commitizen-go &> /dev/null; then
        echo "Commitizen is not installed. Please install it using 'yay -S commitizen-go'." >&2
        exit 1
    fi
}

# Function to stage, commit, and push changes
push_to_github() {
    git add .
    git cz
    git push origin master
}

# Main Function
main() {
    check_commitizen
    push_to_github
}

main