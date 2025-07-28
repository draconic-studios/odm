package actions

import (
	"fmt"
	"odm/types"
	"odm/utils"
	"os"
	"path/filepath"
	"strings"
)

type DocPaths struct {
	Folders []DocItem
	Files   []DocItem
}

type DocItem struct {
	Path         string
	ReletivePath string
	Name         string
}

func BuildDocs(command *utils.Command, orchestrationConfig *types.Orchestrator) (string, error) {

	var rootPath string
	var docsConfig *types.Documentaton
	var outputPath string

	if orchestrationConfig.Documentaton.Output == "" {
		return "", fmt.Errorf("documentation configuration invalid")
	}
	// Parse input
	if rootPathValue, ok := command.Flags["root-path"]; ok {
		rootPath = rootPathValue
	}

	docsConfig = &orchestrationConfig.Documentaton
	outputPath = filepath.Join(docsConfig.Output, "docs")

	// Setup build folder
	fmt.Println("Building Folder: ", rootPath, docsConfig.Output)
	err := utils.CreateFolder(rootPath, docsConfig.Output)
	if err != nil {
		return "", err
	}
	fmt.Println("Building Folder: ", rootPath, outputPath)
	err = utils.CreateFolder(rootPath, outputPath)
	if err != nil {
		return "", err
	}

	// Get all submodules
	subModules := []types.Project{}
	for _, s := range orchestrationConfig.Projects {
		subModules = append(subModules, s)
	}
	for _, s := range orchestrationConfig.Libraries {
		subModules = append(subModules, s)
	}
	for _, s := range orchestrationConfig.Tools {
		subModules = append(subModules, s)
	}

	docsList := []DocPaths{}

	// Get all docs
	for _, p := range subModules {
		fmt.Printf("\n\nFinding docs for %s\n", p.Name)

		err := handleProjectDocs(&p, rootPath, docsConfig.Output)
		if err != nil {
			fmt.Println(err)
			continue
		}
	}

	fmt.Println(docsList)

	// Create folders
	for _, f := range docsList {
		for _, fold := range f.Folders {
			fmt.Println("Creating folder: ", outputPath, fold.ReletivePath)
			err = utils.CreateFolder(outputPath, fold.ReletivePath)
			if err != nil {
				fmt.Println(err)
				continue
			}

		}
		for _, fil := range f.Files {
			destination := filepath.Join(outputPath, fil.ReletivePath)
			fmt.Println("Creating file: ", fil.Path, destination)
			err = utils.CopyFile(fil.Path, destination)
			if err != nil {
				fmt.Println(err)
				return "", err
			}

		}
	}

	// Build js server
	err = buildDocServer(docsConfig.Output, outputPath)
	if err != nil {
		return "", err
	}

	// Build sidebar for page
	err = buildDocsSidebar(outputPath)
	if err != nil {
		return "", err
	}

	emptyString := ""
	err = utils.WriteFile(filepath.Join(outputPath, ".nojekyll"), &emptyString)
	if err != nil {
		return "", err
	}

	return "", nil
}

func handleProjectDocs(project *types.Project, rootPath string, output string) error {
	var docPaths DocPaths
	// project path
	projectPath := filepath.Join(rootPath, project.Path)
	fmt.Println("Project Path: ", projectPath)
	// Path to docs inside of project
	docsPath := filepath.Join(projectPath, "docs")
	fmt.Println("Docs Path: ", docPaths)
	// check for docs folder
	err := utils.FolderExists(docsPath)
	if err != nil {
		fmt.Println(err)
		return err
	}
	buildPath := filepath.Join(rootPath, output, "docs", project.Type, project.Name)

	fmt.Println("Docs Found Copy: ", docsPath, buildPath)
	// Copy docs folder of project into docs server
	utils.CopyFolderContents(docsPath, buildPath)
	return nil
}

func buildDocsSidebar(contentPath string) error {
	fmt.Println("Building sidebar...")
	contents, err := utils.ReadFolderContents(contentPath)
	if err != nil {
		fmt.Println(err)
		return err
	}

	list, err := createSidebarStrings(contents, contentPath, []string{}, 0)
	if err != nil {
		return err
	}

	fmt.Println("Sidebar entries: ", list)

	newList := []string{}
	for _, i := range list {
		newList = append(newList, strings.ReplaceAll(i, contentPath, ""))
	}

	sidebarMD := strings.Join(newList, "\n")

	utils.WriteFile(filepath.Join(contentPath, "_sidebar.md"), &sidebarMD)

	return nil

}

func createSidebarStrings(contents *[]os.DirEntry, basePath string, list []string, indent int) ([]string, error) {
	newList := []string{}
	newList = append(newList, list...)

	for _, item := range *contents {
		itemInfo, err := item.Info()
		if err != nil {
			fmt.Println(err)
			return newList, err
		}
		newPath := filepath.Join(basePath, item.Name())
		fileName := itemInfo.Name()

		fmt.Printf("Content Item: %s\n", fileName)

		// If is folder
		if item.IsDir() {
			fmt.Println("Item is Dir")
			newList = append(list, createSidebaritem(fileName, "header", "", indent))
			newContents, err := utils.ReadFolderContents(newPath)
			if err != nil {
				fmt.Println(err)
				return newList, err
			}
			newList, err = createSidebarStrings(newContents, newPath, newList, indent+2)
			if err != nil {
				return nil, err
			}
			continue
		}

		// If is file
		if strings.Contains(fileName, ".md") || strings.Contains(fileName, "md") {
			fmt.Println("Item is markdown")
			newList = append(newList, createSidebaritem(fileName, "link", newPath, indent))
			continue
		}

	}

	return newList, nil
}

func createSidebaritem(text string, itemType string, url string, indent int) string {
	// - [Home](README.md)
	// - Guide
	//   - [test](react-nexus-lib/test.md)
	// - API
	//   - [Authentication](api/authentication.md)
	indentText := ""
	for range indent {
		indentText = fmt.Sprintf("%s ", indentText)
	}

	switch itemType {
	case "header":
		return fmt.Sprintf("%s - %s", indentText, text)
	case "link":
		return fmt.Sprintf("%s - [%s](%s)", indentText, text, url)

	default:
		return ""
	}
}

func buildDocServer(baseOutput string, contentPath string) error {
	html := types.DocsHtml
	packageJson := types.DocsPackageJson

	err := utils.WriteFile(filepath.Join(contentPath, "index.html"), &html)
	if err != nil {
		return err
	}
	err = utils.WriteFile(filepath.Join(baseOutput, "package.json"), &packageJson)
	if err != nil {
		return err
	}
	return nil
}
