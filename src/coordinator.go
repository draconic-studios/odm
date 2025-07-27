package main

import (
	"encoding/json"
	"fmt"
	"odm/actions"
	coreplugins "odm/core-plugins"
	"odm/plugin"
	"odm/types"
	"odm/utils"
	"path/filepath"

	odmplugin "github.com/hembrow-innovations/odm-plugin"
)

func getOdmConfigFile(rootPath string) (*types.Orchestrator, error) {

	// Get contents of root folder
	rootContents, err := utils.ReadFolderContents(rootPath)
	if err != nil {
		return nil, err
	}

	// determine type of config file yaml or json
	var configType string
	for _, item := range *rootContents {
		if item.Name() == "odm.config.yaml" {
			configType = "yaml"
			break
		}
		if item.Name() == "odm.config.json" {
			configType = "json"
			break
		}
	}

	// error if config does not exist
	if configType == "" {
		return nil, fmt.Errorf("odm config file not found")
	}

	// Read and parse config file
	configPath := filepath.Join(rootPath, fmt.Sprintf("odm.config.%s", configType))
	config, err := utils.ReadOdmConfig(configPath, configType)
	if err != nil {
		return nil, err
	}

	return config, nil
}

// Take in Users command and executes the assosicated functionality. returns (msg, err)
func Coordinator(command *utils.Command) (string, error) {
	fmt.Println("Coorindation entry")
	// if "help" present in command
	if command.Help {
		return "help Command found", nil
	}

	rootPath, ok := command.Flags["root-path"]
	if !ok {
		return "", fmt.Errorf("root path not found")
	}

	// Get Config definition
	orchestrationConfig, err := getOdmConfigFile(rootPath)
	if err != nil {
		return "", err
	}

	// If command is a core action
	// - add 	(Command to add project to ochestrator)
	// - remove (Command to remove project to ochestrator)
	// -
	coreAction, ok := actions.ActionList[command.Name]
	if ok {
		result, err := coreAction(command)
		if err != nil {
			return "", err
		}
		return result, nil
	}

	// Setup plugin manager for use
	pluginManagerOptions := &plugin.PluginManagerOptions{
		PluginDir: filepath.Join(rootPath, "tools", "odm", "src", "testplugins"),
		// Verbose:   cli.logging.Verbose,
	}

	pluginManager := plugin.NewPluginManager(pluginManagerOptions)

	// If action defined in config
	action, ok := orchestrationConfig.Actions[command.Name]
	if !ok {
		return "", fmt.Errorf("action not found")
	}

	currentOutput := ""
	for _, task := range action.Tasks {

		// Create request body
		taskBody := &odmplugin.ExecutionRequestBody{
			Args:    action.Args,
			Options: task.Options,
			Input:   currentOutput,
		}

		// If task is core plugin execute
		corePlugin, ok := coreplugins.CorePluginList[task.Executer]
		if ok {
			result, err := corePlugin(taskBody)
			if err != nil {
				return "", err
			}
			// Set output for next task
			currentOutput = result
			continue
		}

		// Check plugin exists
		var pluginExists bool
		for _, p := range pluginManager.Plugins {
			if p == task.Executer {
				pluginExists = true
			}
		}
		if !pluginExists {
			return "", fmt.Errorf("plugin: %s not found", task.Executer)
		}

		// Json request body
		body, err := json.Marshal(taskBody)
		if err != nil {
			return "", err
		}

		// Run plugin
		output, err := pluginManager.Run(task.Executer, string(body))
		if err != nil {
			return "", err
		}

		// Set output for next task
		currentOutput = string(output)

	}

	return "", nil

}
