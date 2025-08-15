package installer

import (
	"encoding/json"
	"odm/plugin"
	"odm/utils"
	"path/filepath"
)

type PackageJson struct {
	Name    string `json:"name"`
	Version string `json:"version"`
}

type PluginDeclarationList struct {
	Plugins map[string]plugin.PluginDeclaration `json:"plugins"`
}

// Init for node use
func initNodeReq(pluginFolder string) error {
	// ==========================================
	// Node Req
	// ==========================================
	// write package.json for node plugins
	packageJsonPath := filepath.Join(pluginFolder, "package.json")
	err := utils.FileExists(packageJsonPath)
	if err != nil {
		newPackageJson := PackageJson{
			Name:    "plugins",
			Version: "1.0.0",
		}
		dataBytes, err := json.Marshal(newPackageJson)
		if err != nil {
			return err
		}
		jsonString := string(dataBytes)

		err = utils.WriteFile(packageJsonPath, &jsonString)
		if err != nil {
			return err
		}
	}

	return nil
}

func initFs(rootPath string) error {

	// create the plugins folder
	err := utils.CreateFolder(rootPath, filepath.Join(".odm", "plugins", "definitions"))
	if err != nil {
		return err
	}

	// Create plugins json if it doesn't exist
	pdPath := filepath.Join(rootPath, ".odm", "plugins", "plugins.json")
	err = utils.FileExists(pdPath)
	if err != nil {
		newPdList := PluginDeclarationList{
			Plugins: map[string]plugin.PluginDeclaration{},
		}

		dataBytes, err := json.Marshal(newPdList)
		if err != nil {
			return err
		}
		pdString := string(dataBytes)

		err = utils.WriteFile(pdPath, &pdString)
		if err != nil {
			return err
		}
	}

	return nil
}

func (i *Installation) PreInstall() error {
	// init file system
	err := initFs(i.RootPath)
	if err != nil {
		return err
	}

	// init fs for node/npm
	err = initNodeReq(filepath.Join(i.RootPath, ".odm", "plugins"))
	if err != nil {
		return err
	}

	return nil
}
