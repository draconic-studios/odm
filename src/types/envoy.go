package types

import "time"

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

// VirtualHost represents a virtual host configuration
type VirtualHost struct {
	Name    string   `yaml:"name" json:"name"`
	Domains []string `yaml:"domains" json:"domains"`
	Routes  []Route  `yaml:"routes" json:"routes"`
	CORS    *CORS    `yaml:"cors,omitempty" json:"cors,omitempty"`
}

// RouteConfiguration represents the top-level route configuration
type RouteConfiguration struct {
	Name         string           `yaml:"name" json:"name"`
	VirtualHosts []VirtualHost    `yaml:"virtual_hosts" json:"virtual_hosts"`
	Headers      []HeaderValue    `yaml:"response_headers_to_add,omitempty" json:"response_headers_to_add,omitempty"`
	RequestID    *RequestIDConfig `yaml:"request_id_extension,omitempty" json:"request_id_extension,omitempty"`
}

// Route represents an individual route
type Route struct {
	Name                 string                 `yaml:"name,omitempty" json:"name,omitempty"`
	Match                RouteMatch             `yaml:"match" json:"match"`
	Route                *RouteAction           `yaml:"route,omitempty" json:"route,omitempty"`
	Redirect             *RedirectAction        `yaml:"redirect,omitempty" json:"redirect,omitempty"`
	DirectResponse       *DirectResponseAction  `yaml:"direct_response,omitempty" json:"direct_response,omitempty"`
	Metadata             *Metadata              `yaml:"metadata,omitempty" json:"metadata,omitempty"`
	Decorator            *Decorator             `yaml:"decorator,omitempty" json:"decorator,omitempty"`
	TypedPerFilterConfig map[string]interface{} `yaml:"typed_per_filter_config,omitempty" json:"typed_per_filter_config,omitempty"`
}

// RouteMatch defines route matching criteria
type RouteMatch struct {
	Prefix          string                    `yaml:"prefix,omitempty" json:"prefix,omitempty"`
	Path            string                    `yaml:"path,omitempty" json:"path,omitempty"`
	Regex           string                    `yaml:"safe_regex,omitempty" json:"safe_regex,omitempty"`
	Headers         []HeaderMatcher           `yaml:"headers,omitempty" json:"headers,omitempty"`
	QueryParams     []QueryParamMatcher       `yaml:"query_parameters,omitempty" json:"query_parameters,omitempty"`
	CaseSensitive   *bool                     `yaml:"case_sensitive,omitempty" json:"case_sensitive,omitempty"`
	RuntimeFraction *RuntimeFractionalPercent `yaml:"runtime_fraction,omitempty" json:"runtime_fraction,omitempty"`
}

// RouteAction defines the routing action
type RouteAction struct {
	Cluster          string              `yaml:"cluster,omitempty" json:"cluster,omitempty"`
	ClusterHeader    string              `yaml:"cluster_header,omitempty" json:"cluster_header,omitempty"`
	WeightedClusters *WeightedCluster    `yaml:"weighted_clusters,omitempty" json:"weighted_clusters,omitempty"`
	HostRewrite      string              `yaml:"host_rewrite_literal,omitempty" json:"host_rewrite_literal,omitempty"`
	PrefixRewrite    string              `yaml:"prefix_rewrite,omitempty" json:"prefix_rewrite,omitempty"`
	RegexRewrite     *RegexRewrite       `yaml:"regex_rewrite,omitempty" json:"regex_rewrite,omitempty"`
	Timeout          *time.Duration      `yaml:"timeout,omitempty" json:"timeout,omitempty"`
	RetryPolicy      *RetryPolicy        `yaml:"retry_policy,omitempty" json:"retry_policy,omitempty"`
	RateLimits       []RateLimit         `yaml:"rate_limits,omitempty" json:"rate_limits,omitempty"`
	RequestHeaders   []HeaderValueOption `yaml:"request_headers_to_add,omitempty" json:"request_headers_to_add,omitempty"`
	ResponseHeaders  []HeaderValueOption `yaml:"response_headers_to_add,omitempty" json:"response_headers_to_add,omitempty"`
	HashPolicy       []HashPolicy        `yaml:"hash_policy,omitempty" json:"hash_policy,omitempty"`
}

// RedirectAction defines redirect behavior
type RedirectAction struct {
	HostRedirect  string `yaml:"host_redirect,omitempty" json:"host_redirect,omitempty"`
	PathRedirect  string `yaml:"path_redirect,omitempty" json:"path_redirect,omitempty"`
	PrefixRewrite string `yaml:"prefix_rewrite,omitempty" json:"prefix_rewrite,omitempty"`
	ResponseCode  int    `yaml:"response_code,omitempty" json:"response_code,omitempty"`
	HttpsRedirect bool   `yaml:"https_redirect,omitempty" json:"https_redirect,omitempty"`
	StripQuery    bool   `yaml:"strip_query,omitempty" json:"strip_query,omitempty"`
}

// DirectResponseAction defines direct response behavior
type DirectResponseAction struct {
	Status int    `yaml:"status" json:"status"`
	Body   string `yaml:"body,omitempty" json:"body,omitempty"`
}

// WeightedCluster defines weighted cluster routing
type WeightedCluster struct {
	Clusters    []ClusterWeight `yaml:"clusters" json:"clusters"`
	TotalWeight int             `yaml:"total_weight,omitempty" json:"total_weight,omitempty"`
}

// ClusterWeight defines a weighted cluster
type ClusterWeight struct {
	Name   string `yaml:"name" json:"name"`
	Weight int    `yaml:"weight" json:"weight"`
}

// HeaderMatcher defines header matching criteria
type HeaderMatcher struct {
	Name         string `yaml:"name" json:"name"`
	ExactMatch   string `yaml:"exact_match,omitempty" json:"exact_match,omitempty"`
	RegexMatch   string `yaml:"safe_regex_match,omitempty" json:"safe_regex_match,omitempty"`
	PrefixMatch  string `yaml:"prefix_match,omitempty" json:"prefix_match,omitempty"`
	SuffixMatch  string `yaml:"suffix_match,omitempty" json:"suffix_match,omitempty"`
	PresentMatch bool   `yaml:"present_match,omitempty" json:"present_match,omitempty"`
	InvertMatch  bool   `yaml:"invert_match,omitempty" json:"invert_match,omitempty"`
}

// QueryParamMatcher defines query parameter matching
type QueryParamMatcher struct {
	Name         string `yaml:"name" json:"name"`
	Value        string `yaml:"string_match,omitempty" json:"string_match,omitempty"`
	PresentMatch bool   `yaml:"present_match,omitempty" json:"present_match,omitempty"`
}

// RetryPolicy defines retry behavior
type RetryPolicy struct {
	RetryOn          string          `yaml:"retry_on,omitempty" json:"retry_on,omitempty"`
	NumRetries       int             `yaml:"num_retries,omitempty" json:"num_retries,omitempty"`
	PerTryTimeout    *time.Duration  `yaml:"per_try_timeout,omitempty" json:"per_try_timeout,omitempty"`
	RetryBackOff     *BackOffPolicy  `yaml:"retry_back_off,omitempty" json:"retry_back_off,omitempty"`
	RetriableHeaders []HeaderMatcher `yaml:"retriable_headers,omitempty" json:"retriable_headers,omitempty"`
}

// BackOffPolicy defines backoff behavior
type BackOffPolicy struct {
	BaseInterval time.Duration `yaml:"base_interval" json:"base_interval"`
	MaxInterval  time.Duration `yaml:"max_interval,omitempty" json:"max_interval,omitempty"`
}

// RateLimit defines rate limiting
type RateLimit struct {
	Actions []RateLimitAction `yaml:"actions" json:"actions"`
}

// RateLimitAction defines rate limit actions
type RateLimitAction struct {
	RequestHeaders   *RequestHeaders   `yaml:"request_headers,omitempty" json:"request_headers,omitempty"`
	RemoteAddress    *RemoteAddress    `yaml:"remote_address,omitempty" json:"remote_address,omitempty"`
	GenericKey       *GenericKey       `yaml:"generic_key,omitempty" json:"generic_key,omitempty"`
	HeaderValueMatch *HeaderValueMatch `yaml:"header_value_match,omitempty" json:"header_value_match,omitempty"`
}

// HeaderValueOption represents a header value with options
type HeaderValueOption struct {
	Header HeaderValue `yaml:"header" json:"header"`
	Append bool        `yaml:"append,omitempty" json:"append,omitempty"`
}

// CORS defines CORS policy
type CORS struct {
	AllowOrigin      []string `yaml:"allow_origin,omitempty" json:"allow_origin,omitempty"`
	AllowMethods     string   `yaml:"allow_methods,omitempty" json:"allow_methods,omitempty"`
	AllowHeaders     string   `yaml:"allow_headers,omitempty" json:"allow_headers,omitempty"`
	ExposeHeaders    string   `yaml:"expose_headers,omitempty" json:"expose_headers,omitempty"`
	MaxAge           string   `yaml:"max_age,omitempty" json:"max_age,omitempty"`
	AllowCredentials bool     `yaml:"allow_credentials,omitempty" json:"allow_credentials,omitempty"`
}

// Additional supporting types
type RuntimeFractionalPercent struct {
	DefaultValue int    `yaml:"default_value" json:"default_value"`
	RuntimeKey   string `yaml:"runtime_key,omitempty" json:"runtime_key,omitempty"`
}

type RegexRewrite struct {
	Pattern      string `yaml:"pattern" json:"pattern"`
	Substitution string `yaml:"substitution" json:"substitution"`
}

type HashPolicy struct {
	Header               *Header               `yaml:"header,omitempty" json:"header,omitempty"`
	Cookie               *Cookie               `yaml:"cookie,omitempty" json:"cookie,omitempty"`
	ConnectionProperties *ConnectionProperties `yaml:"connection_properties,omitempty" json:"connection_properties,omitempty"`
	QueryParameter       *QueryParameter       `yaml:"query_parameter,omitempty" json:"query_parameter,omitempty"`
}

type Header struct {
	HeaderName string `yaml:"header_name" json:"header_name"`
}

type Cookie struct {
	Name string `yaml:"name" json:"name"`
	TTL  string `yaml:"ttl,omitempty" json:"ttl,omitempty"`
	Path string `yaml:"path,omitempty" json:"path,omitempty"`
}

type ConnectionProperties struct {
	SourceIP bool `yaml:"source_ip,omitempty" json:"source_ip,omitempty"`
}

type QueryParameter struct {
	Name string `yaml:"name" json:"name"`
}

type Metadata struct {
	FilterMetadata map[string]interface{} `yaml:"filter_metadata,omitempty" json:"filter_metadata,omitempty"`
}

type Decorator struct {
	Operation string `yaml:"operation" json:"operation"`
}

type RequestIDConfig struct {
	TypedConfig map[string]interface{} `yaml:"typed_config" json:"typed_config"`
}

// Rate limit action types
type RequestHeaders struct {
	HeaderName    string `yaml:"header_name" json:"header_name"`
	DescriptorKey string `yaml:"descriptor_key" json:"descriptor_key"`
}

type RemoteAddress struct{}

type GenericKey struct {
	DescriptorValue string `yaml:"descriptor_value" json:"descriptor_value"`
}

type HeaderValueMatch struct {
	DescriptorValue string          `yaml:"descriptor_value" json:"descriptor_value"`
	ExpectMatch     bool            `yaml:"expect_match,omitempty" json:"expect_match,omitempty"`
	Headers         []HeaderMatcher `yaml:"headers" json:"headers"`
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

type ServiceDeclaration struct {
	Clusters []Cluster `yaml:"clusters"`
	Routes   []Route   `yaml:"routes"`
}

// Cluster represents an Envoy cluster configuration
type Cluster struct {
	Name                                  string                       `yaml:"name" json:"name"`
	Type                                  string                       `yaml:"type" json:"type"` // STATIC, STRICT_DNS, LOGICAL_DNS, EDS, etc.
	ConnectTimeout                        *time.Duration               `yaml:"connect_timeout" json:"connect_timeout"`
	PerConnectionBufferLimitBytes         *int                         `yaml:"per_connection_buffer_limit_bytes,omitempty" json:"per_connection_buffer_limit_bytes,omitempty"`
	LbPolicy                              string                       `yaml:"lb_policy,omitempty" json:"lb_policy,omitempty"` // ROUND_ROBIN, LEAST_REQUEST, RING_HASH, etc.
	LoadAssignment                        *ClusterLoadAssignment       `yaml:"load_assignment,omitempty" json:"load_assignment,omitempty"`
	HealthChecks                          []HealthCheck                `yaml:"health_checks,omitempty" json:"health_checks,omitempty"`
	MaxRequestsPerConnection              *int                         `yaml:"max_requests_per_connection,omitempty" json:"max_requests_per_connection,omitempty"`
	Http2ProtocolOptions                  *Http2ProtocolOptions        `yaml:"http2_protocol_options,omitempty" json:"http2_protocol_options,omitempty"`
	HttpProtocolOptions                   *HttpProtocolOptions         `yaml:"http_protocol_options,omitempty" json:"http_protocol_options,omitempty"`
	DnsLookupFamily                       string                       `yaml:"dns_lookup_family,omitempty" json:"dns_lookup_family,omitempty"`
	DnsResolvers                          []Address                    `yaml:"dns_resolvers,omitempty" json:"dns_resolvers,omitempty"`
	OutlierDetection                      *OutlierDetection            `yaml:"outlier_detection,omitempty" json:"outlier_detection,omitempty"`
	CleanupInterval                       *time.Duration               `yaml:"cleanup_interval,omitempty" json:"cleanup_interval,omitempty"`
	UpstreamConnectionOptions             *UpstreamConnectionOptions   `yaml:"upstream_connection_options,omitempty" json:"upstream_connection_options,omitempty"`
	CommonLbConfig                        *CommonLbConfig              `yaml:"common_lb_config,omitempty" json:"common_lb_config,omitempty"`
	TransportSocket                       *TransportSocket             `yaml:"transport_socket,omitempty" json:"transport_socket,omitempty"`
	Metadata                              *Metadata                    `yaml:"metadata,omitempty" json:"metadata,omitempty"`
	ProtocolSelection                     string                       `yaml:"protocol_selection,omitempty" json:"protocol_selection,omitempty"`
	UpstreamHttpProtocolOptions           *UpstreamHttpProtocolOptions `yaml:"upstream_http_protocol_options,omitempty" json:"upstream_http_protocol_options,omitempty"`
	CircuitBreakers                       *CircuitBreakers             `yaml:"circuit_breakers,omitempty" json:"circuit_breakers,omitempty"`
	TypedExtensionProtocolOptions         map[string]interface{}       `yaml:"typed_extension_protocol_options,omitempty" json:"typed_extension_protocol_options,omitempty"`
	DnsRefreshRate                        *time.Duration               `yaml:"dns_refresh_rate,omitempty" json:"dns_refresh_rate,omitempty"`
	DnsFailureRefreshRate                 *DnsFailureRefreshRate       `yaml:"dns_failure_refresh_rate,omitempty" json:"dns_failure_refresh_rate,omitempty"`
	RespectDnsTtl                         bool                         `yaml:"respect_dns_ttl,omitempty" json:"respect_dns_ttl,omitempty"`
	LbSubsetConfig                        *LbSubsetConfig              `yaml:"lb_subset_config,omitempty" json:"lb_subset_config,omitempty"`
	RingHashLbConfig                      *RingHashLbConfig            `yaml:"ring_hash_lb_config,omitempty" json:"ring_hash_lb_config,omitempty"`
	MaglevLbConfig                        *MaglevLbConfig              `yaml:"maglev_lb_config,omitempty" json:"maglev_lb_config,omitempty"`
	OriginalDstLbConfig                   *OriginalDstLbConfig         `yaml:"original_dst_lb_config,omitempty" json:"original_dst_lb_config,omitempty"`
	LeastRequestLbConfig                  *LeastRequestLbConfig        `yaml:"least_request_lb_config,omitempty" json:"least_request_lb_config,omitempty"`
	CommonHttpProtocolOptions             *CommonHttpProtocolOptions   `yaml:"common_http_protocol_options,omitempty" json:"common_http_protocol_options,omitempty"`
	AltStatName                           string                       `yaml:"alt_stat_name,omitempty" json:"alt_stat_name,omitempty"`
	PreconnectPolicy                      *PreconnectPolicy            `yaml:"preconnect_policy,omitempty" json:"preconnect_policy,omitempty"`
	ConnectionPoolPerDownstreamConnection bool                         `yaml:"connection_pool_per_downstream_connection,omitempty" json:"connection_pool_per_downstream_connection,omitempty"`
}

// ClusterLoadAssignment defines load assignment for the cluster
type ClusterLoadAssignment struct {
	ClusterName string     `yaml:"cluster_name" json:"cluster_name"`
	Endpoints   []Locality `yaml:"endpoints" json:"endpoints"`
	Policy      *Policy    `yaml:"policy,omitempty" json:"policy,omitempty"`
}

// Locality represents a locality-aware endpoint group
type Locality struct {
	Locality            *LocalityInfo `yaml:"locality,omitempty" json:"locality,omitempty"`
	LbEndpoints         []LbEndpoint  `yaml:"lb_endpoints" json:"lb_endpoints"`
	LoadBalancingWeight *int          `yaml:"load_balancing_weight,omitempty" json:"load_balancing_weight,omitempty"`
	Priority            int           `yaml:"priority,omitempty" json:"priority,omitempty"`
	Proximity           *int          `yaml:"proximity,omitempty" json:"proximity,omitempty"`
}

// LocalityInfo represents locality information
type LocalityInfo struct {
	Region  string `yaml:"region,omitempty" json:"region,omitempty"`
	Zone    string `yaml:"zone,omitempty" json:"zone,omitempty"`
	SubZone string `yaml:"sub_zone,omitempty" json:"sub_zone,omitempty"`
}

// LbEndpoint represents a load balanced endpoint
type LbEndpoint struct {
	Endpoint            *Endpoint `yaml:"endpoint,omitempty" json:"endpoint,omitempty"`
	HealthStatus        string    `yaml:"health_status,omitempty" json:"health_status,omitempty"`
	Metadata            *Metadata `yaml:"metadata,omitempty" json:"metadata,omitempty"`
	LoadBalancingWeight *int      `yaml:"load_balancing_weight,omitempty" json:"load_balancing_weight,omitempty"`
}

// Endpoint represents an endpoint
type Endpoint struct {
	Address           *Address                `yaml:"address" json:"address"`
	HealthCheckConfig *EnvoyHealthCheckConfig `yaml:"health_check_config,omitempty" json:"health_check_config,omitempty"`
}

// Address represents a network address
type Address struct {
	SocketAddress *SocketAddress `yaml:"socket_address,omitempty" json:"socket_address,omitempty"`
	Pipe          *Pipe          `yaml:"pipe,omitempty" json:"pipe,omitempty"`
}

// SocketAddress represents a socket address
type SocketAddress struct {
	Protocol     string `yaml:"protocol,omitempty" json:"protocol,omitempty"` // TCP, UDP
	Address      string `yaml:"address" json:"address"`
	PortValue    int    `yaml:"port_value,omitempty" json:"port_value,omitempty"`
	NamedPort    string `yaml:"named_port,omitempty" json:"named_port,omitempty"`
	ResolverName string `yaml:"resolver_name,omitempty" json:"resolver_name,omitempty"`
	Ipv4Compat   bool   `yaml:"ipv4_compat,omitempty" json:"ipv4_compat,omitempty"`
}

// Pipe represents a pipe address
type Pipe struct {
	Path string `yaml:"path" json:"path"`
	Mode int    `yaml:"mode,omitempty" json:"mode,omitempty"`
}

// HealthCheck defines health check configuration
type HealthCheck struct {
	Timeout                      *time.Duration     `yaml:"timeout" json:"timeout"`
	Interval                     *time.Duration     `yaml:"interval" json:"interval"`
	IntervalJitter               *time.Duration     `yaml:"interval_jitter,omitempty" json:"interval_jitter,omitempty"`
	IntervalJitterPercent        int                `yaml:"interval_jitter_percent,omitempty" json:"interval_jitter_percent,omitempty"`
	UnhealthyThreshold           int                `yaml:"unhealthy_threshold,omitempty" json:"unhealthy_threshold,omitempty"`
	HealthyThreshold             int                `yaml:"healthy_threshold,omitempty" json:"healthy_threshold,omitempty"`
	AltPort                      *int               `yaml:"alt_port,omitempty" json:"alt_port,omitempty"`
	ReuseConnection              bool               `yaml:"reuse_connection,omitempty" json:"reuse_connection,omitempty"`
	HttpHealthCheck              *HttpHealthCheck   `yaml:"http_health_check,omitempty" json:"http_health_check,omitempty"`
	TcpHealthCheck               *TcpHealthCheck    `yaml:"tcp_health_check,omitempty" json:"tcp_health_check,omitempty"`
	GrpcHealthCheck              *GrpcHealthCheck   `yaml:"grpc_health_check,omitempty" json:"grpc_health_check,omitempty"`
	CustomHealthCheck            *CustomHealthCheck `yaml:"custom_health_check,omitempty" json:"custom_health_check,omitempty"`
	NoTrafficInterval            *time.Duration     `yaml:"no_traffic_interval,omitempty" json:"no_traffic_interval,omitempty"`
	UnhealthyInterval            *time.Duration     `yaml:"unhealthy_interval,omitempty" json:"unhealthy_interval,omitempty"`
	UnhealthyEdgeInterval        *time.Duration     `yaml:"unhealthy_edge_interval,omitempty" json:"unhealthy_edge_interval,omitempty"`
	HealthyEdgeInterval          *time.Duration     `yaml:"healthy_edge_interval,omitempty" json:"healthy_edge_interval,omitempty"`
	EventLogPath                 string             `yaml:"event_log_path,omitempty" json:"event_log_path,omitempty"`
	AlwaysLogHealthCheckFailures bool               `yaml:"always_log_health_check_failures,omitempty" json:"always_log_health_check_failures,omitempty"`
	TlsOptions                   *TlsOptions        `yaml:"tls_options,omitempty" json:"tls_options,omitempty"`
	TransportSocket              *TransportSocket   `yaml:"transport_socket,omitempty" json:"transport_socket,omitempty"`
}

// HttpHealthCheck defines HTTP health check
type HttpHealthCheck struct {
	Host                   string              `yaml:"host,omitempty" json:"host,omitempty"`
	Path                   string              `yaml:"path" json:"path"`
	Send                   string              `yaml:"send,omitempty" json:"send,omitempty"`
	Receive                []string            `yaml:"receive,omitempty" json:"receive,omitempty"`
	RequestHeaders         []HeaderValueOption `yaml:"request_headers_to_add,omitempty" json:"request_headers_to_add,omitempty"`
	RequestHeadersToRemove []string            `yaml:"request_headers_to_remove,omitempty" json:"request_headers_to_remove,omitempty"`
	ExpectedStatuses       []StatusRange       `yaml:"expected_statuses,omitempty" json:"expected_statuses,omitempty"`
	CodecClientType        string              `yaml:"codec_client_type,omitempty" json:"codec_client_type,omitempty"`
	ServiceNameMatcher     *StringMatcher      `yaml:"service_name_matcher,omitempty" json:"service_name_matcher,omitempty"`
}

// TcpHealthCheck defines TCP health check
type TcpHealthCheck struct {
	Send    string   `yaml:"send,omitempty" json:"send,omitempty"`
	Receive []string `yaml:"receive,omitempty" json:"receive,omitempty"`
}

// GrpcHealthCheck defines gRPC health check
type GrpcHealthCheck struct {
	ServiceName string `yaml:"service_name,omitempty" json:"service_name,omitempty"`
	Authority   string `yaml:"authority,omitempty" json:"authority,omitempty"`
}

// CustomHealthCheck defines custom health check
type CustomHealthCheck struct {
	Name        string                 `yaml:"name" json:"name"`
	TypedConfig map[string]interface{} `yaml:"typed_config,omitempty" json:"typed_config,omitempty"`
}

// OutlierDetection defines outlier detection configuration
type OutlierDetection struct {
	Consecutive5xx                         *int           `yaml:"consecutive_5xx,omitempty" json:"consecutive_5xx,omitempty"`
	Interval                               *time.Duration `yaml:"interval,omitempty" json:"interval,omitempty"`
	BaseEjectionTime                       *time.Duration `yaml:"base_ejection_time,omitempty" json:"base_ejection_time,omitempty"`
	MaxEjectionPercent                     *int           `yaml:"max_ejection_percent,omitempty" json:"max_ejection_percent,omitempty"`
	MinHealthPercent                       *int           `yaml:"min_health_percent,omitempty" json:"min_health_percent,omitempty"`
	SplitExternalLocalOriginErrors         bool           `yaml:"split_external_local_origin_errors,omitempty" json:"split_external_local_origin_errors,omitempty"`
	ConsecutiveLocalOriginFailure          *int           `yaml:"consecutive_local_origin_failure,omitempty" json:"consecutive_local_origin_failure,omitempty"`
	ConsecutiveGatewayFailure              *int           `yaml:"consecutive_gateway_failure,omitempty" json:"consecutive_gateway_failure,omitempty"`
	EnforcingConsecutive5xx                *int           `yaml:"enforcing_consecutive_5xx,omitempty" json:"enforcing_consecutive_5xx,omitempty"`
	EnforcingSuccessRate                   *int           `yaml:"enforcing_success_rate,omitempty" json:"enforcing_success_rate,omitempty"`
	SuccessRateMinimumHosts                *int           `yaml:"success_rate_minimum_hosts,omitempty" json:"success_rate_minimum_hosts,omitempty"`
	SuccessRateRequestVolume               *int           `yaml:"success_rate_request_volume,omitempty" json:"success_rate_request_volume,omitempty"`
	SuccessRateStdevFactor                 *int           `yaml:"success_rate_stdev_factor,omitempty" json:"success_rate_stdev_factor,omitempty"`
	EnforcingLocalOriginSuccessRate        *int           `yaml:"enforcing_local_origin_success_rate,omitempty" json:"enforcing_local_origin_success_rate,omitempty"`
	EnforcingConsecutiveLocalOriginFailure *int           `yaml:"enforcing_consecutive_local_origin_failure,omitempty" json:"enforcing_consecutive_local_origin_failure,omitempty"`
	EnforcingConsecutiveGatewayFailure     *int           `yaml:"enforcing_consecutive_gateway_failure,omitempty" json:"enforcing_consecutive_gateway_failure,omitempty"`
	MaxEjectionTime                        *time.Duration `yaml:"max_ejection_time,omitempty" json:"max_ejection_time,omitempty"`
}

// CircuitBreakers defines circuit breaker configuration
type CircuitBreakers struct {
	Thresholds []Thresholds `yaml:"thresholds,omitempty" json:"thresholds,omitempty"`
}

// Thresholds defines circuit breaker thresholds
type Thresholds struct {
	Priority           string       `yaml:"priority,omitempty" json:"priority,omitempty"`
	MaxConnections     *int         `yaml:"max_connections,omitempty" json:"max_connections,omitempty"`
	MaxPendingRequests *int         `yaml:"max_pending_requests,omitempty" json:"max_pending_requests,omitempty"`
	MaxRequests        *int         `yaml:"max_requests,omitempty" json:"max_requests,omitempty"`
	MaxRetries         *int         `yaml:"max_retries,omitempty" json:"max_retries,omitempty"`
	RetryBudget        *RetryBudget `yaml:"retry_budget,omitempty" json:"retry_budget,omitempty"`
	TrackRemaining     bool         `yaml:"track_remaining,omitempty" json:"track_remaining,omitempty"`
	MaxConnectionPools *int         `yaml:"max_connection_pools,omitempty" json:"max_connection_pools,omitempty"`
}

// RetryBudget defines retry budget configuration
type RetryBudget struct {
	BudgetPercent       *Percent `yaml:"budget_percent,omitempty" json:"budget_percent,omitempty"`
	MinRetryConcurrency *int     `yaml:"min_retry_concurrency,omitempty" json:"min_retry_concurrency,omitempty"`
}

// Http2ProtocolOptions defines HTTP/2 protocol options
type Http2ProtocolOptions struct {
	HpackTableSize                               *int `yaml:"hpack_table_size,omitempty" json:"hpack_table_size,omitempty"`
	MaxConcurrentStreams                         *int `yaml:"max_concurrent_streams,omitempty" json:"max_concurrent_streams,omitempty"`
	InitialStreamWindowSize                      *int `yaml:"initial_stream_window_size,omitempty" json:"initial_stream_window_size,omitempty"`
	InitialConnectionWindowSize                  *int `yaml:"initial_connection_window_size,omitempty" json:"initial_connection_window_size,omitempty"`
	AllowConnect                                 bool `yaml:"allow_connect,omitempty" json:"allow_connect,omitempty"`
	MaxOutboundFrames                            *int `yaml:"max_outbound_frames,omitempty" json:"max_outbound_frames,omitempty"`
	MaxOutboundControlFrames                     *int `yaml:"max_outbound_control_frames,omitempty" json:"max_outbound_control_frames,omitempty"`
	MaxConsecutiveInboundFramesWithEmptyPayload  *int `yaml:"max_consecutive_inbound_frames_with_empty_payload,omitempty" json:"max_consecutive_inbound_frames_with_empty_payload,omitempty"`
	MaxInboundPriorityFramesPerStream            *int `yaml:"max_inbound_priority_frames_per_stream,omitempty" json:"max_inbound_priority_frames_per_stream,omitempty"`
	MaxInboundWindowUpdateFramesPerDataFrameSent *int `yaml:"max_inbound_window_update_frames_per_data_frame_sent,omitempty" json:"max_inbound_window_update_frames_per_data_frame_sent,omitempty"`
	StreamErrorOnInvalidHttpMessaging            bool `yaml:"stream_error_on_invalid_http_messaging,omitempty" json:"stream_error_on_invalid_http_messaging,omitempty"`
	OverrideStreamErrorOnInvalidHttpMessage      bool `yaml:"override_stream_error_on_invalid_http_message,omitempty" json:"override_stream_error_on_invalid_http_message,omitempty"`
}

// HttpProtocolOptions defines HTTP/1.1 protocol options
type HttpProtocolOptions struct {
	IdleTimeout                  *time.Duration `yaml:"idle_timeout,omitempty" json:"idle_timeout,omitempty"`
	MaxConnectionDuration        *time.Duration `yaml:"max_connection_duration,omitempty" json:"max_connection_duration,omitempty"`
	MaxHeadersCount              *int           `yaml:"max_headers_count,omitempty" json:"max_headers_count,omitempty"`
	MaxStreamDuration            *time.Duration `yaml:"max_stream_duration,omitempty" json:"max_stream_duration,omitempty"`
	HeadersWithUnderscoresAction string         `yaml:"headers_with_underscores_action,omitempty" json:"headers_with_underscores_action,omitempty"`
	MaxRequestHeadersKb          *int           `yaml:"max_request_headers_kb,omitempty" json:"max_request_headers_kb,omitempty"`
}

// TransportSocket defines transport socket configuration
type TransportSocket struct {
	Name        string                 `yaml:"name" json:"name"`
	TypedConfig map[string]interface{} `yaml:"typed_config,omitempty" json:"typed_config,omitempty"`
}

// Additional supporting types
type Policy struct {
	DropOverloads          []DropOverload `yaml:"drop_overloads,omitempty" json:"drop_overloads,omitempty"`
	OverprovisioningFactor *int           `yaml:"overprovisioning_factor,omitempty" json:"overprovisioning_factor,omitempty"`
	EndpointStaleAfter     *time.Duration `yaml:"endpoint_stale_after,omitempty" json:"endpoint_stale_after,omitempty"`
	WeightedPriorityHealth bool           `yaml:"weighted_priority_health,omitempty" json:"weighted_priority_health,omitempty"`
}

type DropOverload struct {
	Category       string   `yaml:"category" json:"category"`
	DropPercentage *Percent `yaml:"drop_percentage" json:"drop_percentage"`
}

type Percent struct {
	Value float64 `yaml:"value" json:"value"`
}

type EnvoyHealthCheckConfig struct {
	PortValue int    `yaml:"port_value,omitempty" json:"port_value,omitempty"`
	Hostname  string `yaml:"hostname,omitempty" json:"hostname,omitempty"`
}

type StatusRange struct {
	Start int `yaml:"start" json:"start"`
	End   int `yaml:"end" json:"end"`
}

type StringMatcher struct {
	Exact    string `yaml:"exact,omitempty" json:"exact,omitempty"`
	Prefix   string `yaml:"prefix,omitempty" json:"prefix,omitempty"`
	Suffix   string `yaml:"suffix,omitempty" json:"suffix,omitempty"`
	Regex    string `yaml:"safe_regex,omitempty" json:"safe_regex,omitempty"`
	Contains string `yaml:"contains,omitempty" json:"contains,omitempty"`
}

type TlsOptions struct {
	AlpnProtocols []string `yaml:"alpn_protocols,omitempty" json:"alpn_protocols,omitempty"`
}

type HeaderValue struct {
	Key   string `yaml:"key" json:"key"`
	Value string `yaml:"value" json:"value"`
}

type UpstreamConnectionOptions struct {
	TcpKeepalive  *TcpKeepalive  `yaml:"tcp_keepalive,omitempty" json:"tcp_keepalive,omitempty"`
	SocketOptions []SocketOption `yaml:"socket_options,omitempty" json:"socket_options,omitempty"`
}

type TcpKeepalive struct {
	KeepaliveProbes   int `yaml:"keepalive_probes,omitempty" json:"keepalive_probes,omitempty"`
	KeepaliveTime     int `yaml:"keepalive_time,omitempty" json:"keepalive_time,omitempty"`
	KeepaliveInterval int `yaml:"keepalive_interval,omitempty" json:"keepalive_interval,omitempty"`
}

type SocketOption struct {
	Level    int    `yaml:"level" json:"level"`
	Name     int    `yaml:"name" json:"name"`
	Value    int    `yaml:"value,omitempty" json:"value,omitempty"`
	BufValue string `yaml:"buf_value,omitempty" json:"buf_value,omitempty"`
}

type CommonLbConfig struct {
	HealthyPanicThreshold           *Percent                   `yaml:"healthy_panic_threshold,omitempty" json:"healthy_panic_threshold,omitempty"`
	LocalityWeightedLbConfig        *LocalityWeightedLbConfig  `yaml:"locality_weighted_lb_config,omitempty" json:"locality_weighted_lb_config,omitempty"`
	UpdateMergeWindow               *time.Duration             `yaml:"update_merge_window,omitempty" json:"update_merge_window,omitempty"`
	IgnoreNewHostsUntilFirstHc      bool                       `yaml:"ignore_new_hosts_until_first_hc,omitempty" json:"ignore_new_hosts_until_first_hc,omitempty"`
	CloseConnectionsOnHostSetChange bool                       `yaml:"close_connections_on_host_set_change,omitempty" json:"close_connections_on_host_set_change,omitempty"`
	ConsistentHashingLbConfig       *ConsistentHashingLbConfig `yaml:"consistent_hashing_lb_config,omitempty" json:"consistent_hashing_lb_config,omitempty"`
}

type LocalityWeightedLbConfig struct{}

type ConsistentHashingLbConfig struct {
	UseHostnameForHashing bool `yaml:"use_hostname_for_hashing,omitempty" json:"use_hostname_for_hashing,omitempty"`
	HashBalanceFactor     *int `yaml:"hash_balance_factor,omitempty" json:"hash_balance_factor,omitempty"`
}

type UpstreamHttpProtocolOptions struct {
	AutoSni           bool `yaml:"auto_sni,omitempty" json:"auto_sni,omitempty"`
	AutoSanValidation bool `yaml:"auto_san_validation,omitempty" json:"auto_san_validation,omitempty"`
}

type CommonHttpProtocolOptions struct {
	IdleTimeout                  *time.Duration `yaml:"idle_timeout,omitempty" json:"idle_timeout,omitempty"`
	MaxConnectionDuration        *time.Duration `yaml:"max_connection_duration,omitempty" json:"max_connection_duration,omitempty"`
	MaxHeadersCount              *int           `yaml:"max_headers_count,omitempty" json:"max_headers_count,omitempty"`
	MaxStreamDuration            *time.Duration `yaml:"max_stream_duration,omitempty" json:"max_stream_duration,omitempty"`
	HeadersWithUnderscoresAction string         `yaml:"headers_with_underscores_action,omitempty" json:"headers_with_underscores_action,omitempty"`
	MaxRequestHeadersKb          *int           `yaml:"max_request_headers_kb,omitempty" json:"max_request_headers_kb,omitempty"`
}

type DnsFailureRefreshRate struct {
	BaseInterval *time.Duration `yaml:"base_interval" json:"base_interval"`
	MaxInterval  *time.Duration `yaml:"max_interval,omitempty" json:"max_interval,omitempty"`
}

type LbSubsetConfig struct {
	FallbackPolicy         string                 `yaml:"fallback_policy" json:"fallback_policy"`
	DefaultSubset          map[string]interface{} `yaml:"default_subset,omitempty" json:"default_subset,omitempty"`
	SubsetSelectors        []SubsetSelector       `yaml:"subset_selectors,omitempty" json:"subset_selectors,omitempty"`
	LocalityWeightAware    bool                   `yaml:"locality_weight_aware,omitempty" json:"locality_weight_aware,omitempty"`
	ScaleLocalityWeight    bool                   `yaml:"scale_locality_weight,omitempty" json:"scale_locality_weight,omitempty"`
	PanicModeAny           bool                   `yaml:"panic_mode_any,omitempty" json:"panic_mode_any,omitempty"`
	ListAsAny              bool                   `yaml:"list_as_any,omitempty" json:"list_as_any,omitempty"`
	MetadataFallbackPolicy string                 `yaml:"metadata_fallback_policy,omitempty" json:"metadata_fallback_policy,omitempty"`
}

type SubsetSelector struct {
	Keys                []string `yaml:"keys" json:"keys"`
	FallbackPolicy      string   `yaml:"fallback_policy,omitempty" json:"fallback_policy,omitempty"`
	SingleHostPerSubset bool     `yaml:"single_host_per_subset,omitempty" json:"single_host_per_subset,omitempty"`
}

type RingHashLbConfig struct {
	MinimumRingSize *int   `yaml:"minimum_ring_size,omitempty" json:"minimum_ring_size,omitempty"`
	HashFunction    string `yaml:"hash_function,omitempty" json:"hash_function,omitempty"`
	MaximumRingSize *int   `yaml:"maximum_ring_size,omitempty" json:"maximum_ring_size,omitempty"`
}

type MaglevLbConfig struct {
	TableSize *int `yaml:"table_size,omitempty" json:"table_size,omitempty"`
}

type OriginalDstLbConfig struct {
	UseHttpHeader bool `yaml:"use_http_header,omitempty" json:"use_http_header,omitempty"`
}

type LeastRequestLbConfig struct {
	ChoiceCount *int `yaml:"choice_count,omitempty" json:"choice_count,omitempty"`
}

type PreconnectPolicy struct {
	PerUpstreamPreconnectRatio *float64 `yaml:"per_upstream_preconnect_ratio,omitempty" json:"per_upstream_preconnect_ratio,omitempty"`
	PredictivePreconnectRatio  *float64 `yaml:"predictive_preconnect_ratio,omitempty" json:"predictive_preconnect_ratio,omitempty"`
}
