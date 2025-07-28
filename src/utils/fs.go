package utils

import (
	"fmt"
	"io"
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

func CreateFolder(basePath string, folder string) error {
	// Path with multiple levels
	dirPath := filepath.Join(basePath, folder) // Cross-platform path construction
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

// Copy folder contents recursively (preserves directory structure)
func CopyFolderContents(src, dst string) error {
	// Create destination directory if it doesn't exist
	if err := os.MkdirAll(dst, 0755); err != nil {
		return fmt.Errorf("failed to create destination directory: %w", err)
	}

	// Walk through source directory
	return filepath.Walk(src, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}

		// Calculate relative path from source
		relPath, err := filepath.Rel(src, path)
		if err != nil {
			return err
		}

		// Skip the root directory itself
		if relPath == "." {
			return nil
		}

		// Construct destination path
		dstPath := filepath.Join(dst, relPath)

		if info.IsDir() {
			// Create directory in destination
			return os.MkdirAll(dstPath, info.Mode())
		} else {
			// Copy file
			return CopyFile(path, dstPath)
		}
	})
}

// Helper function to copy individual files
func CopyFile(src, dst string) error {
	// Open source file
	srcFile, err := os.Open(src)
	if err != nil {
		return fmt.Errorf("failed to open source file %s: %w", src, err)
	}
	defer srcFile.Close()

	// Get source file info for permissions
	srcInfo, err := srcFile.Stat()
	if err != nil {
		return fmt.Errorf("failed to get source file info: %w", err)
	}

	// Create destination file
	dstFile, err := os.OpenFile(dst, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, srcInfo.Mode())
	if err != nil {
		return fmt.Errorf("failed to create destination file %s: %w", dst, err)
	}
	defer dstFile.Close()

	// Copy file contents
	_, err = io.Copy(dstFile, srcFile)
	if err != nil {
		return fmt.Errorf("failed to copy file contents: %w", err)
	}

	return nil
}

func WriteFile(filePath string, content *string) error {
	return os.WriteFile(filePath, []byte(*content), 0644)

}
