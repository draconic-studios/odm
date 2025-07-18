package main

import (
	"fmt"
	"odm/cli"
	"os"
	"strconv" // For converting string arguments to integers
	"strings" // For string manipulation like TrimPrefix, HasPrefix
)

// Define structures to hold parsed command-line data
type Command struct {
	Name  string
	Args  []string          // Positional arguments
	Flags map[string]string // Key-value pairs for flags
	// Add other specific fields if needed, e.g., bool flags
	BoolFlags map[string]bool
}

func main() {

	cli := cli.NewOdmCli()

	cli.Execute()

	// os.Args[0] is the program name, so we start parsing from os.Args[1]
	// if len(os.Args) < 2 {
	// 	printGlobalUsage()
	// 	os.Exit(1)
	// }

	// cmd := parseArgs(os.Args[1:])

	// switch cmd.Name {
	// case "build":
	// 	handleBuildCommand(cmd)
	// case "run":
	// 	handleRunCommand(cmd)
	// case "help":
	// 	handleHelpCommand(cmd)
	// default:
	// 	fmt.Printf("Error: Unknown command \"%s\"\n", cmd.Name)
	// 	printGlobalUsage()
	// 	os.Exit(1)
	// }
}

// parseArgs takes a slice of arguments (excluding the program name)
// and returns a Command struct.
func parseArgs(args []string) Command {
	if len(args) == 0 {
		return Command{} // Should be caught by main's initial check
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

	return Command{
		Name:      commandName,
		Args:      positionalArgs,
		Flags:     parsedFlags,
		BoolFlags: parsedBoolFlags,
	}
}

// --- Command Handlers ---

func handleBuildCommand(cmd Command) {
	fmt.Println("--- Build Command ---")
	fmt.Printf("Command: %s\n", cmd.Name)
	fmt.Printf("Positional Args: %v\n", cmd.Args)
	fmt.Printf("Flags: %v\n", cmd.Flags)

	imageName := ""
	if len(cmd.Args) > 0 {
		imageName = cmd.Args[0]
	} else {
		fmt.Println("Error: 'build' command requires an image name.")
		printBuildUsage()
		os.Exit(1)
	}

	tag, hasTag := cmd.Flags["tag"]
	if !hasTag {
		tag = "latest" // Default tag
	}

	fmt.Printf("Building image: %s with tag: %s\n", imageName, tag)

	// Simulate build logic here
	fmt.Println("Simulating image build...")
	fmt.Println("Build completed successfully!")
}

func handleRunCommand(cmd Command) {
	fmt.Println("--- Run Command ---")
	fmt.Printf("Command: %s\n", cmd.Name)
	fmt.Printf("Positional Args: %v\n", cmd.Args)
	fmt.Printf("Flags: %v\n", cmd.Flags)
	fmt.Printf("Boolean Flags: %v\n", cmd.BoolFlags)

	imageName := ""
	if len(cmd.Args) > 0 {
		imageName = cmd.Args[0]
	} else {
		fmt.Println("Error: 'run' command requires an image name.")
		printRunUsage()
		os.Exit(1)
	}

	containerName, _ := cmd.Flags["name"] // Defaults to empty string if not present
	if containerName == "" {
		containerName = fmt.Sprintf("random-container-%d", os.Getpid()) // Simple default
	}

	portMapping := ""
	if p, ok := cmd.Flags["p"]; ok {
		portMapping = p
		// You might want to parse the portMapping further (e.g., "8080:80")
		if _, err := strconv.Atoi(strings.Split(portMapping, ":")[0]); err != nil {
			fmt.Printf("Warning: Invalid host port format for -p: %s\n", portMapping)
		}
	}

	detached := cmd.BoolFlags["d"]            // Check for -d or --d (assuming 'd' is a boolean flag)
	if _, ok := cmd.BoolFlags["detach"]; ok { // Also check for --detach
		detached = true
	}

	fmt.Printf("Running container from image: %s\n", imageName)
	fmt.Printf("  Container Name: %s\n", containerName)
	if portMapping != "" {
		fmt.Printf("  Port Mapping: %s\n", portMapping)
	}
	fmt.Printf("  Detached Mode: %t\n", detached)

	// Simulate run logic here
	fmt.Println("Simulating container run...")
	fmt.Println("Container started.")
}

func handleHelpCommand(cmd Command) {
	if len(cmd.Args) > 0 {
		subcommand := cmd.Args[0]
		switch subcommand {
		case "build":
			printBuildUsage()
		case "run":
			printRunUsage()
		default:
			fmt.Printf("Help for unknown subcommand: %s\n", subcommand)
			printGlobalUsage()
		}
	} else {
		printGlobalUsage()
	}
}

// --- Usage Functions ---

func printGlobalUsage() {
	fmt.Println(`
Usage: mycli <command> [arguments]

Commands:
  build    Build a Docker image
  run      Run a Docker container
  help     Display help information

Use "mycli help <command>" for more information about a command.
`)
}

func printBuildUsage() {
	fmt.Println(`
Usage: mycli build <image-name> [flags]

Build a Docker image from a Dockerfile.

Arguments:
  <image-name>  The name for the Docker image (e.g., myapp).

Flags:
  --tag <tag>   Specify a tag for the image (default: latest).
`)
}

func printRunUsage() {
	fmt.Println(`
Usage: mycli run <image-name> [flags]

Run a Docker container from an image.

Arguments:
  <image-name>  The name of the Docker image to run.

Flags:
  --name <name>  Assign a name to the container.
  -p <host:cont> Publish a container's port(s) to the host (e.g., -p 8080:80).
  -d, --detach   Run container in detached mode (in the background).
`)
}
