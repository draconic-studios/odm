package installer

import (
	"context"
	"fmt"
	"odm/plugin"
)

// PluginInstaller is an interface for installing plugins from different sources.
type PluginInstaller interface {
	Install(ctx context.Context, pluginSource plugin.PluginDeclaration, opts PluginInstallOptions) error
}

// Options for the install
type PluginInstallOptions struct {
	RootPath     string
	PluginFolder string
}

// installPlugin handles the main logic of finding the right installer.
func InstallPlugin(opts PluginInstallOptions, plugin plugin.PluginDeclaration) error {
	var ctx context.Context

	// Pre installation
	err := PreInstall(opts)
	if err != nil {
		return err
	}

	// A map to hold our different installers, keyed by the source type.
	installers := map[string]PluginInstaller{
		"npm": &NPMInstaller{},
		// "pypi": &PipInstaller{}, // ! Stopped til odm works then pip package support with start
		// Add other installers here...
	}

	installer, ok := installers[plugin.Type]
	if !ok {
		return fmt.Errorf("unsupported plugin type: %s", plugin.Type)
	}

	fmt.Printf("Installing plugin %s from %s...\n", plugin.Package, plugin.Type)
	return installer.Install(ctx, plugin, opts)
}
