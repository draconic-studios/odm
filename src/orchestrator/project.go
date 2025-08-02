package orchestrator

import "fmt"

// get project by name
func (o *Orchestrator) GetProject(projectName string) (*Project, error) {

	// get project using name provided
	if p, ok := o.Config.Projects[projectName]; ok {
		return &p, nil
	}

	// project not found
	return nil, fmt.Errorf("project: %s not found", projectName)
}

// update or add project
func (o *Orchestrator) UpdateProject(project *Project) error {
	// validate project fields
	if project.Path == "" {
		return fmt.Errorf("path not found: %s", project.Path)
	}
	if project.Repo == "" {
		return fmt.Errorf("repo not found: %s", project.Repo)
	}
	if project.Type == "" {
		return fmt.Errorf("type not found: %s", project.Type)
	}
	if project.Name == "" {
		return fmt.Errorf("name not found: %s", project.Name)
	}

	// add project
	o.Config.Projects[project.Name] = *project

	return nil
}

// delete project by name
func (o *Orchestrator) DeleteProject(projectName string) error {
	// validate name
	if projectName == "" {
		return fmt.Errorf("project name not provided")
	}

	// delete project by name
	if _, ok := o.Config.Projects[projectName]; ok {
		delete(o.Config.Projects, projectName)
		return nil
	}

	// project not found
	return fmt.Errorf("project: %s not found", projectName)

}
