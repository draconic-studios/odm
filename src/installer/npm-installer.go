package installer

import (
	"encoding/json"
	"fmt"
	"odm/plugin"
	"os"
	"os/exec"
	"path/filepath"
)

func (i *Installation) NpmInstall() error {
	pluginFolderPath := filepath.Join(i.RootPath, i.PluginFolder)

	// Construct the command to run, e.g., 'npm install @my-plugins/javascript-plugin@1.2.3'
	pkgSpec := i.Declaration.Package
	if i.Declaration.Version != "" {
		pkgSpec = fmt.Sprintf("%s@%s", i.Declaration.Package, i.Declaration.Version)

	}
	cmd := exec.CommandContext(i.Ctx, "npm", "install", pkgSpec)

	// Set the working directory to where you want to install plugins.
	// For example, a dedicated 'plugins' directory.
	cmd.Dir = pluginFolderPath

	// Capture and print output for debugging or logging.
	output, err := cmd.CombinedOutput()
	if err != nil {
		return fmt.Errorf("failed to install plugin %s: %w\nOutput: %s", pkgSpec, err, output)
	}

	fmt.Printf("Installed npm package %s\n", pkgSpec)

	// Read and parse package.json
	var pluginDeclaration *plugin.PluginDeclaration
	pluginDeclarationPath := filepath.Join(pluginFolderPath, "plugin.json")
	bytesData, err := os.ReadFile(pluginDeclarationPath)
	if err != nil {
		return err
	}
	err = json.Unmarshal(bytesData, &pluginDeclaration)
	if err != nil {
		return err
	}

	if pluginDeclaration.Source == "" {
		return fmt.Errorf("executable path not provided in plugin declaration json")
	}

	// Install plugin declaration
	err = i.InstallPluginDeclaration()
	if err != nil {
		return err
	}

	return nil
}
