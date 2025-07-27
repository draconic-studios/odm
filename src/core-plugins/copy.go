package coreplugins

import (
	"fmt"
	"odm/utils"
	"os"
	"path/filepath"

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

	sourcePath := filepath.Join(rootPath, options.Source)
	destinationPath := filepath.Join(rootPath, options.Destination)

	// create command to be run
	if options.Type == "folder" {
		err = utils.CopyFolderContents(sourcePath, destinationPath)
		if err != nil {
			return "", err
		}
	} else {
		err = utils.CopyFile(sourcePath, destinationPath)
		if err != nil {
			return "", err
		}
	}

	// run command and run its output
	return "Successfully Copied", nil
}
