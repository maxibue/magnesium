read -p "Enter the web address (URL) to send the POST request: " webaddress
read -p "Enter the admin key: " key
read -p "Enter the name for the new key: " name

curl -X POST "$webaddress" \
     -H "Content-Type: application/json" \
     -H "ADMIN_KEY: $key" \
     -H "KEY_TO_REMOVE: $ktm" \
     -d '{}'

echo "POST request sent to $webaddress with ADMIN_KEY: $key and KEY_TO_REMOVE: $ktm"
