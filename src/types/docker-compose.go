package types

import (
	"time"
)

// DockerCompose represents the root structure of a docker-compose.yml file
type DockerCompose struct {
	Version  string             `yaml:"version,omitempty"`
	Services map[string]Service `yaml:"services,omitempty"`
	Networks map[string]Network `yaml:"networks,omitempty"`
	Volumes  map[string]Volume  `yaml:"volumes,omitempty"`
	Secrets  map[string]Secret  `yaml:"secrets,omitempty"`
	Configs  map[string]Config  `yaml:"configs,omitempty"`
}

// Service represents a service definition in docker-compose
type Service struct {
	Image           string                 `yaml:"image,omitempty"`
	Environment     map[string]string      `yaml:"environment,omitempty"` // Changed to []string
	Build           *BuildConfig           `yaml:"build,omitempty"`
	ContainerName   string                 `yaml:"container_name,omitempty"`
	Command         interface{}            `yaml:"command,omitempty"`    // string or []string
	Entrypoint      interface{}            `yaml:"entrypoint,omitempty"` // string or []string
	EnvFile         interface{}            `yaml:"env_file,omitempty"`   // string or []string
	Ports           []string               `yaml:"ports,omitempty"`
	Expose          []string               `yaml:"expose,omitempty"`
	Volumes         []string               `yaml:"volumes,omitempty"`
	VolumesFrom     []string               `yaml:"volumes_from,omitempty"`
	Networks        interface{}            `yaml:"networks,omitempty"`   // []string or map[string]NetworkConfig
	DependsOn       interface{}            `yaml:"depends_on,omitempty"` // []string or map[string]DependsOnConfig
	Links           []string               `yaml:"links,omitempty"`
	ExternalLinks   []string               `yaml:"external_links,omitempty"`
	Restart         string                 `yaml:"restart,omitempty"`
	User            string                 `yaml:"user,omitempty"`
	WorkingDir      string                 `yaml:"working_dir,omitempty"`
	Hostname        string                 `yaml:"hostname,omitempty"`
	DomainName      string                 `yaml:"domainname,omitempty"`
	MacAddress      string                 `yaml:"mac_address,omitempty"`
	Privileged      bool                   `yaml:"privileged,omitempty"`
	ReadOnly        bool                   `yaml:"read_only,omitempty"`
	StdinOpen       bool                   `yaml:"stdin_open,omitempty"`
	Tty             bool                   `yaml:"tty,omitempty"`
	CPU             float64                `yaml:"cpu_shares,omitempty"`
	CPUs            string                 `yaml:"cpus,omitempty"`
	CPUSet          string                 `yaml:"cpuset,omitempty"`
	Memory          string                 `yaml:"mem_limit,omitempty"`
	MemSwap         string                 `yaml:"memswap_limit,omitempty"`
	ShmSize         string                 `yaml:"shm_size,omitempty"`
	PidMode         string                 `yaml:"pid,omitempty"`
	IPC             string                 `yaml:"ipc,omitempty"`
	SecurityOpt     []string               `yaml:"security_opt,omitempty"`
	StopSignal      string                 `yaml:"stop_signal,omitempty"`
	StopGracePeriod *time.Duration         `yaml:"stop_grace_period,omitempty"`
	Ulimits         map[string]interface{} `yaml:"ulimits,omitempty"`
	Devices         []string               `yaml:"devices,omitempty"`
	Labels          map[string]string      `yaml:"labels,omitempty"`
	LogDriver       string                 `yaml:"log_driver,omitempty"`
	LogOpt          map[string]string      `yaml:"log_opt,omitempty"`
	ExtraHosts      []string               `yaml:"extra_hosts,omitempty"`
	DNS             interface{}            `yaml:"dns,omitempty"` // string or []string
	DNSSearch       []string               `yaml:"dns_search,omitempty"`
	DNSOpt          []string               `yaml:"dns_opt,omitempty"`
	TmpFS           interface{}            `yaml:"tmpfs,omitempty"` // string or []string
	Secrets         []string               `yaml:"secrets,omitempty"`
	Configs         []string               `yaml:"configs,omitempty"`
	Deploy          *DeployConfig          `yaml:"deploy,omitempty"`
	HealthCheck     *HealthCheckConfig     `yaml:"healthcheck,omitempty"`
}

// BuildConfig represents build configuration
type BuildConfig struct {
	Context    string            `yaml:"context,omitempty"`
	Dockerfile string            `yaml:"dockerfile,omitempty"`
	Args       map[string]string `yaml:"args,omitempty"`
	Target     string            `yaml:"target,omitempty"`
	Labels     map[string]string `yaml:"labels,omitempty"`
	CacheFrom  []string          `yaml:"cache_from,omitempty"`
	Network    string            `yaml:"network,omitempty"`
	ShmSize    string            `yaml:"shm_size,omitempty"`
	Secrets    []string          `yaml:"secrets,omitempty"`
}

// NetworkConfig represents network configuration for a service
type NetworkConfig struct {
	Aliases     []string `yaml:"aliases,omitempty"`
	IPv4Address string   `yaml:"ipv4_address,omitempty"`
	IPv6Address string   `yaml:"ipv6_address,omitempty"`
}

// DependsOnConfig represents depends_on configuration
type DependsOnConfig struct {
	Condition string `yaml:"condition,omitempty"`
}

// DeployConfig represents deployment configuration
type DeployConfig struct {
	Mode          string               `yaml:"mode,omitempty"`
	Replicas      int                  `yaml:"replicas,omitempty"`
	Labels        map[string]string    `yaml:"labels,omitempty"`
	UpdateConfig  *UpdateConfig        `yaml:"update_config,omitempty"`
	Resources     *ResourcesConfig     `yaml:"resources,omitempty"`
	RestartPolicy *RestartPolicyConfig `yaml:"restart_policy,omitempty"`
	Placement     *PlacementConfig     `yaml:"placement,omitempty"`
	EndpointMode  string               `yaml:"endpoint_mode,omitempty"`
}

// UpdateConfig represents update configuration
type UpdateConfig struct {
	Parallelism     int            `yaml:"parallelism,omitempty"`
	Delay           *time.Duration `yaml:"delay,omitempty"`
	FailureAction   string         `yaml:"failure_action,omitempty"`
	Monitor         *time.Duration `yaml:"monitor,omitempty"`
	MaxFailureRatio float64        `yaml:"max_failure_ratio,omitempty"`
	Order           string         `yaml:"order,omitempty"`
}

// ResourcesConfig represents resource constraints
type ResourcesConfig struct {
	Limits       *ResourceLimit `yaml:"limits,omitempty"`
	Reservations *ResourceLimit `yaml:"reservations,omitempty"`
}

// ResourceLimit represents resource limits
type ResourceLimit struct {
	CPUs    string   `yaml:"cpus,omitempty"`
	Memory  string   `yaml:"memory,omitempty"`
	Devices []string `yaml:"devices,omitempty"`
}

// RestartPolicyConfig represents restart policy
type RestartPolicyConfig struct {
	Condition   string         `yaml:"condition,omitempty"`
	Delay       *time.Duration `yaml:"delay,omitempty"`
	MaxAttempts int            `yaml:"max_attempts,omitempty"`
	Window      *time.Duration `yaml:"window,omitempty"`
}

// PlacementConfig represents placement constraints
type PlacementConfig struct {
	Constraints []string            `yaml:"constraints,omitempty"`
	Preferences []map[string]string `yaml:"preferences,omitempty"`
	MaxReplicas int                 `yaml:"max_replicas_per_node,omitempty"`
}

// HealthCheckConfig represents health check configuration
type HealthCheckConfig struct {
	Test        interface{}    `yaml:"test,omitempty"` // string or []string
	Interval    *time.Duration `yaml:"interval,omitempty"`
	Timeout     *time.Duration `yaml:"timeout,omitempty"`
	Retries     int            `yaml:"retries,omitempty"`
	StartPeriod *time.Duration `yaml:"start_period,omitempty"`
	Disable     bool           `yaml:"disable,omitempty"`
}

// Network represents a network definition
type Network struct {
	Driver     string            `yaml:"driver,omitempty"`
	DriverOpts map[string]string `yaml:"driver_opts,omitempty"`
	IPAM       *IPAMConfig       `yaml:"ipam,omitempty"`
	External   interface{}       `yaml:"external,omitempty"` // bool or map[string]string
	Labels     map[string]string `yaml:"labels,omitempty"`
	EnableIPv6 bool              `yaml:"enable_ipv6,omitempty"`
	Attachable bool              `yaml:"attachable,omitempty"`
	Internal   bool              `yaml:"internal,omitempty"`
	Name       string            `yaml:"name,omitempty"`
}

// IPAMConfig represents IPAM configuration
type IPAMConfig struct {
	Driver  string                   `yaml:"driver,omitempty"`
	Config  []map[string]interface{} `yaml:"config,omitempty"`
	Options map[string]string        `yaml:"options,omitempty"`
}

// Volume represents a volume definition
type Volume struct {
	Driver     string            `yaml:"driver,omitempty"`
	DriverOpts map[string]string `yaml:"driver_opts,omitempty"`
	External   interface{}       `yaml:"external,omitempty"` // bool or map[string]string
	Labels     map[string]string `yaml:"labels,omitempty"`
	Name       string            `yaml:"name,omitempty"`
}

// Secret represents a secret definition
type Secret struct {
	File     string            `yaml:"file,omitempty"`
	External interface{}       `yaml:"external,omitempty"` // bool or map[string]string
	Labels   map[string]string `yaml:"labels,omitempty"`
	Name     string            `yaml:"name,omitempty"`
}

// Config represents a config definition
type Config struct {
	File     string            `yaml:"file,omitempty"`
	External interface{}       `yaml:"external,omitempty"` // bool or map[string]string
	Labels   map[string]string `yaml:"labels,omitempty"`
	Name     string            `yaml:"name,omitempty"`
}
