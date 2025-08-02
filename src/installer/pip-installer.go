package installer

import (
	"context"
	"fmt"
	"odm/plugin"
	"os/exec"
)

// ! Stopped til odm works then pip package support with start

// PipInstaller implements the PluginInstaller interface for pip.
type PipInstaller struct{}

func (i *PipInstaller) Install(ctx context.Context, plugin plugin.PluginDeclaration, opts PluginInstallOptions) error {
	// Construct the command, e.g., 'pip install my-python-plugin==0.5.0'
	pkgSpec := plugin.Package
	if plugin.Version != "" {
		pkgSpec = fmt.Sprintf("%s@%s", plugin.Package, plugin.Version)

	}
	cmd := exec.CommandContext(ctx, "pip", "install", pkgSpec)
	cmd.Dir = "./plugins" // Example working directory

	output, err := cmd.CombinedOutput()
	if err != nil {
		return fmt.Errorf("failed to install plugin %s: %w\nOutput: %s", pkgSpec, err, output)
	}

	fmt.Printf("Successfully installed pip plugin %s\n", pkgSpec)
	return nil
}
