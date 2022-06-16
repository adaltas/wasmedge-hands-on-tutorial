# WasmEdge: WebAssembly runtimes are coming for the edge

This repository is better read along with [this article](https://adaltas.com/en/..)

```bash
# Setting up python dependencies
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
# Setting up ansible dependencies
ansible-galaxy collection install -r requirements.yml
# Setting up the virtual machine
vagrant up
# Running the playbooks
./hosts.sh # generates inventory file
ansible-playbook ansible/00_all.yml
```
