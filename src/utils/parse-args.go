package utils

import (
	"fmt"
	"strings"
)

// Define structures to hold parsed command-line data
type Command struct {
	Name      string
	Args      []string          // Positional arguments
	Flags     map[string]string // Key-value pairs for flags
	BoolFlags map[string]bool
	Help      bool
}

// parseArgs takes a slice of arguments (excluding the program name)
// and returns a Command struct and an error.
func ParseArgs(args []string) (*Command, error) {
	if len(args) == 0 {

		return &Command{}, fmt.Errorf("command line arguments: %s", "0") // Should be caught by main's initial check
	}

	commandName := args[0]
	parsedFlags := make(map[string]string)
	parsedBoolFlags := make(map[string]bool)
	var positionalArgs []string
	help := false
	flagsStarted := false

	// Iterate through arguments starting from the second one (index 1)
	// to parse flags and positional arguments for the command.

	skipIndex := -1

	for i := 1; i < len(args); i++ {
		if skipIndex == i {
			continue
		}
		arg := args[i]
		// Positional argument (before flags)
		if !strings.HasPrefix(arg, "-") && !flagsStarted {
			if arg == "help" {
				help = true
			}
			positionalArgs = append(positionalArgs, arg)
		} else {
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
				continue
			}

			// if next item in array is flag (flag is bool)
			// and current items value not connect value with "="
			if strings.HasPrefix(args[i+1], "-") && !strings.Contains(arg, "=") {
				parsedBoolFlags[arg] = true
				continue
			}

			// if next item is not flag, next item is value

			if !strings.Contains(arg, "=") {
				value = args[i+1]
				skipIndex = i + 1
			}

			parts := strings.SplitN(arg[dashCount:], "=", 2) // Split at most once
			flagName := parts[0]
			if len(parts) == 2 {
				value = parts[1]
			}
			parsedFlags[flagName] = value
			continue
		}
	}

	return &Command{
		Name:      commandName,
		Args:      positionalArgs,
		Flags:     parsedFlags,
		BoolFlags: parsedBoolFlags,
		Help:      help,
	}, nil
}
