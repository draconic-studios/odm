package orchestrator

import (
	"encoding/json"
	"fmt"
	"os"

	"gopkg.in/yaml.v3"
)

func (o *Orchestrator) ReadOdmConfig() error {
	// Read the entire file content into a byte slice
	dataBytes, err := os.ReadFile(o.FilePath)
	if err != nil {
		return fmt.Errorf("error reading project file: %s", err)
	}

	var config OrchestratorConfig

	// Parse config file according to file type (json, yaml)
	switch o.FileType {
	case "json":
		err = json.Unmarshal(dataBytes, &config)
		if err != nil {
			return err
		}

	case "yaml":
		err = yaml.Unmarshal(dataBytes, &config)
		if err != nil {
			return err
		}
	default:
		return fmt.Errorf("unknown odm.config file type")
	}

	// Store config inside self
	o.Config = config

	// return nil as success
	return nil
}

func (o *Orchestrator) WriteOdmConfig() error {
	var err error
	var dataBytes []byte
	// Parse config file according to file type (json, yaml)
	switch o.FileType {
	case "json":
		dataBytes, err = json.Marshal(o.Config)
		if err != nil {
			return err
		}
	case "yaml":
		dataBytes, err = yaml.Marshal(o.Config)
		if err != nil {
			return err
		}
	default:
		return fmt.Errorf("unknown odm.config type")
	}

	err = os.WriteFile(o.FilePath, dataBytes, 0644)
	if err != nil {
		return nil
	}

	return nil
}
