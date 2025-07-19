package actions

import (
	"fmt"
	"odm/actions/build"
	"odm/types"
)

// Take in Users command and executes the assosicated functions. returns (msg, err)
func Coordinator(command *types.Command) (string, error) {

	// Execute Function according to command user entered
	switch command.Name {

	// Build the system to a level that it is able to run (dev/prod)
	case "build":
		buildConfig, err := build.ParseCommand(command)
		if err != nil {
			return "", err
		}
		fmt.Printf(
			"Building %s System:\n\tProject: %s\n\tOutout: %s\n\tServices Folder: %s\n\tConfig Folder: %s\n\n",

			buildConfig.BuildType,
			buildConfig.ProjectPath,
			buildConfig.Output,
			buildConfig.ServicesFolder,
			buildConfig.ConfigFolder,
		)

		err = build.Build(buildConfig)
		if err != nil {
			return "", err
		}
		return fmt.Sprintf("Successfully built for %s", buildConfig.BuildType), nil

	}

	return "", fmt.Errorf("command not found")
}
