package cmd

import (
	"fmt"
	"odm/messages"
	"odm/plugin"
	"odm/utils"
	"os"
)

type Cli struct {
	rootPath      string
	logging       Logging
	pluginManager *plugin.PluginManager
}

type Logging struct {
	Verbose bool
}

func (cli *Cli) LogVerbose(text string) {
	if cli.logging.Verbose {
		fmt.Println(text)
	}

}
func (cli *Cli) CheckVerbose(args []string) {

	for _, v := range args {
		if v == "--verbose" {
			cli.logging.Verbose = true
		}
	}

}

func (cli *Cli) Entry() (string, error) {

	if len(os.Args) < 2 {
		fmt.Print(messages.GlobalUsage + "\n")
		return "", fmt.Errorf("commands not passed")
	}

	args := os.Args

	// Check logging (verbose)
	cli.CheckVerbose(args)

	// Parse input into command
	command, err := utils.ParseArgs(args[1:])
	if err != nil {
		return "", err
	}
	cli.LogVerbose(fmt.Sprintf("Command: %s\n Args: %s\n Flags: %s\n Bool Flags: %v\n Help: %t", command.Name, command.Args, command.Flags, command.BoolFlags, command.Help))

	// No command passed
	if command.Name == "" {
		return "", fmt.Errorf("command not passed")
	}

	// Set root path
	if command.Flags["project-path"] != "" {
		cli.rootPath = command.Flags["project-path"]
		cli.LogVerbose(fmt.Sprintf("setting root path found as project-path: '%s'", command.Flags["project-path"]))
	} else {
		cwd, err := os.Getwd()
		if err != nil {
			return "", err
		}
		cli.rootPath = cwd
		cli.LogVerbose(fmt.Sprintf("Setting root path as current working directory: '%s'", cwd))
	}

	// Run coordinator
	return cli.Coordinator(command)

}
