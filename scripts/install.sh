# For user-specific:
cd src
go build -o odm main.go
cd ..
mv ./src/odm .
mv odm /usr/local/bin/