package main

import (
	"fmt"
	"odm/actions"
	"odm/messages"
	"odm/utils"
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

	if len(os.Args) < 2 {
		fmt.Print(messages.GlobalUsage + "\n")
		os.Exit(1)
	}

	command := utils.ParseArgs(os.Args[1:])

	msg, err := actions.Coordinator(command)
	if err != nil {
		// Print error message and exit with invalid input code
		fmt.Printf("%s: error!\n\t%s", command.Name, err)
		os.Exit(ExitInvalidInput)
	}

	// Print message and exit with success code
	fmt.Println(msg)
	os.Exit(ExitSuccess)
}
