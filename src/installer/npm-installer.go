package installer

import (
	"context"
	"encoding/json"
	"fmt"
	"odm/plugin"
	"os"
	"os/exec"
	"path/filepath"
)

// Package.json
type PackageJson struct {
	Main string `json:"main"` // path to executable
}

// NPMInstaller implements the PluginInstaller interface for npm.
type NPMInstaller struct{}

func (i *NPMInstaller) Install(ctx context.Context, plugin plugin.PluginDeclaration, opts PluginInstallOptions) error {
	pluginFolderPath := filepath.Join(opts.RootPath, opts.PluginFolder)

	// Construct the command to run, e.g., 'npm install @my-plugins/javascript-plugin@1.2.3'
	pkgSpec := plugin.Package
	if plugin.Version != "" {
		pkgSpec = fmt.Sprintf("%s@%s", plugin.Package, plugin.Version)

	}
	cmd := exec.CommandContext(ctx, "npm", "install", pkgSpec)

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
	var packageJson PackageJson
	packageJsonPath := filepath.Join(pluginFolderPath, "package.json")
	bytesData, err := os.ReadFile(packageJsonPath)
	if err != nil {
		return err
	}
	err = json.Unmarshal(bytesData, &packageJson)
	if err != nil {
		return err
	}

	// check main file exists and has a value in pckage.json
	if packageJson.Main == "" {
		return fmt.Errorf("executable path not provided in package.json file under key: main")
	}

	// Create paths
	packagePath := filepath.Join(pluginFolderPath, "node_modules", plugin.Package)
	declarationPath := filepath.Join(packagePath, "plugin.json")
	executeableFilePath := filepath.Join(packagePath, packageJson.Main)

	// Install plugin declaration
	err = InstallPluginDeclaration(pluginFolderPath, declarationPath, executeableFilePath)
	if err != nil {
		return err
	}

	return nil
}
