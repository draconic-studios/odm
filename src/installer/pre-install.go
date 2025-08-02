package installer

import (
	"odm/utils"
	"path/filepath"
)

var packageJson string = `
{
  "name": "plugins",
  "version": "1.0.0"
}
`

func PreInstall(opts PluginInstallOptions) error {

	// create the plugins folder
	err := utils.CreateFolder(opts.RootPath, opts.PluginFolder)
	if err != nil {
		return err
	}
	// plugin definitions
	err = utils.CreateFolder(filepath.Join(opts.RootPath, ".plugins"), "definitions")
	if err != nil {
		return err
	}

	// ==========================================
	// Node Req
	// ==========================================
	// write package.json for node plugins
	err = utils.WriteFile(filepath.Join(opts.RootPath, "package.json"), &packageJson)
	if err != nil {
		return err
	}

	return nil
}
