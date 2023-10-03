#!/bin/bash

# Detect the operating system
os=$(uname)

# Get the network interface and IP address based on the OS
if [ "$os" == "Linux" ]; then
  interface=$(ip route | grep default | awk '{print $5}')
  ip_address=$(ip addr show $interface | grep 'inet ' | awk '{print $2}' | cut -d/ -f1)
elif [ "$os" == "Darwin" ]; then  # macOS
  interface=$(route -n get default | grep 'interface:' | awk '{print $2}')
  ip_address=$(ifconfig $interface | grep 'inet ' | awk '{print $2}')
else
  echo "Unsupported OS"
  exit 1
fi

# Copy the example env file to .env if it doesn't exist
[ ! -f .env ] && cp env.example .env

# Remove the existing DATABASE_URL line
if [ "$os" == "Linux" ]; then
  sed -i '/^DATABASE_URL/d' .env
elif [ "$os" == "Darwin" ]; then
  sed -i '' '/^DATABASE_URL/d' .env
fi
