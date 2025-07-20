package build

import (
	"fmt"
	"odm/docker"
	"odm/environment"
	"odm/envoy"
	"odm/types"
	"odm/utils"
	"path/filepath"
	"strings"
)

// PROCESS
// 1. Valid Config/Args passed
// 2. Build File system / directory
// 3. Build API gateway config (Envoy Proxy)
// 4. Build docker-compose.yml file
// 5. Build env file

// Execute System Build
func Build(buildConfig *types.BuildOptions) error {
	// STEP 1:
	fmt.Println("Executing System Build")
	// Check if config is valid
	var errorList []string
	if buildConfig.ProjectPath == "" {
		errorList = append(errorList, "base path not set")
	}
	if buildConfig.Output == "" {
		errorList = append(errorList, "output path not set")
	}
	if buildConfig.ServicesFolder == "" {
		errorList = append(errorList, "templates path not set")
	}
	if buildConfig.ConfigFolder == "" {
		errorList = append(errorList, "templates path not set")
	}
	if buildConfig.BuildType == "" {
		errorList = append(errorList, "build type not set")
	}

	// Check project path is valid
	err := utils.FolderExists(buildConfig.ProjectPath)
	if err != nil {
		errorList = append(errorList, "project does not exist")
	}

	if len(errorList) > 0 {
		return fmt.Errorf("errors in build config: \n\t%s", strings.Join(errorList, "\n\t"))
	}

	// Get project definination
	project, err := utils.ReadProject(buildConfig.ProjectPath + "/project.json")
	if err != nil {
		return err
	}

	// Get list of system services (this excludes services like mobile apps)
	var systemServices []string
	for _, s := range project.Services {
		if s.Type == "system service" {
			systemServices = append(systemServices, s.Name)

		}
	}
	buildConfig.Services = systemServices

	// STEP 2:

	fmt.Println("building system directories")
	// Build FS
	folders := [][]string{
		{"build", "volumes", "api-gateway"},
		{"build", "volumes", "postgresql-data"},
		{"build", "config"},
		{"build", "docker"},
	}

	for _, dir := range folders {
		err = utils.CreateFolders(buildConfig.ProjectPath, dir)
		if err != nil {
			return err
		}
	}

	// Copy Creds over
	configFolderPath := filepath.Join(buildConfig.ProjectPath, buildConfig.ConfigFolder, buildConfig.BuildType, "creds")
	outputConfigFolderPath := filepath.Join(buildConfig.ProjectPath, buildConfig.Output, buildConfig.ConfigFolder)
	err = utils.CopyFolderContents(configFolderPath, outputConfigFolderPath)
	if err != nil {
		return nil
	}
	// Get Services

	// STEP 3:

	// Build API gateway config (Envoy Proxy)
	envoyBuilder := &envoy.EnvoyCompiler{
		Config: buildConfig,
	}

	err = envoyBuilder.Build()
	if err != nil {
		return fmt.Errorf("error building api gateway configuration file: %s", err)
	}

	// STEP 4:
	fmt.Println("Building Docker Compose file")
	composeBuilder := &docker.DockerComposeCompiler{
		Config: buildConfig,
	}

	err = composeBuilder.Build()
	if err != nil {
		return err
	}

	// STEP 5:
	fmt.Println("Building .env file")

	err = environment.EnvBuilder(buildConfig)
	if err != nil {
		return err
	}

	return nil

}
