#!/bin/bash

# Define file names
PRIVATE_KEY_FILE="keys/jwt_ecdsa_key.pem"
PUBLIC_KEY_FILE="keys/jwt_ecdsa_key.pub.pem"

# Generate ECDSA private key using prime256v1 curve
echo "Generating ECDSA private key..."
openssl ecparam -genkey -name prime256v1 -noout | openssl pkcs8 -topk8 -nocrypt -out "$PRIVATE_KEY_FILE"

# Check if private key generation was successful
if [[ $? -ne 0 ]]; then
    echo "Error: Failed to generate the private key."
    exit 1
fi
echo "Private key saved to $PRIVATE_KEY_FILE"

# Generate the public key in PEM format from the private key
echo "Generating ECDSA public key..."
openssl ec -in "$PRIVATE_KEY_FILE" -pubout -out "$PUBLIC_KEY_FILE"

# Check if public key generation was successful
if [[ $? -ne 0 ]]; then
    echo "Error: Failed to generate the public key."
    exit 1
fi
echo "Public key saved to $PUBLIC_KEY_FILE"

echo "ECDSA key pair generated successfully."
