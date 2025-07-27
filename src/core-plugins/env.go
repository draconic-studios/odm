package coreplugins

import (
	"bufio"
	"encoding/json"
	"fmt"
	"maps"
	"odm/types"
	"odm/utils"
	"os"
	"path/filepath"
	"strings"

	odmplugin "github.com/hembrow-innovations/odm-plugin"
	"gopkg.in/yaml.v3"
)

func Env(body *odmplugin.ExecutionRequestBody) (string, error) {
	var options types.EnvOptions
	var rootPath string
	var err error

	// Set root path
	if rootPathValue, ok := body.Args["root-path"]; ok {
		rootPath = rootPathValue
	} else {
		v, err := os.Getwd()
		if err != nil {
			return "", err
		}
		rootPath = v
	}

	// Parse options
	if itemsValue, ok := body.Options["items"].([]types.BuildItem); ok {
		options.Items = itemsValue
	}

	// Output path
	if outputValue, ok := body.Options["output"].(string); ok {

		options.Output = outputValue
	}
	// Put env file name ".env" if not already present
	if !strings.HasSuffix(options.Output, ".env") {
		options.Output = filepath.Join(options.Output, ".env")
	}

	// Items
	if buildItemsValue, ok := body.Options["items"].([]types.BuildItem); ok {
		options.Items = buildItemsValue
	}

	envMap := make(map[string]string)

	for _, item := range options.Items {
		itemPath := filepath.Join(rootPath, item.FilePath)
		var envsToAdd *map[string]string

		// Handle each file type
		switch item.File {
		case "env":
			envsToAdd, err = readEnvFile(itemPath, &item.EnvKeys)
			if err != nil {
				return "", err
			}
		case "json":
			jsonMap, err := readJson(itemPath)
			if err != nil {
				return "", err
			}
			envsToAdd, err = getEnvFromMap(jsonMap, &item.Keys)
			if err != nil {
				return "", err
			}
		case "yaml":
			yamlMap, err := readYaml(itemPath)
			if err != nil {
				return "", err
			}
			envsToAdd, err = getEnvFromMap(yamlMap, &item.Keys)
			if err != nil {
				return "", err
			}
		default:
			continue
		}

		// Merge new envs into final map
		maps.Copy(envMap, *envsToAdd)
	}

	// Write env file
	writeEnv(&envMap, options.Output)

	return fmt.Sprintf("Wrote %d items to %s", len(options.Items), options.Output), nil
}

// get env from data map
func getEnvFromMap(dataMap *map[string]any, keys *[]types.BuildItemKey) (*map[string]string, error) {
	newMap := make(map[string]string)

	for _, keyPath := range *keys {
		keys := strings.Split(keyPath.Key, ".")

		value, ok := utils.GetNestedValue(*dataMap, keys)
		if !ok {
			return nil, fmt.Errorf("value not found at %s", keyPath.Key)
		}
		newValue := utils.ConvertAnyToString(value)

		if value != "" {
			newMap[keyPath.EnvName] = newValue
		}
	}
	return &newMap, nil
}

// Read a .env file and return a map of envs found
func readEnvFile(envPath string, filter *[]string) (*map[string]string, error) {

	// Open file from give path
	file, err := os.Open(envPath)
	if err != nil {
		return nil, fmt.Errorf("failed to open file: %v", err)
	}

	// Ensure the file is closed when func completes
	defer file.Close()

	envs := make(map[string]string)
	// filter
	whitelistAll := len(*filter) < 1
	whitelist := make(map[string]bool)
	for _, key := range *filter {
		whitelist[key] = true
	}

	// Process file line by line
	// use scanner to reduce loading large files into memory all at once
	scanner := bufio.NewScanner(file)

	// Remove null lines and remove comments
	for scanner.Scan() {
		line := scanner.Text()

		var scrubbedLine string

		// Split line at start of comment
		before, _, found := strings.Cut(line, "#")

		// if comment if at the end of a line containing a env
		if found {
			if before != "" {
				scrubbedLine = before
			}
		} else if line != "" {
			// if line has env and no comment
			scrubbedLine = line
		}
		// env can't be on the right side of line (this would be inside comment)

		// split env into key and value and assign to map
		keyValue := strings.Split(scrubbedLine, "=")
		if len(keyValue) == 2 {
			// Check whitelist
			if whitelist[keyValue[0]] || whitelistAll {
				// add if whitelisted
				envs[keyValue[0]] = keyValue[1]
			}
		}

	}

	// Err reading file contents with scanner
	if err := scanner.Err(); err != nil {
		return nil, fmt.Errorf("error reading file: %v", err)
	}

	// Return map of envs
	return &envs, nil
}

// writes a map to as a env file
func writeEnv(env *map[string]string, destination string) error {

	var content string

	// Loop over map and append to string "{key}={value}"
	for k, v := range *env {
		content = fmt.Sprintf("%s\n%s=%s", content, k, v)
	}

	// write string to file as .env file
	err := os.WriteFile(destination, []byte(content), 0644)
	if err != nil {
		return err
	}
	return nil
}

func readJson(jsonPath string) (*map[string]any, error) {

	// Read the entire json file content into a byte slice
	dataBytes, err := os.ReadFile(jsonPath)
	if err != nil {
		return nil, err
	}

	// Declare a map[string]any to hold the unmarshaled JSON data
	var data map[string]any

	// Unmarshal the JSON string into the map
	err = json.Unmarshal(dataBytes, &data)
	if err != nil {
		return nil, err
	}

	return &data, nil
}
func readYaml(jsonPath string) (*map[string]any, error) {

	// Read the entire json file content into a byte slice
	dataBytes, err := os.ReadFile(jsonPath)
	if err != nil {
		return nil, err
	}

	// Declare a map[string]any to hold the unmarshaled JSON data
	var data map[string]any

	// Unmarshal the JSON string into the map
	err = yaml.Unmarshal(dataBytes, &data)
	if err != nil {
		return nil, err
	}

	return &data, nil
}
