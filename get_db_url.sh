#!/bin/bash
interface=$(ip route | grep default | awk '{print $5}')
ip_address=$(ip addr show $interface | grep 'inet ' | awk '{print $2}' | cut -d/ -f1)

# Copy the example env file to .env if it doesn't exist
[ ! -f .env ] && cp env.example .env

# Remove the existing DATABASE_URL line
sed -i '/^DATABASE_URL/d' .env

# Append the new DATABASE_URL line
echo "DATABASE_URL=postgres://root:root@$ip_address:5440/exploro" >> .env
