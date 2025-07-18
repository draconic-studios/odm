package projects

import (
	"encoding/json" // For JSON encoding/decoding
	"fmt"           // For logging errors
	"os"            // For file operations
)

// Define Go structs that match the structure of your JSON
// Use `json:"fieldName"` tags to map JSON keys to struct fields.
// Only exported fields (starting with a capital letter) will be unmarshaled.

type Project struct {
	Name       string    `json:"name"`
	Submodules Submodule `json:"submodules"`
}

type Submodule struct {
	Services  map[string]Service `json:"services"`
	Libraries map[string]Service `json:"libraries"`
	Tools     map[string]Service `json:"tools"`
}

type Service struct {
	Name string `json:"name"`
	Url  string `json:"url"`
}
type Library struct {
	Name string `json:"name"`
	Url  string `json:"url"`
}
type Tool struct {
	Name string `json:"name"`
	Url  string `json:"url"`
}

func ReadProject(filePath string) (*Project, error) {

	// Step 1: Read the entire file content into a byte slice
	jsonData, err := os.ReadFile(filePath)
	if err != nil {
		return nil, err
	}

	fmt.Printf("Successfully read file: %s\n", filePath)
	// Optional: Print raw JSON data (for debugging)
	// fmt.Println("Raw JSON data:", string(jsonData))

	// Step 2: Unmarshal the JSON data into your Go struct
	var project Project
	err = json.Unmarshal(jsonData, &project)
	if err != nil {
		return nil, err
	}

	fmt.Println("\n--- Parsed Project ---")
	fmt.Println(project.Submodules)
	return &project, err
}
