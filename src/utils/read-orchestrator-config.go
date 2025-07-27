package utils

import (
	"encoding/json"
	"fmt"
	"odm/types"
	"os"

	"gopkg.in/yaml.v3"
)

func ReadOdmConfig(filePath string, fileType string) (*types.Orchestrator, error) {
	// Read the entire file content into a byte slice
	dataBytes, err := os.ReadFile(filePath)
	if err != nil {
		return nil, fmt.Errorf("error reading project file: %s", err)
	}

	var config types.Orchestrator

	// Parse config file according to file type (json, yaml)
	switch fileType {
	case "json":
		err = json.Unmarshal(dataBytes, &config)
		if err != nil {
			return nil, err
		}

	case "yaml":
		err = yaml.Unmarshal(dataBytes, &config)
		if err != nil {
			return nil, err
		}
	default:
		return nil, fmt.Errorf("unknown odm.config file type")
	}

	// Return config struct
	return &config, err
}
