package git

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

// RemoveGitSubmodule removes a Git submodule from the current repository.
//
// parentRepoPath: The absolute or relative path to the parent Git repository.
//
// if empty, the current working directory is assumed.
//
// submodulePath:  The local path within the parent repository where the submodule
func RemoveGitSubmodule(parentRepoPath, submodulePath string) error {
	// Determine the directory to execute the git command in.
	var cmdDir string
	if parentRepoPath != "" {
		absPath, err := filepath.Abs(parentRepoPath)
		if err != nil {
			return fmt.Errorf("failed to get absolute path for parent repository: %w", err)
		}
		cmdDir = absPath
	} else {
		currentDir, err := os.Getwd()
		if err != nil {
			return fmt.Errorf("failed to get current working directory: %w", err)
		}
		cmdDir = currentDir
	}

	// Change to the parent repository directory before executing the git command.
	originalDir, err := os.Getwd()
	if err != nil {
		return fmt.Errorf("failed to get original working directory: %w", err)
	}
	defer os.Chdir(originalDir) // Defer changing back to the original directory

	if err := os.Chdir(cmdDir); err != nil {
		return fmt.Errorf("failed to change directory to %s: %w", cmdDir, err)
	}

	fmt.Printf("Attempting to remove submodule '%s' from '%s'\n", submodulePath, cmdDir)

	// Step 1: Deinitialize the submodule
	// This removes the submodule's entry from .git/config and clears its work tree.
	fmt.Printf("Step 1/5: Deinitializing submodule '%s'...\n", submodulePath)
	cmd := exec.Command("git", "submodule", "deinit", "-f", submodulePath)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	if err := cmd.Run(); err != nil {
		return fmt.Errorf("failed to deinitialize submodule '%s': %w", submodulePath, err)
	}

	// Step 2: Remove the submodule entry from .gitmodules file
	// We'll use 'git config --file .gitmodules --remove-section'
	// First, get the submodule name from its path (e.g., "my-submodule-folder" -> "my-submodule-folder")
	// For simple cases, the path is often the name, but technically Git uses the name from .gitmodules
	// which might be different if 'submodule.<name>.path' differs from 'name'.
	// A robust solution would read .gitmodules, but for simplicity, we assume path == name here.
	submoduleName := filepath.Base(submodulePath) // Extract last element of path as name for config
	fmt.Printf("Step 2/5: Removing submodule section from .gitmodules for '%s'...\n", submoduleName)
	cmd = exec.Command("git", "config", "-f", ".gitmodules", "--remove-section", "submodule."+submoduleName)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	// It might return an error if the section doesn't exist, which is fine if it was already removed.
	if err := cmd.Run(); err != nil {
		// Check if the error indicates "no such section"
		if strings.Contains(err.Error(), "no such section") {
			fmt.Printf("Warning: Submodule section 'submodule.%s' not found in .gitmodules (might already be removed).\n", submoduleName)
		} else {
			return fmt.Errorf("failed to remove section from .gitmodules for '%s': %w", submoduleName, err)
		}
	} else {
		fmt.Printf("Successfully removed section from .gitmodules for '%s'.\n", submoduleName)
	}

	// Step 3: Remove the submodule from the Git cache (index)
	fmt.Printf("Step 3/5: Removing submodule from Git cache '%s'...\n", submodulePath)
	cmd = exec.Command("git", "rm", "--cached", submodulePath)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	if err := cmd.Run(); err != nil {
		// If the submodule wasn't in the cache, it's not an error.
		if !strings.Contains(err.Error(), "did not match any files") {
			return fmt.Errorf("failed to remove submodule from Git cache '%s': %w", submodulePath, err)
		} else {
			fmt.Printf("Warning: Submodule '%s' not found in Git cache (might already be removed).\n", submodulePath)
		}
	}

	// Step 4: Remove the submodule's directory from .git/modules
	// This is where Git stores the actual Git repository for the submodule.
	gitModulesPath := filepath.Join(".git", "modules", submodulePath) // This path usually mirrors the submodule's path
	fmt.Printf("Step 4/5: Removing submodule's .git/modules directory '%s'...\n", gitModulesPath)
	if _, err := os.Stat(gitModulesPath); !os.IsNotExist(err) {
		// Directory exists, proceed with removal
		rmCmd := exec.Command("rm", "-rf", gitModulesPath)
		rmCmd.Stdout = os.Stdout
		rmCmd.Stderr = os.Stderr
		if err := rmCmd.Run(); err != nil {
			return fmt.Errorf("failed to remove .git/modules directory for '%s': %w", gitModulesPath, err)
		}
	} else {
		fmt.Printf("Warning: .git/modules directory '%s' not found (might already be removed or not initialized).\n", gitModulesPath)
	}

	// Step 5: Remove the actual submodule directory from the working tree
	fmt.Printf("Step 5/5: Removing submodule's working directory '%s'...\n", submodulePath)
	if _, err := os.Stat(submodulePath); !os.IsNotExist(err) {
		// Directory exists, proceed with removal
		rmCmd := exec.Command("rm", "-rf", submodulePath)
		rmCmd.Stdout = os.Stdout
		rmCmd.Stderr = os.Stderr
		if err := rmCmd.Run(); err != nil {
			return fmt.Errorf("failed to remove submodule working directory '%s': %w", submodulePath, err)
		}
	} else {
		fmt.Printf("Warning: Submodule working directory '%s' not found (might already be removed).\n", submodulePath)
	}

	fmt.Println("Submodule removal process complete. Remember to commit the changes!")
	return nil
}

// AddGitSubmodule adds a Git repository as a submodule to the current repository.
//
// parentRepoPath: The absolute or relative path to the parent Git repository.
//
// if empty, the current working directory is assumed.
//
// submoduleURL:   The URL of the Git repository to add as a submodule.
//
// submodulePath:  The local path within the parent repository where the submodule
func AddGitSubmodule(parentRepoPath, submoduleURL, submodulePath string) error {
	// Determine the directory to execute the git command in.
	// If parentRepoPath is empty, we assume the current working directory.
	var cmdDir string
	if parentRepoPath != "" {
		absPath, err := filepath.Abs(parentRepoPath)
		if err != nil {
			return fmt.Errorf("failed to get absolute path for parent repository: %w", err)
		}
		cmdDir = absPath
	} else {
		currentDir, err := os.Getwd()
		if err != nil {
			return fmt.Errorf("failed to get current working directory: %w", err)
		}
		cmdDir = currentDir
	}

	// Change to the parent repository directory before executing the git command
	// to ensure it's executed in the correct context.
	originalDir, err := os.Getwd()
	if err != nil {
		return fmt.Errorf("failed to get original working directory: %w", err)
	}
	defer os.Chdir(originalDir) // Defer changing back to the original directory

	if err := os.Chdir(cmdDir); err != nil {
		return fmt.Errorf("failed to change directory to %s: %w", cmdDir, err)
	}

	fmt.Printf("Attempting to add submodule '%s' from '%s' into '%s' in directory '%s'\n", submodulePath, submoduleURL, cmdDir, submodulePath)

	// Construct the git submodule add command
	cmd := exec.Command("git", "submodule", "add", submoduleURL, submodulePath)

	// Capture stdout and stderr for better error reporting
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	// Run the command
	if err := cmd.Run(); err != nil {
		return fmt.Errorf("failed to add git submodule: %w", err)
	}

	fmt.Printf("Successfully added submodule '%s' from '%s' to '%s'\n", submodulePath, submoduleURL, cmdDir)
	return nil
}
