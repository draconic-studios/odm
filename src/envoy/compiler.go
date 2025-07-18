package envoy

import (
	"fmt"
	"odm/utils"
	"os"

	"gopkg.in/yaml.v3"
)

type EnvoyCompiler struct {
	basePath      string
	outputPath    string
	servicePaths  []string
	templatesPath string

	store *utils.EnvoyConfig
}

func NewEnvoyCompiler(
	basePath string,
	outputPath string,
	servicePaths []string, templatesPath string,
) *EnvoyCompiler {
	return &EnvoyCompiler{
		basePath:      basePath,
		outputPath:    outputPath,
		servicePaths:  servicePaths,
		templatesPath: templatesPath,
	}
}

func (ec *EnvoyCompiler) Build() error {
	if ec.basePath == "" {
		return fmt.Errorf("base path not set")
	}
	if ec.outputPath == "" {
		return fmt.Errorf("output path not set")
	}
	if ec.templatesPath == "" {
		return fmt.Errorf("templates path not set")
	}

	err := ec.getTemplates()
	if err != nil {
		return err
	}

	err = ec.GetServices()
	if err != nil {
		return err
	}
	err = ec.WriteConfigYaml()
	if err != nil {
		return err
	}

	return nil
}
func (ec *EnvoyCompiler) getTemplates() error {

	baseConfig, err := utils.ReadEnvoyBaseTemplate(ec.templatesPath)
	if err != nil {
		return err
	}
	ec.store = baseConfig
	return nil

}

func (ec *EnvoyCompiler) GetServices() error {
	for _, s := range ec.servicePaths {
		serviceConfig, err := utils.ReadEnvoyServiceConfig(s)
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

	// Optional: Print the YAML string to console
	fmt.Println("--- Generated YAML Content ---")
	fmt.Println(string(yamlBytes))
	fmt.Println("----------------------------")

	// 3. Define the output file path
	outputFilePath := "output_config.yaml"

	// 4. Write the YAML bytes to the file
	// os.WriteFile handles opening, writing, and closing the file.
	// 0644 are file permissions (read/write for owner, read for group/others)
	err = os.WriteFile(outputFilePath, yamlBytes, 0644)
	if err != nil {
		return err
	}
	return nil
}
