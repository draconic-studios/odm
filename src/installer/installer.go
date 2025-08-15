package installer

import (
	"context"
	"fmt"
	"odm/plugin"
	"path/filepath"
	"strings"
)

type Installation struct {
	RootPath                  string // Ochestrator Path (./)
	PluginFolder              string // Path to plugins (.odm/plugins)
	PluginDeclarationListPath string
	Declaration               plugin.PluginDeclaration
	Ctx                       context.Context
}

// PluginInstaller is an interface for installing plugins from different sources.
type PluginInstaller interface {
	Install(ctx context.Context, pluginSource plugin.PluginDeclaration, opts PluginInstallOptions) error
}

// initialize Installation struct
func (i *Installation) initInstaller() error {

	// Pre installation
	err := i.PreInstall()
	if err != nil {
		return err
	}

	i.Ctx = context.Background()
	return nil
}

// Options for the install
type PluginInstallOptions struct {
	RootPath     string
	PluginFolder string
	Plugin       plugin.PluginDeclaration
}

// Get installer
func (i *Installation) start() error {
	switch strings.ToLower(i.Declaration.Type) {
	case "npm":
		return i.NpmInstall()

	default:
		return fmt.Errorf("unknown installation type")
	}

}

// installPlugin handles the main logic of finding the right installer.
func InstallPlugin(opts PluginInstallOptions) error {

	// Init installation
	ins := &Installation{
		RootPath:                  opts.RootPath,
		PluginFolder:              opts.PluginFolder,
		Declaration:               opts.Plugin,
		PluginDeclarationListPath: filepath.Join(opts.RootPath, opts.PluginFolder, "plugins", "plugins.json"),
	}

	err := ins.initInstaller()
	if err != nil {
		return err
	}

	fmt.Printf("Installing plugin %s from %s...\n", opts.Plugin.Name, opts.Plugin.Type)
	return ins.start()
}
