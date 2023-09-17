#!/usr/bin/bash
export DEBIAN_FRONTEND=noninteractive

sudo apt update
sudo apt upgrade
sudo apt install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev
