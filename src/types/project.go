package types

type Project struct {
	Name      string                    `json:"name"`
	Services  map[string]ProjectService `json:"services"`
	Libraries map[string]ProjectLibrary `json:"libraries"`
	Tools     map[string]ProjectTool    `json:"tools"`
}

type ProjectService struct {
	Name string `json:"name"`
	Url  string `json:"url"`
	Type string `json:"type"`
}
type ProjectLibrary struct {
	Name string `json:"name"`
	Url  string `json:"url"`
}
type ProjectTool struct {
	Name string `json:"name"`
	Url  string `json:"url"`
}
