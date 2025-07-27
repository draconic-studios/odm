package coreplugins

import (
	"fmt"
	"odm/utils"
	"os"

	odmplugin "github.com/hembrow-innovations/odm-plugin"
)

type CopyOptions struct {
	Source      string
	Destination string
	Type        string
}

func Copy(body *odmplugin.ExecutionRequestBody) (string, error) {
	var options CopyOptions
	var err error

	// Parse options
	if sourceValue, ok := body.Options["source"].(string); ok {
		options.Source = sourceValue
	}
	if destinationValue, ok := body.Options["destination"].(string); ok {
		options.Destination = destinationValue
	}
	if typeValue, ok := body.Options["type"].(string); ok {
		options.Type = typeValue
	}

	var rootPath string
	// Root path
	if rootPathValue, ok := body.Args["root-path"]; ok {
		rootPath = rootPathValue
	} else {
		rootPath, err = os.Getwd()
		if err != nil {
			return "", err
		}
	}

	// Validate required option values
	if options.Source == "" {
		return "", fmt.Errorf("source path not found")
	}
	if options.Destination == "" {
		return "", fmt.Errorf("destination path not found")
	}

	// Type defaults to folder
	if options.Type == "" {

	}

	// create command to be run
	command := fmt.Sprintf("cp %s %s", options.Source, options.Destination)
	if options.Type == "folder" {
		command = fmt.Sprintf("cp -r %s %s", options.Source, options.Destination)
	}

	// run command and run its output
	return utils.RunCommand(rootPath, command)
}
