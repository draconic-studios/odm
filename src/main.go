package main

import (
	"encoding/json"
	"fmt"
	coreplugins "odm/core-plugins"
	"odm/orchestrator"
	"odm/plugin"
	"odm/utils"
	"os"
	"path/filepath"

	odmplugin "github.com/hembrow-innovations/odm-plugin"
)

const (
	ExitSuccess      = 0
	ExitGenericError = 1
	ExitInvalidInput = 2
	ExitNetworkError = 3
	// ... define more as needed
)

type Coordinator struct {
	Orchestrator  *orchestrator.Orchestrator
	Command       *utils.Command
	RootPath      string
	PluginManager *plugin.PluginManager
}

// Read odm config file if it is either json or yanl
func (c *Coordinator) GetOdmConfigFile() error {

	if c.RootPath == "" {
		return fmt.Errorf("root path not provided")
	}
	// Get contents of root folder
	rootContents, err := utils.ReadFolderContents(c.RootPath)
	if err != nil {
		return err
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
		return fmt.Errorf("odm config file not found")
	}

	// Read and parse config file
	configPath := filepath.Join(c.RootPath, fmt.Sprintf("odm.config.%s", configType))

	orc := &orchestrator.Orchestrator{
		RootPath: c.RootPath,
		FilePath: configPath,
		FileType: configType,
	}
	err = orc.ReadOdmConfig()
	if err != nil {
		return err
	}
	c.Orchestrator = orc
	return nil
}

// execute defined action
func (c *Coordinator) executeDefinedAction(actionName string) error {
	// Get action declartion
	action, ok := c.Orchestrator.Config.Actions[actionName]
	if !ok {
		return fmt.Errorf("action not found: %s", actionName)
	}

	// Setup base input/output for task cycle
	currentOutput := ""

	fmt.Printf("%d tasks to execute\n", len(action.Tasks))
	// loop tasks and execute
	for idx, task := range action.Tasks {
		fmt.Printf("Executing Task %d: %s\n", idx+1, task.Executer)
		// Create request body
		taskBody := &odmplugin.ExecutionRequestBody{
			Args:    action.Args,
			Options: task.Options,
			Input:   currentOutput,
		}

		// If task is core plugin execute
		corePlugin, ok := coreplugins.CorePluginList[task.Executer]
		if ok {
			fmt.Println("Executing core plugin")
			result, err := corePlugin(taskBody)
			if err != nil {
				fmt.Printf("Error executing core plugin: %s\n", err)
				return err
			}
			// Set output for next task
			currentOutput = result
			continue
		}

		// Check plugin exists
		var pluginExists bool
		for _, p := range c.PluginManager.Plugins {
			if p.Name == task.Executer {
				pluginExists = true
			}
		}
		if !pluginExists {
			return fmt.Errorf("plugin: %s not found", task.Executer)
		}

		// Json request body
		body, err := json.Marshal(taskBody)
		if err != nil {
			return err
		}

		// Run plugin
		output, err := c.PluginManager.Run(task.Executer, string(body))
		if err != nil {
			return err
		}

		// Set output for next task
		currentOutput = string(output)

	}
	return nil
}

// is command a core action
func (c *Coordinator) isDefinedAction(action string) bool {
	_, ok := c.Orchestrator.Config.Actions[action]
	return ok
}

// Initialize Coordinator
func (c *Coordinator) initCoordinator(command *utils.Command) error {
	fmt.Println("Initialize Coorindator")
	c.Command = command
	// Set root path
	rootPath, ok := command.Flags["root-path"]
	if !ok {
		return fmt.Errorf("root path not found")
	}
	c.RootPath = rootPath

	// Get Orchestrator
	err := c.GetOdmConfigFile()
	if err != nil {
		return err
	}

	if c.Orchestrator == nil {
		return fmt.Errorf("orchestrater not initialized")
	}
	return nil
}

// Initialize Plugin Manager
func (c *Coordinator) initPluginManager() error {
	fmt.Println("Initialize plugin manager")

	// Default plugin location
	if c.Orchestrator.Config.PluginConfig.Location == "" {
		c.Orchestrator.Config.PluginConfig.Location = ".plugins"
	}
	if c.Orchestrator.Config.PluginConfig.Location == "" {
		c.Orchestrator.Config.PluginConfig.PluginSuffix = "-plugin"
	}

	// Setup plugin manager for use
	pluginManagerOptions := &plugin.PluginManagerOptions{
		PluginDir: filepath.Join(c.RootPath, c.Orchestrator.Config.PluginConfig.Location),
		// Verbose:   cli.logging.Verbose,
	}

	c.PluginManager = plugin.NewPluginManager(pluginManagerOptions)

	return nil
}

func (c *Coordinator) runHelp(command *utils.Command) (string, error) {
	// TODO add help text for commands
	fmt.Println("Help")

	return "", nil
}

// Take in Users command and executes the assosicated functionality. returns (msg, err)
func (c *Coordinator) runCoordinator(command *utils.Command) (string, error) {
	if c.Orchestrator == nil {
		return "", fmt.Errorf("orchestrater not initialized")
	}
	/*
		1. check if help command is passed
		2. check if core action and execute
		3. check if defined action and execute
	*/

	fmt.Println("Coorindation entry")
	// if "help" present in command
	if command.Help {
		return c.runHelp(command)

	}

	// If command is a core action
	// - add 	(Command to add project to ochestrator)
	// - remove (Command to remove project to ochestrator)
	// !move this to a plugin - build-docs (Command to build out a web server serving docs from orchestrator and submodules)
	if c.Orchestrator.IsCoreAction(c.Command.Name) {
		err := c.Orchestrator.ExecuteCoreAction(c.Command)
		if err != nil {
			return "", err
		}
		return "Executed Action", nil

	}

	// If action defined in config
	isAction := c.isDefinedAction(command.Name)

	if !isAction {
		return "", fmt.Errorf("action not found")
	}

	// Execute defined action
	c.executeDefinedAction(command.Name)

	return "", nil

}

// get user command and parse into struct
func HandleInput() (*utils.Command, error) {

	// Check for args
	if len(os.Args) < 2 {
		return nil, fmt.Errorf("command not passed")
	}

	args := os.Args

	// Parse input into command
	command, err := utils.ParseArgs(args[1:])
	if err != nil {
		return nil, err
	}

	// No command passed
	if command.Name == "" {
		return nil, fmt.Errorf("command not passed")
	}

	// Set root path
	if command.Flags["root-path"] == "" {
		cwd, err := os.Getwd()
		if err != nil {
			return nil, err
		}
		command.Flags["root-path"] = cwd
	}

	// Run coordinator
	return command, nil

}

func main() {

	// Parse input in command struct
	cmd, err := HandleInput()
	if err != nil {
		fmt.Println("Error:", err)
		os.Exit(2)
	}
	fmt.Println(cmd)

	// create coordinator
	newCoordinator := &Coordinator{}

	// Execute command
	err = newCoordinator.initCoordinator(cmd)
	if err != nil {
		fmt.Println("Error:", err)
		os.Exit(2)
	}
	result, err := newCoordinator.runCoordinator(cmd)
	if err != nil {
		fmt.Println("Error:", err)
		os.Exit(2)
	}

	fmt.Println(result)

}
