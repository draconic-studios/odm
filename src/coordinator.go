package main

import (
	"encoding/json"
	"fmt"
	"odm/actions"
	coreplugins "odm/core-plugins"
	"odm/plugin"
	"odm/utils"
	"path/filepath"

	odmplugin "github.com/hembrow-innovations/odm-plugin"
)

// Take in Users command and executes the assosicated functions. returns (msg, err)
func Coordinator(command *utils.Command) (string, error) {
	fmt.Println("Coorindation entry")
	// if "help" present in command
	if command.Help {
		return "help Command found", nil
	}

	rawProjectPath, ok := command.Flags["project-path"]
	if !ok {
		return "", fmt.Errorf("project path not found")
	}

	// Read project definition
	projectYamlPath := filepath.Join(rawProjectPath, "project.yaml")

	fmt.Println("Project: ", projectYamlPath)
	project, err := utils.ReadProject(projectYamlPath)
	if err != nil {
		return "", err
	}
	// fmt.Println("Project: ", project)

	// If command is a core action
	// - add (Command to add project to ochestrator) - in construction
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
		PluginDir: filepath.Join(rawProjectPath, "tools", "odm", "src", "testplugins"),
		// Verbose:   cli.logging.Verbose,
	}

	pluginManager := plugin.NewPluginManager(pluginManagerOptions)

	// If project defined action

	action, ok := project.Actions[command.Name]
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
