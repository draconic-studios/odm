package utils

import (
	"os"
	"path/filepath"
	"strings"

	"gopkg.in/yaml.v3"
)

// Main Envoy configuration structure
type EnvoyConfig struct {
	StaticResources StaticResources `yaml:"static_resources"`
	// Clusters     []Cluster       `yaml:"clusters"` // Uncomment and define Cluster struct if needed
}

// StaticResources contains listeners and clusters
type StaticResources struct {
	Listeners []Listener `yaml:"listeners"`
	Clusters  []Cluster  `yaml:"clusters"` // Adding an empty slice for clusters as per YAML
}

// Listener configuration
type Listener struct {
	Name         string        `yaml:"name"`
	Address      Address       `yaml:"address"`
	FilterChains []FilterChain `yaml:"filter_chains"`
}

// Address configuration for a socket
type Address struct {
	SocketAddress SocketAddress `yaml:"socket_address"`
}

// SocketAddress details
type SocketAddress struct {
	Protocol  string `yaml:"protocol"`
	Address   string `yaml:"address"`
	PortValue int    `yaml:"port_value"`
}

// FilterChain holds network filters
type FilterChain struct {
	Filters []Filter `yaml:"filters"`
}

// Filter configuration (e.g., HTTP Connection Manager)
type Filter struct {
	Name        string      `yaml:"name"`
	TypedConfig TypedConfig `yaml:"typed_config"`
}

// TypedConfig holds the actual configuration for a filter.
// We'll embed specific types based on the "@type" field.
// This requires careful mapping.
type TypedConfig struct {
	Type string `yaml:"@type"` // The "@type" field
	// For HttpConnectionManager
	StatPrefix  string       `yaml:"stat_prefix,omitempty"`
	CodecType   string       `yaml:"codec_type,omitempty"`
	RouteConfig RouteConfig  `yaml:"route_config,omitempty"`
	HTTPFilters []HTTPFilter `yaml:"http_filters,omitempty"`
	AccessLog   []AccessLog  `yaml:"access_log,omitempty"`

	// Add other specific fields if this struct were to represent other TypedConfig types
	// For instance, if you had another filter with a different @type, its specific
	// fields would go here, often requiring omitempty.
}

// RouteConfig for HTTP Connection Manager
type RouteConfig struct {
	Name         string        `yaml:"name"`
	VirtualHosts []VirtualHost `yaml:"virtual_hosts"`
}

// VirtualHost for routing
type VirtualHost struct {
	Name    string   `yaml:"name"`
	Domains []string `yaml:"domains"`
	Routes  []Route  `yaml:"routes"` // This is an empty slice in your example, but defined for completeness
}

// Route configuration (currently empty in your example)
type Route struct {
	// Add route details here if your routes were defined
}

// HTTPFilter configuration
type HTTPFilter struct {
	Name        string                `yaml:"name"`
	TypedConfig HTTPFilterTypedConfig `yaml:"typed_config"`
}

// HTTPFilterTypedConfig holds configurations for various HTTP filters
// This struct will hold specific configurations based on the filter's @type
type HTTPFilterTypedConfig struct {
	Type string `yaml:"@type"` // The "@type" field
	// For ExtAuthz filter
	HTTPService      *HTTPService `yaml:"http_service,omitempty"`
	PathPrefix       string       `yaml:"path_prefix,omitempty"`
	FailureModeAllow bool         `yaml:"failure_mode_allow,omitempty"`
	ClearRouteCache  bool         `yaml:"clear_route_cache,omitempty"`

	// For Cors filter
	// No specific fields mentioned in your YAML for Cors beyond @type,
	// so it can be omitted or just the Type field exists.

	// For Compressor filter
	ResponseDirectionConfig *ResponseDirectionConfig `yaml:"response_direction_config,omitempty"`
	CompressorLibrary       *CompressorLibrary       `yaml:"compressor_library,omitempty"`
}

// HTTPService for ExtAuthz
type HTTPService struct {
	ServerURI ServerURI `yaml:"server_uri"`
	// Other fields like path_prefix are directly in HTTPFilterTypedConfig
	AuthorizationRequest  *AuthorizationRequest  `yaml:"authorization_request,omitempty"`
	AuthorizationResponse *AuthorizationResponse `yaml:"authorization_response,omitempty"`
}

// ServerURI for ExtAuthz HTTP service
type ServerURI struct {
	URI     string `yaml:"uri"`
	Cluster string `yaml:"cluster"`
	Timeout string `yaml:"timeout"` // Duration as string, Go's time.Duration can parse this
}

// AuthorizationRequest for ExtAuthz
type AuthorizationRequest struct {
	AllowedHeaders AllowedHeaders `yaml:"allowed_headers"`
}

// AuthorizationResponse for ExtAuthz
type AuthorizationResponse struct {
	AllowedUpstreamHeaders AllowedHeaders `yaml:"allowed_upstream_headers"`
}

// AllowedHeaders for ExtAuthz authorization
type AllowedHeaders struct {
	Patterns []HeaderPattern `yaml:"patterns"`
}

// HeaderPattern for ExtAuthz allowed headers
type HeaderPattern struct {
	Exact string `yaml:"exact"`
}

// ResponseDirectionConfig for Compressor filter
type ResponseDirectionConfig struct {
	CommonConfig CommonCompressionConfig `yaml:"common_config"`
}

// CommonCompressionConfig for Compressor filter
type CommonCompressionConfig struct {
	MinContentLength int      `yaml:"min_content_length"`
	ContentType      []string `yaml:"content_type"`
}

// CompressorLibrary for Compressor filter
type CompressorLibrary struct {
	Name        string                  `yaml:"name"`
	TypedConfig CompressorLibraryConfig `yaml:"typed_config"`
}

// CompressorLibraryConfig for Gzip compressor
type CompressorLibraryConfig struct {
	Type string `yaml:"@type"` // Should be type.googleapis.com/envoy.extensions.compression.gzip.compressor.v3.Gzip
}

// AccessLog configuration
type AccessLog struct {
	Name        string          `yaml:"name"`
	TypedConfig AccessLogConfig `yaml:"typed_config"`
}

// AccessLogConfig for StdoutAccessLog
type AccessLogConfig struct {
	Type string `yaml:"@type"` // Should be type.googleapis.com/envoy.extensions.access_loggers.stream.v3.StdoutAccessLog
}

// Cluster configuration (empty in your example, but good to have a placeholder)
type Cluster struct {
	// Define cluster fields here if your YAML had cluster configurations
	Name string `yaml:"name"`
	// ... other cluster properties
}

type ServiceDeclaration struct {
	Clusters []Cluster `yaml:"clusters"`
	Routes   []Route   `yaml:"routes"`
}

func ReadEnvoyBaseTemplate(filePath string) (*EnvoyConfig, error) {
	// Read the YAML file
	data, err := os.ReadFile(filePath)
	if err != nil {
		return nil, err
	}

	var config EnvoyConfig
	err = yaml.Unmarshal(data, &config)
	if err != nil {
		return nil, err
	}

	return &config, nil

}

func ReadEnvoyServiceConfig(filePath string) (*ServiceDeclaration, error) {
	service := &ServiceDeclaration{Clusters: []Cluster{}, Routes: []Route{}}
	// Read the YAML file
	clustersPath := filePath + "/envoy/clusters"
	routesPath := filePath + "/envoy/routes"

	// Get all cluster files in cluster folder
	clusters, err := ReadFolderContents(clustersPath)
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
		var config ServiceDeclaration
		err = yaml.Unmarshal(cluster, &config)
		if err != nil {
			return nil, err
		}
		service.Clusters = append(service.Clusters, config.Clusters...)
	}

	routes, err := ReadFolderContents(routesPath)
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
		var config ServiceDeclaration
		err = yaml.Unmarshal(route, &config)
		if err != nil {
			return nil, err
		}
		service.Routes = append(service.Routes, config.Routes...)
	}

	return service, nil
}
