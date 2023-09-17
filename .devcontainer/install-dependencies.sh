#!/usr/bin/bash
sudo apt update
DEBIAN_FRONTEND=noninteractive sudo apt upgrade -y
DEBIAN_FRONTEND=noninteractive sudo apt install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev
