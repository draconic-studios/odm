package cmd

import (
	"fmt"
	"odm/types"
	"strings"
)

// parseArgs takes a slice of arguments (excluding the program name)
// and returns a Command struct.
func (cli *Cli) ParseArgs(args []string) *types.Command {
	cli.LogVerbose("started passing command")
	if len(args) == 0 {
		cli.LogVerbose(fmt.Sprintf("command line arguments: %s", "0"))
		return &types.Command{} // Should be caught by main's initial check
	}

	commandName := args[0]
	parsedFlags := make(map[string]string)
	parsedBoolFlags := make(map[string]bool)
	var positionalArgs []string
	help := false
	flagsStarted := false

	// Iterate through arguments starting from the second one (index 1)
	// to parse flags and positional arguments for the command.
	cli.LogVerbose(fmt.Sprintf("arguments to be parsed: %s", args))

	skipIndex := -1

	for i := 1; i < len(args); i++ {
		if skipIndex == i {
			cli.LogVerbose(fmt.Sprintf("skipping index: %d", skipIndex))
			continue
		}
		arg := args[i]
		cli.LogVerbose(fmt.Sprintf("current argument: %s", arg))
		// Positional argument (before flags)
		if !strings.HasPrefix(arg, "-") && !flagsStarted {
			if arg == "help" {
				cli.LogVerbose("'help' argument found")
				help = true
			}
			cli.LogVerbose(fmt.Sprintf("positional argument: %s", arg))
			positionalArgs = append(positionalArgs, arg)
		} else {
			cli.LogVerbose("argument is a flag")
			// Flags
			value := ""
			flagsStarted = true

			// Set short or long flag
			dashCount := 1
			if strings.HasPrefix(arg, "--") {
				dashCount = 2
			}

			// If no more args/flags (flag is a bool)
			if len(args) == i+1 {
				parsedBoolFlags[arg] = true
				cli.LogVerbose("flag parsed as bool flag")
				continue
			}

			// if next item in array is flag (flag is bool)
			// and current items value not connect value with "="
			if strings.HasPrefix(args[i+1], "-") && !strings.Contains(arg, "=") {
				parsedBoolFlags[arg] = true
				cli.LogVerbose("flag parsed as bool flag")
				continue
			}

			// if next item is not flag, next item is value

			if !strings.Contains(arg, "=") {
				value = args[i+1]
				skipIndex = i + 1
				cli.LogVerbose("flag value is next argument")
			}

			parts := strings.SplitN(arg[dashCount:], "=", 2) // Split at most once
			flagName := parts[0]
			if len(parts) == 2 {
				cli.LogVerbose("flag value is seperated by '='")
				value = parts[1]
			}
			parsedFlags[flagName] = value
			cli.LogVerbose(fmt.Sprintf("flag parsed: %s=%s", flagName, value))
			continue
		}
	}

	return &types.Command{
		Name:      commandName,
		Args:      positionalArgs,
		Flags:     parsedFlags,
		BoolFlags: parsedBoolFlags,
		Help:      help,
	}
}
