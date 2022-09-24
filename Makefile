build:
	cargo build
vm:
	multipass launch jammy --name linux-containers --mount .:/workspace --cloud-init .config/cloud-init.yaml

vm-ip:
	multipass list --format json | jq '.list[] | select(.name=="linux-containers").ipv4[0]'

shell:
	multipass shell linux-containers