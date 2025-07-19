package build

import (
	"odm/types"
	"os"
)

// Get Build config from command
func ParseCommand(cmd *types.Command) (*types.BuildOptions, error) {
	buildConfig := &types.BuildOptions{
		BuildType:      "",
		ProjectPath:    "",
		Output:         "",
		ServicesFolder: "",
		ConfigFolder:   "",
	}

	// Get the the project path from flags else default to "${pwd}"
	if cmd.Flags["project"] == "" {
		if cmd.Flags["p"] == "" {
			pwd, err := os.Getwd()
			if err != nil {
				return nil, err
			}
			buildConfig.ProjectPath = pwd
		} else {
			buildConfig.ProjectPath = cmd.Flags["p"]
		}
	} else {
		buildConfig.ProjectPath = cmd.Flags["project"]
	}

	// Get the output path from flags else default to "build"
	if cmd.Flags["output"] == "" {
		if cmd.Flags["o"] == "" {
			buildConfig.Output = "build"
		} else {
			buildConfig.Output = cmd.Flags["o"]
		}
	} else {
		buildConfig.Output = cmd.Flags["output"]
	}

	// Get the services path from flags else default to "services"
	if cmd.Flags["services"] == "" {
		if cmd.Flags["s"] == "" {
			buildConfig.ServicesFolder = "services"
		} else {
			buildConfig.ServicesFolder = cmd.Flags["s"]
		}
	} else {
		buildConfig.ServicesFolder = cmd.Flags["services"]
	}

	// Get the config path from flags else default to "config"
	if cmd.Flags["config"] == "" {
		if cmd.Flags["c"] == "" {
			buildConfig.ConfigFolder = "config"
		} else {
			buildConfig.ConfigFolder = cmd.Flags["c"]
		}
	} else {
		buildConfig.ConfigFolder = cmd.Flags["config"]
	}

	// Get the build type from flags else default to "dev"
	// Options ("dev", "prod")
	if cmd.Flags["build-type"] == "" {
		if cmd.Flags["t"] == "" {
			buildConfig.BuildType = "dev"
		} else {
			buildConfig.BuildType = cmd.Flags["t"]
		}
	} else {
		buildConfig.BuildType = cmd.Flags["build-type"]
	}

	return buildConfig, nil
}
