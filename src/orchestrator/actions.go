package orchestrator

import (
	"fmt"
	"odm/git"
	"odm/installer"
	"odm/plugin"
	"odm/utils"
	"slices"
	"strings"
)

/*
ACTIONS: actions performed by main cli

  - RemoveProject: removes git submodule and project from config
  - AddProject: add git submodule and project
  - install plugin: install plugin
*/

// check if core action exists
func (o *Orchestrator) IsCoreAction(cmd string) bool {
	actionsList := []string{"add", "remove", "install"}
	return slices.Contains(actionsList, cmd)
}

// initialize action list
func (o *Orchestrator) ExecuteCoreAction(cmd *utils.Command) error {
	var err error
	switch cmd.Name {
	case "add":

		// validate input
		if len(cmd.Args) < 2 {
			return fmt.Errorf("insufficient arguments passed")
		}

		// Get values to pass to func
		repoUrl := cmd.Args[0]
		destinationPath := cmd.Args[1]
		projectType := cmd.Flags["type"]
		if projectType == "" {
			projectType = "project"
		}

		// execute core action
		err = o.AddProject(repoUrl, destinationPath, projectType)
		return err

	case "remove":
		// validate input
		if len(cmd.Args) < 1 {
			return fmt.Errorf("insufficient arguments passed")
		}

		// get first arg (should be the project name to remove)
		projectName := cmd.Args[0]
		err = o.RemoveProject(projectName)
		return err
	case "install":
		// validate input
		if len(cmd.Args) < 2 {
			return fmt.Errorf("insufficient arguments passed")
		}
		installType := cmd.Args[0]
		packageName := cmd.Args[1]
		opts := installer.PluginInstallOptions{
			RootPath:     cmd.Flags["root-path"],
			PluginFolder: ".odm/plugins",
			Plugin: plugin.PluginDeclaration{
				Type:    installType,
				Package: packageName,
			},
		}
		err = installer.InstallPlugin(opts)
		if err != nil {
			return err
		}
		return nil

	default:
		return fmt.Errorf("core action not defined")
	}

}

// removes project sub module and config declaration by project name
func (o *Orchestrator) RemoveProject(projectName string) error {

	// if project name is empty return error
	if projectName == "" {
		return fmt.Errorf("project name not provided")
	}

	// get project declaraton
	var project *Project
	if p, ok := o.Config.Projects[projectName]; ok {
		project = &p
	}

	// Check project declaration exists
	if project.Name == "" {
		return fmt.Errorf("project: %s not found", projectName)
	}

	// remove submodule
	err := git.RemoveGitSubmodule(o.RootPath, project.Path)
	if err != nil {
		return err
	}

	// remove project from config
	err = o.RemoveProject(projectName)
	if err != nil {
		return err
	}

	return err

}

func (o *Orchestrator) AddProject(projectUrl string, projectPath string, projectType string) error {

	// validate input
	if projectUrl == "" || projectPath == "" {
		return fmt.Errorf("project url/path not provided")
	}

	// Add git repo as submodule
	err := git.AddGitSubmodule(o.RootPath, projectUrl, projectPath)
	if err != nil {
		return err
	}

	// create project struct
	pathPart := strings.Split(projectPath, "/")
	projectName := pathPart[len(pathPart)-1]

	newProject := &Project{
		Name: projectName,
		Repo: projectUrl,
		Path: projectPath,
		Type: projectType,
	}

	// Add project to config
	err = o.UpdateProject(newProject)
	if err != nil {
		return err
	}

	return nil
}
