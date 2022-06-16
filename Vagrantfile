IMAGE_NAME = "generic/ubuntu2004"   # Overwrite for virtualbox
MASTER_NAME="master"                # Master node name

Vagrant.configure("2") do |config|
    config.ssh.insert_key = false

    config.vm.provider :virtualbox do |v|
        v.customize ["modifyvm", :id, "--memory", 4096, "--cpus", 4]
        v.name = MASTER_NAME
    end

    config.vm.define MASTER_NAME do |master|
        master.vm.box = IMAGE_NAME
        master.vm.hostname = MASTER_NAME
    end
end
