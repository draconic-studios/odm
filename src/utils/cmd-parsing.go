package utils

import (
	"odm/types"
	"strings"
)

// parseArgs takes a slice of arguments (excluding the program name)
// and returns a Command struct.
func ParseArgs(args []string) *types.Command {
	if len(args) == 0 {
		return &types.Command{} // Should be caught by main's initial check
	}

	commandName := args[0]
	parsedFlags := make(map[string]string)
	parsedBoolFlags := make(map[string]bool)
	var positionalArgs []string

	// Iterate through arguments starting from the second one (index 1)
	// to parse flags and positional arguments for the command.
	for i := 1; i < len(args); i++ {
		arg := args[i]

		if strings.HasPrefix(arg, "--") {
			// Long flag, e.g., --tag=value or --verbose
			parts := strings.SplitN(arg[2:], "=", 2) // Split at most once
			flagName := parts[0]
			if len(parts) == 2 {
				parsedFlags[flagName] = parts[1]
			} else {
				// Boolean flag (e.g., --verbose without a value)
				parsedBoolFlags[flagName] = true
			}
		} else if strings.HasPrefix(arg, "-") {
			// Short flag, e.g., -p 8080 or -v
			flagName := arg[1:]
			if i+1 < len(args) && !strings.HasPrefix(args[i+1], "-") {
				// Check if the next argument is a value for this flag
				parsedFlags[flagName] = args[i+1]
				i++ // Consume the next argument as the flag value
			} else {
				// Boolean flag (e.g., -v without a value)
				parsedBoolFlags[flagName] = true
			}
		} else {
			// Positional argument
			positionalArgs = append(positionalArgs, arg)
		}
	}

	return &types.Command{
		Name:      commandName,
		Args:      positionalArgs,
		Flags:     parsedFlags,
		BoolFlags: parsedBoolFlags,
	}
}
