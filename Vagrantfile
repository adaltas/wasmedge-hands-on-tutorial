
IMAGE_NAME = "generic/ubuntu2004"   # Overwrite for virtualbox
MASTER_NAME="master"                # Master node name
NODE_NETWORK_BASE = "192.168.80"    # First three octets of the IP address that will be assign to all type of nodes


# ENV['VAGRANT_NO_PARALLEL'] = 'yes'

Vagrant.configure("2") do |config|
    config.ssh.insert_key = false

    # RAM and CPU config
    config.vm.provider :libvirt do |v|
        v.cpus = 4
        v.memory = 4092
    end

    # Master node config
    config.vm.define MASTER_NAME do |master|
        # Hostname and network config
        master.vm.box = IMAGE_NAME
        master.vm.network "private_network", ip: "#{NODE_NETWORK_BASE}.10"
        master.vm.hostname = MASTER_NAME

    end
end
