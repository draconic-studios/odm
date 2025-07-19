package environment

import (
	"bufio"
	"encoding/json"
	"fmt"
	"maps"
	"odm/types"
	"odm/utils"
	"os"
	"path/filepath"
	"strings"
)

func EnvBuilder(config *types.BuildOptions) error {
	configPath := filepath.Join(config.ProjectPath, config.ConfigFolder, config.BuildType, ".env")
	outputPath := filepath.Join(config.ProjectPath, config.Output, ".env")

	lines, err := ReadEnvFile(configPath)
	if err != nil {
		return err
	}

	envMap, err := MapEnvLines(lines)
	if err != nil {
		return err
	}

	// Get Google Creds
	googleEnvMap, err := ConvertGoogleAuthCreds(filepath.Join(config.ProjectPath, config.ConfigFolder, config.BuildType))
	if err != nil {
		return err
	}
	maps.Copy((*envMap), *googleEnvMap)

	err = WriteEnv(envMap, outputPath)
	if err != nil {
		return err
	}

	return nil

}

func WriteEnv(env *map[string]string, output string) error {

	content := ""

	for k, v := range *env {
		content = fmt.Sprintf("%s\n%s=%s", content, k, v)

	}

	err := os.WriteFile(output, []byte(content), 0644)
	if err != nil {
		return err
	}
	return nil
}

func MapEnvLines(lines *[]string) (*map[string]string, error) {
	// Create map from spliting lines by "="
	envMap := map[string]string{}

	for _, env := range *lines {
		parts := strings.Split(env, "=")
		envMap[parts[0]] = parts[1]
	}

	return &envMap, nil
}

func ReadEnvFile(envPath string) (*[]string, error) {
	file, err := os.Open(envPath) // Replace with your file path
	if err != nil {
		return nil, fmt.Errorf("failed to open file: %v", err)
	}
	defer file.Close() // Ensure the file is closed

	var scrubbedLines []string

	scanner := bufio.NewScanner(file)

	// Remove null lines and remove comments
	for scanner.Scan() {
		line := scanner.Text()

		before, _, found := strings.Cut(line, "#")
		if found {
			if before != "" {
				scrubbedLines = append(scrubbedLines, before)
			}
		} else if line != "" {
			scrubbedLines = append(scrubbedLines, line)
		}

	}

	if err := scanner.Err(); err != nil {
		return nil, fmt.Errorf("error reading file: %v", err)
	}

	return &scrubbedLines, nil
}

func ConvertGoogleAuthCreds(configFolder string) (*map[string]string, error) {
	credsPath := filepath.Join(configFolder, "creds")
	googleCredsFilePath := ""

	credsFolderContents, err := utils.ReadFolderContents(credsPath)
	if err != nil {
		return nil, err
	}
	for _, item := range *credsFolderContents {
		if strings.Contains(item.Name(), "google_client_creds") {
			googleCredsFilePath = filepath.Join(credsPath, item.Name())
		}
	}

	fileBytes, err := os.ReadFile(googleCredsFilePath)
	if err != nil {
		return nil, err
	}

	var creds types.GoogleOAuthCreds
	// Unmarshal the JSON string (converted to bytes) into the struct
	err = json.Unmarshal(fileBytes, &creds)
	if err != nil {
		return nil, err
	}

	return &map[string]string{
		"GOOGLE_CLIENT_ID":     creds.Web.ClientID,
		"GOOGLE_CLIENT_SECRET": creds.Web.ClientSecret,
		"GOOGLE_PROJECT_ID":    creds.Web.ProjectID,
		"GOOGLE_REDIRECT_URL":  creds.Web.RedirectUris[0],
	}, nil
}
