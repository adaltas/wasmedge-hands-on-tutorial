# WasmEdge: WebAssembly runtimes are coming for the edge

This repository is better read along with [this article](https://adaltas.com/en/..)

Install pip dependencies:
```bash
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

Install ansible dependencies:
```bash
source venv/bin/activate
ansible-galaxy collection install -r requirements.yml
```

Setting up the virtual machine:
```bash
vagrant up
```

Run the playbooks:
```bash
./hosts.sh # generates inventory file
ansible-playbook ansible/00_all.yml
```
