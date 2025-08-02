package installer

import (
	"encoding/json"
	"odm/plugin"
	"os"
	"path/filepath"
)

// Copy plugin definition into def folder
func InstallPluginDeclaration(pluginFolderPath string, declarationPath string, executablePath string) error {

	// Read declaration file
	var declaration plugin.PluginDeclaration

	bytesData, err := os.ReadFile(declarationPath)
	if err != nil {
		return err
	}
	err = json.Unmarshal(bytesData, &declaration)
	if err != nil {
		return err
	}

	// Get name of plugin
	pluginName := declaration.Name

	// Output path
	declaration.Source = executablePath
	destination := filepath.Join(pluginFolderPath, "definitions", pluginName+".json")

	// Change source field to executable path

	// Copy file into defs folder
	bytes, err := json.Marshal(declaration)
	if err != nil {
		return err
	}

	// Write declaration file into definitions folder
	err = os.WriteFile(destination, bytes, 0644)
	if err != nil {
		return err
	}

	return nil
}
