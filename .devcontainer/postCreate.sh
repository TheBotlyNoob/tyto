#!/bin/bash

sudo apt-get update

sudo apt-get install curl qemu qemu-system -y

curl -fsSL https://sh.rustup.rs | bash -s -- -y
