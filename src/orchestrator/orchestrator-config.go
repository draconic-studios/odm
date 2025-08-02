package orchestrator

type OrchestratorConfig struct {
	Name         string             `yaml:"name" json:"name"`
	Documentaton Documentaton       `yaml:"documentaton" json:"documentaton"`
	Projects     map[string]Project `yaml:"projects" json:"projects"`
	Actions      map[string]Action  `yaml:"actions" json:"actions"`
	Plugins      map[string]Plugin  `yaml:"plugins" json:"plugins"`
	PluginConfig PluginConfig       `yaml:"plugin-config" json:"plugin-config"`
}

type PluginConfig struct {
	Location     string `yaml:"location" json:"location"`
	PluginSuffix string `yaml:"plugin-suffix" json:"plugin-suffix"`
}

type Plugin struct {
	Name string `yaml:"name" json:"name"` // name of plugin
	Path string `ymal:"path" json:"path"` // path to plugin executable
	Type string `ymal:"type" json:"type"` // Language plugin is written in (node.js, go, python)
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
