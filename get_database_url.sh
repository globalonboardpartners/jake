interface=$(ip route | grep default | awk '{print $5}')
ip_address=$(ip addr show $interface | grep 'inet ' | awk '{print $2}' | cut -d/ -f1)

cp env.example .env
echo "DATABASE_URL=postgres://root:root@$ip_address:5450/celeyou" >> .env
