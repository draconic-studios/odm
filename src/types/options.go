package types

type ExecuterOptions struct {
	BuildType      string   //Type of build (dev, prod)
	ProjectPath    string   // Path to the root of you project
	Output         string   // Name of the folder to build the output into
	ServicesFolder string   // name of the folder within your projects name
	Services       []string // an array of string (names of the service "folder name")
	BasePath       string   // path from your project root to the folder containing a base level file (docker-compose.yml etc)
	ConfigFolder   string   // name of folder containing config
}

// Define structures to hold parsed command-line data
type Command struct {
	Name      string
	Args      []string          // Positional arguments
	Flags     map[string]string // Key-value pairs for flags
	BoolFlags map[string]bool
	Help      bool
}
