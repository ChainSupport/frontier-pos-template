#!/bin/bash

secret=$1
i=$2
# ed25519
subkey inspect --scheme ed25519 --output-type json "//$secret"//fir//ed//$i

# sr25519
subkey inspect --scheme sr25519 --output-type json  "//$secret"/fir/sr/$i

# ecdsa
subkey inspect --scheme ecdsa --output-type json "//$secret"//fir//ecdsa//$i
