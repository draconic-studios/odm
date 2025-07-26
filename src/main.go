package main

import (
	"fmt"
	"odm/cmd"
	"os"
	// "odm/utils"
)

const (
	ExitSuccess      = 0
	ExitGenericError = 1
	ExitInvalidInput = 2
	ExitNetworkError = 3
	// ... define more as needed
)

func main() {

	cli := cmd.Cli{}

	msg, err := cli.Entry()

	if err != nil {
		fmt.Println("Error:", err)
		os.Exit(2)
	}
	fmt.Println(msg)

	// if len(os.Args) < 2 {
	// 	fmt.Print(messages.GlobalUsage + "\n")
	// 	os.Exit(1)
	// }
	// command := cmd.ParseArgs(os.Args[1:])
	// fmt.Println(command.Name)
	// fmt.Println(command.Args)
	// fmt.Println(command.BoolFlags)
	// fmt.Println(command.Flags)
	// fmt.Println(command.Help)
	// command := utils.ParseArgs(os.Args[1:])

	// msg, err := actions.Coordinator(command)
	// if err != nil {
	// 	// Print error message and exit with invalid input code
	// 	fmt.Printf("%s: error!\n\t%s", command.Name, err)
	// 	os.Exit(ExitInvalidInput)
	// }

	// // Print message and exit with success code
	// fmt.Println(msg)
	// os.Exit(ExitSuccess)
}
