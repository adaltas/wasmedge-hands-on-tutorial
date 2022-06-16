export $(vagrant ssh-config master | awk '{if($1) printf toupper($1) "=" $2 " ";}' | xargs)

echo "Inventory:"

cat <<EOF | tee hosts
master ansible_host=$HOSTNAME ansible_port=$PORT ansible_user=$USER ansible_private_key_file=$IDENTITYFILE
EOF
