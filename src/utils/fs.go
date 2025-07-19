package utils

import (
	"fmt"
	"os"
	"path/filepath"
)

// check if a folder exists and is a directory by the path.
func FolderExists(path string) error {
	info, err := os.Stat(path) // Get file info for the given path
	if err != nil {
		return err
	}

	// If no error, check if it's a directory
	if !info.IsDir() {
		return fmt.Errorf("%s is not a folder", path)
	}
	return nil

}

// Check if a file exists by the path
func FileExists(path string) error {
	if _, err := os.Stat(path); os.IsNotExist(err) {
		return err
	}
	return nil
}

func CreateFolders(basePath string, folders []string) error {
	// Path with multiple levels
	foldersWithBasepPath := append([]string{basePath}, folders...)
	dirPath := filepath.Join(foldersWithBasepPath...) // Cross-platform path construction
	permissions := os.FileMode(0755)

	// Attempt to create the directory and all necessary parents
	err := os.MkdirAll(dirPath, permissions)
	if err != nil {
		// MkdirAll doesn't return an error if the directory already exists
		// so we only handle other types of errors here (e.g., permissions)
		return err
	} else {
		return nil
	}
}

func ReadFolderContents(folderPath string) (*[]os.DirEntry, error) {
	// Read directory entries
	entries, err := os.ReadDir(folderPath)
	if err != nil {
		return nil, err
	}
	return &entries, nil
}
