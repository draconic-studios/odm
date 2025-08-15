package coreplugins

import (
	"fmt"
	"odm/utils"
	"os"
	"strings"

	odmplugin "github.com/hembrow-innovations/odm-plugin"
)

func ExecuterCommand(body *odmplugin.ExecutionRequestBody) (string, error) {
	value, ok := body.Options["command"]
	if !ok || value == "" {
		return "", fmt.Errorf("command not found")
	}

	var cmdPath string
	cmdPathValue, ok := body.Options["path"]
	if !ok {
		cwd, err := os.Getwd()
		if err != nil {
			return "", err
		}
		cmdPath = cwd
	} else {
		cmdPath = cmdPathValue.(string)
	}

	commandStr, ok := value.(string)
	if !ok {
		return "", fmt.Errorf("command is not a string")
	}

	// Split the command string into command and args
	parts := strings.Fields(commandStr)
	if len(parts) == 0 {
		return "", fmt.Errorf("empty command")
	}

	command := parts[0]
	args := parts[1:]

	cmdToRun := fmt.Sprintf("%s %s", command, strings.Join(args, " "))
	fmt.Printf("Executing commad: %s\n\t path: %s\n\n", cmdToRun, cmdPath)

	output, err := utils.RunCommand(cmdPath, command, args...)
	fmt.Println("Output: ", output)
	if err != nil {
		return "", err
	}
	return output, nil
}
