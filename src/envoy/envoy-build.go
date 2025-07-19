package envoy

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"odm/types"
	"odm/utils"

	"gopkg.in/yaml.v3"
)

type EnvoyCompiler struct {
	Config   *types.ExecuterOptions
	Services []string

	store *types.EnvoyConfig
}

func (ec *EnvoyCompiler) Build() error {
	if ec.Config.ProjectPath == "" {
		return fmt.Errorf("project path not set")
	}
	if ec.Config.Output == "" {
		return fmt.Errorf("output path not set")
	}

	err := ec.getTemplates()
	if err != nil {
		return fmt.Errorf(
			"error retreiving envoy template file:\n\terror: %s",
			err,
		)
	}

	err = ec.GetServices()
	if err != nil {
		return fmt.Errorf(
			"error retreiving envoy config for service level:\n\terror: %s",
			err,
		)
	}
	err = ec.WriteConfigYaml()
	if err != nil {
		return fmt.Errorf(
			"error Writing envoy config file:\n\terror: %s",
			err,
		)
	}

	return nil
}
func (ec *EnvoyCompiler) getTemplates() error {
	templatesPath := fmt.Sprintf(
		"%s/%s/%s/envoy.base.yaml",
		ec.Config.ProjectPath,
		ec.Config.ConfigFolder,
		ec.Config.BuildType,
	)
	baseConfig, err := ec.ReadEnvoyBaseTemplate(templatesPath)
	if err != nil {
		return err
	}
	ec.store = baseConfig
	return nil

}

func (ec *EnvoyCompiler) GetServices() error {
	for _, s := range ec.Services {
		fmt.Println("Service", s)
		serviceConfig, err := ec.ReadEnvoyServiceConfig(s)
		if err != nil {
			return err
		}
		ec.store.StaticResources.Clusters = append(ec.store.StaticResources.Clusters, serviceConfig.Clusters...)
		ec.store.StaticResources.Listeners[0].FilterChains[0].Filters[0].TypedConfig.RouteConfig.VirtualHosts[0].Routes = append(ec.store.StaticResources.Listeners[0].FilterChains[0].Filters[0].TypedConfig.RouteConfig.VirtualHosts[0].Routes, serviceConfig.Routes...)

	}
	return nil
}

func (ec *EnvoyCompiler) WriteConfigYaml() error {
	// 2. Marshal the Go struct into YAML bytes
	yamlBytes, err := yaml.Marshal(ec.store)
	if err != nil {
		return err
	}

	// 4. Write the YAML bytes to the file
	// os.WriteFile handles opening, writing, and closing the file.
	// 0644 are file permissions (read/write for owner, read for group/others)
	outputFilePath := fmt.Sprintf(
		"%s/%s/volumes/api-gateway/envoy.yaml",
		ec.Config.ProjectPath,
		ec.Config.Output,
	)
	err = os.WriteFile(outputFilePath, yamlBytes, 0644)
	if err != nil {
		return err
	}
	fmt.Printf("\nSuccessfully wrote envoy config file:\n--Path: %s\n\n", outputFilePath)
	return nil
}

func (ec *EnvoyCompiler) ReadEnvoyBaseTemplate(templatePath string) (*types.EnvoyConfig, error) {
	// Read the YAML file
	data, err := os.ReadFile(templatePath)
	if err != nil {
		return nil, err
	}

	var config types.EnvoyConfig
	err = yaml.Unmarshal(data, &config)
	if err != nil {
		return nil, err
	}

	return &config, nil

}

func (ec *EnvoyCompiler) ReadEnvoyServiceConfig(serviceName string) (*types.ServiceDeclaration, error) {
	service := &types.ServiceDeclaration{Clusters: []types.Cluster{}, Routes: []types.Route{}}
	// Read the YAML file
	clustersPath := fmt.Sprintf("%s/%s/%s/envoy/clusters", ec.Config.ProjectPath, ec.Config.ServicesFolder, serviceName)
	routesPath := fmt.Sprintf("%s/%s/%s/envoy/routes", ec.Config.ProjectPath, ec.Config.ServicesFolder, serviceName)

	// Get all cluster files in cluster folder
	clusters, err := utils.ReadFolderContents(clustersPath)
	if err != nil {
		return nil, err
	}
	var clusterFiles []string
	for _, entry := range *clusters {

		if entry.IsDir() {
			continue
		} else {

			fileName := entry.Name()
			ext := strings.ToLower(filepath.Ext(fileName)) // Get extension and convert to lowercase

			if ext == ".yaml" || ext == ".yml" {
				clusterFiles = append(clusterFiles, filepath.Join(clustersPath, fileName))
			}
		}
	}

	for _, c := range clusterFiles {

		cluster, err := os.ReadFile(c)
		if err != nil {
			return nil, err
		}
		var config types.ServiceDeclaration
		err = yaml.Unmarshal(cluster, &config)
		if err != nil {
			return nil, err
		}
		service.Clusters = append(service.Clusters, config.Clusters...)
	}

	routes, err := utils.ReadFolderContents(routesPath)
	if err != nil {
		return nil, err
	}
	var routesFiles []string
	for _, entry := range *routes {

		if entry.IsDir() {
			continue
		} else {
			fileName := entry.Name()
			ext := strings.ToLower(filepath.Ext(fileName)) // Get extension and convert to lowercase

			if ext == ".yaml" || ext == ".yml" {
				routesFiles = append(routesFiles, filepath.Join(routesPath, fileName))
			}
		}
	}

	for _, r := range routesFiles {

		route, err := os.ReadFile(r)
		if err != nil {
			return nil, err
		}
		var config types.ServiceDeclaration
		err = yaml.Unmarshal(route, &config)
		if err != nil {
			return nil, err
		}
		service.Routes = append(service.Routes, config.Routes...)
	}

	return service, nil
}
