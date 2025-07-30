package coreplugins

import (
	"fmt"
	"odm/utils"
	"os"

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

	command, ok := value.(string)
	if !ok {
		return "", fmt.Errorf("command is not a string")
	}

	args, ok := value.([]string)
	if !ok {
		return "", fmt.Errorf("args is not a arry of strings")
	}
	output, err := utils.RunCommand(cmdPath, command, args...)

	if err != nil {
		return "", err
	}
	return output, nil
}
