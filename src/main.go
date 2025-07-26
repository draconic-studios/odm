package main

import (
	"fmt"
	"os"
)

const (
	ExitSuccess      = 0
	ExitGenericError = 1
	ExitInvalidInput = 2
	ExitNetworkError = 3
	// ... define more as needed
)

func main() {

	cmd, err := HandleInput()
	if err != nil {
		fmt.Println("Error:", err)
		os.Exit(2)
	}
	fmt.Println(cmd)

	result, err := Coordinator(cmd)
	if err != nil {
		fmt.Println("Error:", err)
		os.Exit(2)
	}

	fmt.Println(result)

}
