package types

type Orchestrator struct {
	Name         string             `yaml:"name" json:"name"`
	Documentaton Documentaton       `yaml:"documentaton" json:"documentaton"`
	Projects     map[string]Project `yaml:"projects" json:"projects"`
	Libraries    map[string]Project `yaml:"libraries" json:"libraries"`
	Tools        map[string]Project `yaml:"tools" json:"tools"`
	Actions      map[string]Action  `yaml:"actions" json:"actions"`
	Plugins      map[string]Plugin  `yaml:"plugins" json:"plugins"`
}

type Plugin struct {
	Name string `yaml:"name" json:"name"`
}
type Project struct {
	Name string `yaml:"name" json:"name"`
	Path string `yaml:"path" json:"path"`
	Repo string `yaml:"repo" json:"repo"`
	Type string `yaml:"type" json:"type"`
}
type Action struct {
	Args  map[string]string `yaml:"args" json:"args"`
	Tasks []Task            `yaml:"tasks" json:"tasks"`
}

type Task struct {
	Executer string         `yaml:"executer" json:"executer"`
	Options  map[string]any `yaml:"options" json:"options"`
	Input    map[string]any `yaml:"input" json:"input"`
	Output   map[string]any
}

// ========================================
// Core Action: build docs
type Documentaton struct {
	DocsPath string `yaml:"docs-path" json:"docs-path"`
	DocType  string `yaml:"doc-type" json:"doc-type"`
	Output   string `yaml:"output" json:"output"`
}

// ========================================
// Core Plugin: Env

type EnvOptions struct {
	Items  []BuildItem // list of items to create final env file
	Output string      // output path for .env file

}

type BuildItem struct {
	File     string // e.g json, yaml, env
	FilePath string // path to file reletive to root path
	Keys     []BuildItemKey
	EnvKeys  []string // e.g "key=value" env to look for within a .env file (if empty copy entire file)
}

// for json/yaml file to find key values within a file
type BuildItemKey struct {
	Key     string // path to value within a map (e.g "web.clientID")
	EnvName string // name of env within final file (e.g "CLIENT_ID=value")
}

// ========================================
