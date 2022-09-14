#!/bin/bash

sudo apt-get update

sudo apt-get install curl

curl -fsSL https://sh.rustup.rs | bash -s -- -y
