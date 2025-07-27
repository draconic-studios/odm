package main

import (
	"fmt"
	"odm/utils"
	"os"
)

func HandleInput() (*utils.Command, error) {

	// Check for args
	if len(os.Args) < 2 {
		return nil, fmt.Errorf("command not passed")
	}

	args := os.Args

	// Parse input into command
	command, err := utils.ParseArgs(args[1:])
	if err != nil {
		return nil, err
	}

	// No command passed
	if command.Name == "" {
		return nil, fmt.Errorf("command not passed")
	}

	// Set root path
	if command.Flags["root-path"] == "" {
		cwd, err := os.Getwd()
		if err != nil {
			return nil, err
		}
		command.Flags["root-path"] = cwd
	}

	// Run coordinator
	return command, nil

}
