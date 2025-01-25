#!/bin/bash

# Update package list
apt-get update

# Install zsh
apt-get install -y zsh

# Install Oh My Zsh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# Set zsh as default shell
chsh -s /bin/zsh

# Launch zsh
exec zsh