package types

type Project struct {
	Name      string                 `yaml:"name"`
	Projects  map[string]ProjectRepo `yaml:"services"`
	Libraries map[string]ProjectRepo `yaml:"libraries"`
	Tools     map[string]ProjectRepo `yaml:"tools"`
	Actions   map[string]Action      `yaml:"actions"`
	Plugins   map[string]Plugin      `yaml:"plugins"`
}

type Plugin struct {
	Name string `yaml:"name"`
}
type ProjectRepo struct {
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
