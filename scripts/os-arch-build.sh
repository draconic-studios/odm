#!/bin/bash

# Build for all platforms
echo "Building for all platforms..."

cd src

# Mac Apple Silicon
GOOS=darwin GOARCH=arm64 go build -o ../bin/odm-darwin-arm64 main.go

# Mac Intel
GOOS=darwin GOARCH=amd64 go build -o ../bin/odm-darwin-amd64 main.go

# Linux
GOOS=linux GOARCH=amd64 go build -o ../bin/odm-linux-amd64 main.go

# Windows
GOOS=windows GOARCH=amd64 go build -o ../bin/odm-windows-amd64.exe main.go

echo "Build complete!"