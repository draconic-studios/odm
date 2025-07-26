package actions

import (
	"fmt"
	"odm/utils"
	"os"
	"os/exec"
	"path/filepath"
)

// AddGitSubmodule adds a Git repository as a submodule to the current repository.
//
// parentRepoPath: The absolute or relative path to the parent Git repository.
//
//	If empty, the current working directory is assumed.
//
// submoduleURL:   The URL of the Git repository to add as a submodule.
// submodulePath:  The local path within the parent repository where the submodule
//
//	should be added (e.g., "my-submodule-folder").
func AddGitSubmodule(parentRepoPath, submoduleURL, submodulePath string) (string, error) {
	// Determine the directory to execute the git command in.
	// If parentRepoPath is empty, we assume the current working directory.
	var cmdDir string
	if parentRepoPath != "" {
		absPath, err := filepath.Abs(parentRepoPath)
		if err != nil {
			return "", fmt.Errorf("failed to get absolute path for parent repository: %w", err)
		}
		cmdDir = absPath
	} else {
		currentDir, err := os.Getwd()
		if err != nil {
			return "", fmt.Errorf("failed to get current working directory: %w", err)
		}
		cmdDir = currentDir
	}

	// Change to the parent repository directory before executing the git command
	// to ensure it's executed in the correct context.
	originalDir, err := os.Getwd()
	if err != nil {
		return "", fmt.Errorf("failed to get original working directory: %w", err)
	}
	defer os.Chdir(originalDir) // Defer changing back to the original directory

	if err := os.Chdir(cmdDir); err != nil {
		return "", fmt.Errorf("failed to change directory to %s: %w", cmdDir, err)
	}

	fmt.Printf("Attempting to add submodule '%s' from '%s' into '%s' in directory '%s'\n", submodulePath, submoduleURL, cmdDir, submodulePath)

	// Construct the git submodule add command
	cmd := exec.Command("git", "submodule", "add", submoduleURL, submodulePath)

	// Capture stdout and stderr for better error reporting
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	// Run the command
	if err := cmd.Run(); err != nil {
		return "", fmt.Errorf("failed to add git submodule: %w", err)
	}

	return fmt.Sprintf("Successfully added submodule '%s' from '%s' to '%s'\n", submodulePath, submoduleURL, cmdDir), nil
}

func Add(command *utils.Command) (string, error) {

	if len(command.Args) < 2 {
		return "", fmt.Errorf("insufficient arguments passed")
	}
	repoUrl := command.Args[0]
	destinationPath := command.Args[1]

	result, err := AddGitSubmodule(command.Flags["project-path"], repoUrl, destinationPath)

	return result, err
}
