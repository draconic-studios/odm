package utils

import (
	"fmt"
	"odm/types"
	"os"

	"gopkg.in/yaml.v3"
)

func ReadProject(filePath string) (*types.Project, error) {
	// Step 1: Read the entire file content into a byte slice
	dataBytes, err := os.ReadFile(filePath)
	if err != nil {
		return nil, fmt.Errorf("error reading project file: %s", err)
	}

	// Step 2: Unmarshal the JSON data into your Go struct
	var project types.Project
	err = yaml.Unmarshal(dataBytes, &project)
	if err != nil {
		return nil, err
	}

	// Step 3: Return result
	return &project, err
}
