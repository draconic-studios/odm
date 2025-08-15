package installer

import (
	"encoding/json"
	"odm/plugin"
	"os"
	"path/filepath"
)

func (i *Installation) UpdateDeclarationList() error {

	bytes, err := os.ReadFile(i.PluginDeclarationListPath)
	if err != nil {
		return err
	}

	var pd *PluginDeclarationList

	err = json.Unmarshal(bytes, &pd)
	if err != nil {
		return err
	}

	pd.Plugins[i.Declaration.Name] = i.Declaration

	bytreData, err := json.Marshal(pd)
	if err != nil {
		return err
	}

	// Write declaration file into definitions folder
	err = os.WriteFile(i.PluginDeclarationListPath, bytreData, 0644)
	if err != nil {
		return err
	}

	return nil

}

// Copy plugin definition into def folder
func (i *Installation) InstallPluginDeclaration() error {

	packagePath := filepath.Join(i.RootPath, i.PluginFolder, "plugins", "node_modules", i.Declaration.Package)
	packageDeclarationPath := filepath.Join(packagePath, "plugin.json")
	// Read declaration file
	var declaration plugin.PluginDeclaration
	bytesData, err := os.ReadFile(packageDeclarationPath)
	if err != nil {
		return err
	}
	err = json.Unmarshal(bytesData, &declaration)
	if err != nil {
		return err
	}
	i.Declaration = declaration

	// Get name of plugin
	// Output path
	destination := filepath.Join(i.RootPath, i.PluginFolder, "plugins", "definitions", i.Declaration.Name+".json")

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

	return i.UpdateDeclarationList()
}
