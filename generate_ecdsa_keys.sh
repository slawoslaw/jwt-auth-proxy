#!/bin/bash

FOLDER_PATH="keys"

# Check if the folder exists
if [ ! -d "$FOLDER_PATH" ]; then
  # If the folder doesn't exist, create it
  echo "Folder does not exist. Creating it now..."
  mkdir -p "$FOLDER_PATH"
  echo "Folder created at: $FOLDER_PATH"
else
  # If the folder exists
  echo "Folder already exists at: $FOLDER_PATH"
fi

# Define file names
PRIVATE_KEY_FILE="${FOLDER_PATH}/jwt_ecdsa_key.pem"
PUBLIC_KEY_FILE="${FOLDER_PATH}/jwt_ecdsa_key.pub.pem"

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
