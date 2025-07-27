package types

type Orchestrator struct {
	Name      string             `yaml:"name"`
	Projects  map[string]Project `yaml:"projects"`
	Libraries map[string]Project `yaml:"libraries"`
	Tools     map[string]Project `yaml:"tools"`
	Actions   map[string]Action  `yaml:"actions"`
	Plugins   map[string]Plugin  `yaml:"plugins"`
}

type Plugin struct {
	Name string `yaml:"name"`
}
type Project struct {
	Name string `yaml:"name"`
	Path string `yaml:"path"`
	Repo string `yaml:"repo"`
	Type string `yaml:"type"`
}
type Action struct {
	Args  map[string]string `yaml:"args"`
	Tasks []Task            `yaml:"tasks"`
}

type Task struct {
	Executer string         `yaml:"executer"`
	Options  map[string]any `yaml:"options"`
	Input    map[string]any `yaml:"input"`
	Output   map[string]any
}

type TaskOptions struct {
}

// ========================================
// Core Plugin: Env

type EnvOptions struct {
	Items    []BuildItem // list of items to create final env file
	RootPath string      // orchestrator folder path
	Output   string      // output path for .env file

}

type BuildItem struct {
	File     string // e.g json, yaml, .env
	FilePath string // path to file reletive to root path
	Keys     []BuildItemKey
	EnvKey   []string // e.g "key=value" env to look for within a .env file (if empty copy entire file)
}

// for json/yaml file to find key values within a file
type BuildItemKey struct {
	Key     string // path to value within a map (e.g "web.clientID")
	EnvName string // name of env within final file (e.g "CLIENT_ID=value")
}

// ========================================
