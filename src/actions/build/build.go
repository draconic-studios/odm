package build

import (
	"odm/utils"
)

// PROCESS
// 1. Build File system / directory
// 2. Build docker images

func Build(basePath string) error {

	// Check base path is valid
	err := utils.FolderExists(basePath)
	if err != nil {
		return err
	}

	// Build FS
	folders := [][]string{
		{"build", "volumes", "api-gateway"},
		{"build", "volumes", "postgresql-data"},
		{"build", "config"},
		{"build", "docker"},
	}

	for _, dir := range folders {
		err = utils.CreateFolders(basePath, dir)
		if err != nil {
			return err
		}
	}

	return nil

}
