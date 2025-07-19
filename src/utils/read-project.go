package utils

import (
	"encoding/json"
	"fmt"
	"odm/types"
	"os"
)

func ReadProject(filePath string) (*types.Project, error) {
	// Step 1: Read the entire file content into a byte slice
	jsonData, err := os.ReadFile(filePath)
	if err != nil {
		return nil, fmt.Errorf("error reading project.json: %s", err)
	}

	// Step 2: Unmarshal the JSON data into your Go struct
	var project types.Project
	err = json.Unmarshal(jsonData, &project)
	if err != nil {
		return nil, err
	}

	// Step 3: Return result
	return &project, err
}
