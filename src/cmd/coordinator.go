package cmd

import (
	"encoding/json"
	"fmt"
	"odm/plugin"
	"odm/types"
	"odm/utils"
	"path/filepath"
)

// Take in Users command and executes the assosicated functions. returns (msg, err)
func (cli *Cli) Coordinator(command *types.Command) (string, error) {
	fmt.Println("Coorindation entry")
	// if "help" present in command
	if command.Help {
		fmt.Println("Help Command found")
		return cli.Help((command))
	}

	projectPath := filepath.Join(cli.rootPath, "project.yaml")

	fmt.Println("Project: ", projectPath)
	project, err := utils.ReadProject(projectPath)
	if err != nil {
		return "", err
	}
	// fmt.Println("Project: ", project)

	// Setup plugin manager for use
	pluginManagerOptions := &plugin.PluginManagerOptions{
		PluginDir: filepath.Join(cli.rootPath, "tools", "odm", "src", "testplugins"),
		Verbose:   cli.logging.Verbose,
	}

	cli.pluginManager = plugin.NewPluginManager(pluginManagerOptions)

	// If command is a main command
	// - add (Command to add project to repo) - in construction
	// -

	// If project defined action

	action, ok := project.Actions[command.Name]
	if !ok {
		return "", fmt.Errorf("action not found")
	}

	currentOutput := ""
	for index, task := range action.Tasks {
		// Check plugin exists
		var pluginExists bool
		for _, p := range cli.pluginManager.Plugins {
			if p == task.Executer {
				pluginExists = true
			}
		}
		if !pluginExists {
			return "", fmt.Errorf("plugin: %s not found", task.Executer)
		}

		cli.LogVerbose(fmt.Sprintf("Executing task %d. %s...", index+1, task.Executer))

		// Create request body
		taskBody := &plugin.ExecutionRequestBody{
			Args:    action.Args,
			Options: task.Options,
			Input:   currentOutput,
		}

		// Json request body
		body, err := json.Marshal(taskBody)
		if err != nil {
			return "", err
		}

		// Run plugin
		output, err := cli.pluginManager.Run(task.Executer, string(body))
		if err != nil {
			return "", err
		}

		// Set output for next task
		currentOutput = string(output)

		cli.LogVerbose(fmt.Sprintf("Output %d. %s: \n%s", index+1, task.Executer, currentOutput))
	}

	return "", nil
	// // Execute Function according to command user entered
	// switch command.Name {

	// // Build the system to a level that it is able to run (dev/prod)
	// case "build":
	// 	if command.Help {
	// 		return messages.BuildUsage, nil
	// 	}

	// 	buildConfig, err := build.ParseCommand(command)
	// 	if err != nil {
	// 		return "", err
	// 	}
	// 	fmt.Printf(
	// 		"Building %s System:\n\tProject: %s\n\tOutout: %s\n\tServices Folder: %s\n\tConfig Folder: %s\n\n",

	// 		buildConfig.BuildType,
	// 		buildConfig.ProjectPath,
	// 		buildConfig.Output,
	// 		buildConfig.ServicesFolder,
	// 		buildConfig.ConfigFolder,
	// 	)

	// 	err = build.Build(buildConfig)
	// 	if err != nil {
	// 		return "", err
	// 	}
	// 	return fmt.Sprintf("Successfully built for %s", buildConfig.BuildType), nil

	// case "run":
	// 	if command.Help {
	// 		return messages.RunUsage, nil
	// 	}
	// 	runOpts, err := run.ParseCommand(command)
	// 	if err != nil {
	// 		return "", err
	// 	}

	// 	err = run.Run(runOpts)
	// 	if err != nil {
	// 		return "", err
	// 	}

	// 	return "Successfully Started System", nil
	// }

	// return "", fmt.Errorf("command not found")
}
