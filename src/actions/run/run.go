package run

import (
	"fmt"
	"odm/types"
	"odm/utils"
	"os"
	"os/exec"
	"path/filepath"
)

// Run system
func Run(opts *types.RunOptions) error {
	systemPath := filepath.Join(opts.ProjectPath, opts.SystemFolder)

	// Check main path is valid
	err := utils.FolderExists(systemPath)
	if err != nil {
		return fmt.Errorf("error system folder not found: %w", err)
	}

	// Check docker-compose.yml exists
	composeFilePath := filepath.Join(systemPath, opts.DockerComposeFileName)
	err = utils.FileExists(composeFilePath)
	if err != nil {
		return fmt.Errorf("error docker-compose.yml not found: %w", err)
	}

	fmt.Printf(
		"\n\nStarting System:\n\tSystem Path: %s\n\tDocker-Compose File: %s\n\tDocker-Compose Path: %s\n\tAttach Mode: %t\n\n",
		systemPath,
		opts.DockerComposeFileName,
		composeFilePath,
		opts.Attach,
	)

	// Create the command.
	//  -f to specify the compose file path.
	//  --project-directory to set the working directory for Docker Compose,
	// which is crucial for picking up the .env file in 'build'.
	composeCommand := exec.Command(
		"docker",
		"compose",
		"-f",
		opts.DockerComposeFileName,
		"--project-directory",
		systemPath,
		"up",
		"--build",
		"-d",
	)

	// Set the PWD
	composeCommand.Dir = systemPath

	// Optional: Attach stdout and stderr to the current process for real-time output
	composeCommand.Stdout = os.Stdout
	composeCommand.Stderr = os.Stderr

	// Run the command
	fmt.Printf("Executing Command: %s\n\n", composeCommand)
	err = composeCommand.Run()
	if err != nil {
		return fmt.Errorf("failed to start system: %w", err)
	}

	fmt.Println("System started.")
	return nil
}
