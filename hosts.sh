
export $(vagrant ssh-config master | awk '{if($1) printf toupper($1) "=" $2 " ";}' | xargs)


env $ssh_env cat <<EOF > hosts
master ansible_host=$HOSTNAME ansible_port=$PORT ansible_user=$USER ansible_private_key_file=$IDENTITYFILE
EOF


echo "Inventory:"
cat hosts
