package run

import (
	"odm/types"
	"os"
)

// Get Build config from command
func ParseCommand(cmd *types.Command) (*types.RunOptions, error) {
	runOpts := &types.RunOptions{
		ProjectPath:           "",
		SystemFolder:          "",
		DockerComposeFileName: "",
		Attach:                false,
	}

	// Get the the project path from flags else default to "${pwd}"
	if cmd.Flags["project"] == "" {
		if cmd.Flags["p"] == "" {
			pwd, err := os.Getwd()
			if err != nil {
				return nil, err
			}
			runOpts.ProjectPath = pwd
		} else {
			runOpts.ProjectPath = cmd.Flags["p"]
		}
	} else {
		runOpts.ProjectPath = cmd.Flags["project"]
	}

	// Get the attach  flags else default to false
	if cmd.BoolFlags["attach"] {
		runOpts.Attach = cmd.BoolFlags["attach"]
	} else if cmd.BoolFlags["a"] {
		runOpts.Attach = cmd.BoolFlags["a"]
	}

	// Get the system path from flags else default to "build"
	if cmd.Flags["system"] == "" {
		if cmd.Flags["s"] == "" {
			runOpts.SystemFolder = "build"
		} else {
			runOpts.SystemFolder = cmd.Flags["s"]
		}
	} else {
		runOpts.SystemFolder = cmd.Flags["system"]
	}

	// Get the config path from flags else default to "config"
	if cmd.Flags["docker-conpose"] == "" {
		if cmd.Flags["c"] == "" {
			runOpts.DockerComposeFileName = "docker/docker-compose.yml"
		} else {
			runOpts.DockerComposeFileName = cmd.Flags["c"]
		}
	} else {
		runOpts.DockerComposeFileName = cmd.Flags["config"]
	}

	return runOpts, nil
}
