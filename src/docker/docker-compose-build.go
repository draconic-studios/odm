package docker

import (
	"fmt"
	"maps"
	"odm/types"
	"os"
	"strings"

	"gopkg.in/yaml.v3"
)

type DockerComposeCompiler struct {
	Config *types.ExecuterOptions
	Store  *types.DockerCompose
}

// CombineDockerComposeAdvanced merges two DockerCompose structs with advanced merging options
type MergeOptions struct {
	// MergeServices determines if services with same name should be merged (true) or replaced (false)
	MergeServices bool
	// PreferFirst gives precedence to the first compose instead of the second
	PreferFirst bool
	// OnConflict is called when there's a naming conflict, returns true to use the conflicting item
	OnConflict func(itemType, name string, first, second interface{}) bool
}

// PROCESS
// 1. Get and parse base docker-compose.base.yml
// 2. Get and parse services docker-compose.yml files
// 3. Merge base and services
// 4. Write new compose to file

// Build systems compose file
func (dc *DockerComposeCompiler) Build() error {
	if dc.Config.ProjectPath == "" {
		return fmt.Errorf("project path not set")
	}
	if dc.Config.Output == "" {
		return fmt.Errorf("output path not set")
	}

	fmt.Println("Reading base docker-compose file")
	// Get base compose
	basePath := fmt.Sprintf(
		"%s/%s/%s/%s",
		dc.Config.ProjectPath,
		dc.Config.ConfigFolder,
		dc.Config.BuildType,
		"docker-compose.base.yml",
	)
	baseCompose, err := dc.ReadFile(basePath)
	if err != nil {
		return fmt.Errorf(
			"error reading docker-compose.base.yml:\n\tpath:%s\n\terror:%s",
			basePath,
			err,
		)
	}

	fmt.Println("Readding Service level docker-compose files")
	// Get service level compose
	servicesCompose, err := dc.GetServices()
	if err != nil {
		return fmt.Errorf(
			"error reading service level docker-compose.yml:\n\terror:%s",
			err,
		)
	}

	mergeOpts := &MergeOptions{
		MergeServices: true,
		PreferFirst:   false,
		OnConflict:    dc.DefaultOnConflict,
	}

	fmt.Println("Creating base level docker-compose merge")
	combinedStore, err := dc.combineDockerCompose(baseCompose, nil, mergeOpts)
	if err != nil {
		return err
	}

	fmt.Println("Merging service level docker-compose files into main")
	for _, s := range *servicesCompose {
		combinedStore, err = dc.combineDockerCompose(combinedStore, &s, mergeOpts)
		if err != nil {
			return err
		}
	}

	dc.Store = combinedStore

	fmt.Println("Writing file")
	outputFilePath := fmt.Sprintf(
		"%s/%s/docker/docker-compose.yml",
		dc.Config.ProjectPath,
		dc.Config.Output,
	)
	err = dc.writeFile(outputFilePath)
	if err != nil {
		return err
	}

	return nil

}

func (dc *DockerComposeCompiler) GetServices() (*[]types.DockerCompose, error) {
	var services []types.DockerCompose
	for _, s := range dc.Config.Services {
		serviceFilePath := fmt.Sprintf("%s/docker-compose.yml", s)
		fmt.Println("Reading docker-compose:", serviceFilePath)
		serviceCompose, err := dc.ReadFile(serviceFilePath)
		if err != nil {
			fmt.Println("error reading file: ", err)
			continue
		}

		// Set build context
		serviceParts := strings.Split(s, "/")
		serviceName := serviceParts[len(serviceParts)-1]
		fmt.Println("Service", serviceName)
		service, exists := serviceCompose.Services[serviceName]
		fmt.Println("Service exists: ", exists, serviceName)
		if exists {
			fmt.Println("Service exists")
			dc.CheckBuildContext(&service, serviceName)
			serviceCompose.Services[serviceName] = service
		}

		services = append(services, *serviceCompose)
	}

	return &services, nil
}

// Read and Parse docker-compose.yml file and return as a struct
func (dc *DockerComposeCompiler) ReadFile(filePath string) (*types.DockerCompose, error) {

	// Read file into a byte array
	composeFile, err := os.ReadFile(filePath)
	if err != nil {
		return nil, err
	}

	// Parse the file into {DockerCompose struct}
	var dockerCompose types.DockerCompose
	err = yaml.Unmarshal(composeFile, &dockerCompose)
	if err != nil {
		return nil, err
	}

	return &dockerCompose, nil
}

func (dc *DockerComposeCompiler) writeFile(outputFilePath string) error {

	// Marshal the struct into YAML bytes
	yamlBytes, err := yaml.Marshal(dc.Store)
	if err != nil {
		return fmt.Errorf("error marshaling DockerCompose data to YAML: %w", err)
	}

	// Write the YAML bytes to the specified output file
	err = os.WriteFile(outputFilePath, yamlBytes, 0644) // 0644 gives read/write for owner, read-only for others
	if err != nil {
		return fmt.Errorf("error writing docker-compose.yml file: %w", err)
	}

	fmt.Printf("Successfully generated file\n\tPath: %s\n", outputFilePath)
	return nil

}

// DefaultOnConflict is a default implementation for OnConflict that always prefers the second (later) item.
func (dc *DockerComposeCompiler) DefaultOnConflict(itemType, name string, first, second interface{}) bool {
	fmt.Printf("Conflict detected for %s '%s'. Preferring the second item.\n", itemType, name)
	return true // Always return true to use the second (conflicting) item by default
}

func (dc *DockerComposeCompiler) combineDockerCompose(a, b *types.DockerCompose, opts *MergeOptions) (*types.DockerCompose, error) {
	if a == nil && b == nil {
		return nil, fmt.Errorf("no compose files passed")
	}
	if a == nil {
		return b, nil
	}
	if b == nil {
		return a, nil
	}

	result := &types.DockerCompose{
		Services: make(map[string]types.Service),
		Networks: make(map[string]types.Network),
		Volumes:  make(map[string]types.Volume),
		Secrets:  make(map[string]types.Secret),
		Configs:  make(map[string]types.Config),
	}

	// Determine version precedence
	if opts.PreferFirst {
		result.Version = a.Version
		if result.Version == "" {
			result.Version = b.Version
		}
	} else {
		result.Version = b.Version
		if result.Version == "" {
			result.Version = a.Version
		}
	}

	// Merge Services
	maps.Copy(result.Services, a.Services)

	for name, service := range b.Services {
		if existing, exists := result.Services[name]; exists {
			// Handle conflict
			var useSecond bool
			if opts.OnConflict != nil {
				useSecond = opts.OnConflict("service", name, existing, service)
			} else {
				useSecond = !opts.PreferFirst
			}

			if useSecond {
				if opts.MergeServices {
					result.Services[name] = dc.mergeServices(existing, service)
				} else {
					result.Services[name] = service
				}
			}
		} else {
			result.Services[name] = service
		}
	}

	// Merge other sections (Networks, Volumes, Secrets, Configs)
	mergeMap := func(itemType string, aMap, bMap interface{}) {
		switch itemType {
		case "networks":
			aNetworks := aMap.(map[string]types.Network)
			bNetworks := bMap.(map[string]types.Network)
			for name, network := range aNetworks {
				result.Networks[name] = network
			}
			for name, network := range bNetworks {
				if existing, exists := result.Networks[name]; exists && opts.OnConflict != nil {
					if opts.OnConflict("network", name, existing, network) {
						result.Networks[name] = network
					}
				} else if !exists || !opts.PreferFirst {
					result.Networks[name] = network
				}
			}
		case "volumes":
			aVolumes := aMap.(map[string]types.Volume)
			bVolumes := bMap.(map[string]types.Volume)
			for name, volume := range aVolumes {
				result.Volumes[name] = volume
			}
			for name, volume := range bVolumes {
				if existing, exists := result.Volumes[name]; exists && opts.OnConflict != nil {
					if opts.OnConflict("volume", name, existing, volume) {
						result.Volumes[name] = volume
					}
				} else if !exists || !opts.PreferFirst {
					result.Volumes[name] = volume
				}
			}
		case "secrets":
			aSecrets := aMap.(map[string]types.Secret)
			bSecrets := bMap.(map[string]types.Secret)
			for name, secret := range aSecrets {
				result.Secrets[name] = secret
			}
			for name, secret := range bSecrets {
				if existing, exists := result.Secrets[name]; exists && opts.OnConflict != nil {
					if opts.OnConflict("secret", name, existing, secret) {
						result.Secrets[name] = secret
					}
				} else if !exists || !opts.PreferFirst {
					result.Secrets[name] = secret
				}
			}
		case "configs":
			aConfigs := aMap.(map[string]types.Config)
			bConfigs := bMap.(map[string]types.Config)
			for name, config := range aConfigs {
				result.Configs[name] = config
			}
			for name, config := range bConfigs {
				if existing, exists := result.Configs[name]; exists && opts.OnConflict != nil {
					if opts.OnConflict("config", name, existing, config) {
						result.Configs[name] = config
					}
				} else if !exists || !opts.PreferFirst {
					result.Configs[name] = config
				}
			}
		}
	}

	mergeMap("networks", a.Networks, b.Networks)
	mergeMap("volumes", a.Volumes, b.Volumes)
	mergeMap("secrets", a.Secrets, b.Secrets)
	mergeMap("configs", a.Configs, b.Configs)

	return result, nil
}

// mergeServices combines two services, with the second taking precedence for conflicting fields
func (dc *DockerComposeCompiler) mergeServices(a, b types.Service) types.Service {
	result := a // Start with first service

	// Override with non-zero values from second service
	if b.Image != "" {
		result.Image = b.Image
	}
	if b.Build != nil {
		result.Build = b.Build
	}
	if b.ContainerName != "" {
		result.ContainerName = b.ContainerName
	}
	if b.Command != nil {
		result.Command = b.Command
	}
	if b.Entrypoint != nil {
		result.Entrypoint = b.Entrypoint
	}

	// Merge maps
	if len(b.Environment) > 0 {
		if result.Environment == nil {
			result.Environment = make(map[string]string)
		}
		for k, v := range b.Environment {
			result.Environment[k] = v
		}
	}

	if len(b.Labels) > 0 {
		if result.Labels == nil {
			result.Labels = make(map[string]string)
		}
		for k, v := range b.Labels {
			result.Labels[k] = v
		}
	}

	// Merge slices (append)
	if len(b.Ports) > 0 {
		result.Ports = append(result.Ports, b.Ports...)
	}
	if len(b.Volumes) > 0 {
		result.Volumes = append(result.Volumes, b.Volumes...)
	}

	// Handle service dependsOn
	result.DependsOn = b.DependsOn

	// Override other fields
	if b.Restart != "" {
		result.Restart = b.Restart
	}
	if b.User != "" {
		result.User = b.User
	}
	if b.WorkingDir != "" {
		result.WorkingDir = b.WorkingDir
	}

	return result
}

// Check if a service has a build context and set it correctly
func (dc *DockerComposeCompiler) CheckBuildContext(service *types.Service, serviceName string) error {
	if service.Build == nil {
		// Build field is nil, so no build context exists
		return fmt.Errorf("Build field not found")
	}

	// Set Build context to work correctly
	service.Build.Context = fmt.Sprintf("%s/%s/%s", dc.Config.ProjectPath, dc.Config.ServicesFolder, serviceName)

	return nil
}
