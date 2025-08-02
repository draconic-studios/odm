package plugin

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"odm/utils"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"

	"github.com/hashicorp/go-plugin"
	odmPlugin "github.com/hembrow-innovations/odm-plugin"
)

// plugin struct
type PluginManager struct {
	pluginDir string
	verbose   bool
	clients   map[string]*plugin.Client
	Plugins   []PluginDeclaration
}

// options to configure plugin manager
type PluginManagerOptions struct {
	PluginDir string
	Verbose   bool
}

// PluginConfig defines the structure of our plugin manifest.
type PluginDeclaration struct {
	Name     string `json:"name"`
	Version  string `json:"version"`
	Language string `json:"language"`
	Source   string `json:"source"`
	Type     string `json:"type"`
	Package  string `json:"package"`
}

// create instance of plugin manager
func NewPluginManager(options *PluginManagerOptions) *PluginManager {
	newManager := &PluginManager{
		pluginDir: options.PluginDir,
		verbose:   options.Verbose,
		clients:   make(map[string]*plugin.Client),
	}
	newManager.init()

	return newManager
}

// print text
func (pm *PluginManager) printVerbose(text string) {
	if pm.verbose {
		fmt.Println(text)
	}
}

// Get plugin dec from pm
func (pm *PluginManager) getPluginInfo(name string) (*PluginDeclaration, error) {
	var pl *PluginDeclaration
	for _, p := range pm.Plugins {
		if p.Name == name {
			pl = &p
		}
	}

	if pl == nil {
		return nil, fmt.Errorf("plugin not found")
	}

	return pl, nil
}

// load plugin from path
func (pm *PluginManager) loadPlugin(name string) (odmPlugin.Executer, error) {

	pl, err := pm.getPluginInfo(name)
	if err != nil {
		return nil, err
	}

	pluginPath := pl.Source

	// Check if plugin binary exists
	if _, err := os.Stat(pluginPath); os.IsNotExist(err) {
		return nil, fmt.Errorf("plugin not found: %s", pluginPath)
	}

	// Create plugin client
	client := plugin.NewClient(&plugin.ClientConfig{
		HandshakeConfig:  odmPlugin.HandshakeConfig,
		Plugins:          odmPlugin.PluginMap, // TODO Need this to map to all plugins a user may install
		Cmd:              exec.Command(pluginPath),
		AllowedProtocols: []plugin.Protocol{plugin.ProtocolNetRPC},
		SyncStdout:       os.Stdout,
		SyncStderr:       os.Stderr,
	})

	// Store client for cleanup
	pm.clients[name] = client

	// Connect via RPC
	rpcClient, err := client.Client()
	if err != nil {
		client.Kill()
		delete(pm.clients, name)
		return nil, fmt.Errorf("error getting RPC client: %v", err)
	}

	// Dispense the plugin
	raw, err := rpcClient.Dispense("executer")
	if err != nil {
		client.Kill()
		delete(pm.clients, name)
		return nil, fmt.Errorf("error dispensing plugin: %v", err)
	}

	// Type assert
	executer, ok := raw.(odmPlugin.Executer)
	if !ok {
		client.Kill()
		delete(pm.clients, name)
		return nil, fmt.Errorf("failed to type assert plugin to Executer interface")
	}

	return executer, nil
}

// Read plugin declaration file
func (pm *PluginManager) readDeclaration(declarationPath string) (*PluginDeclaration, error) {
	var pluginDeclaration PluginDeclaration
	dataBytes, err := os.ReadFile(declarationPath)
	if err != nil {
		return nil, err
	}

	err = json.Unmarshal(dataBytes, &pluginDeclaration)
	if err != nil {
		return nil, err
	}

	return &pluginDeclaration, nil
}

// discoverPlugins finds all plugins with a specified suffix within a directory path
func (pm *PluginManager) discoverPlugins(pluginPath string) ([]PluginDeclaration, error) {

	decFolder := filepath.Join(pluginPath, "declarations")
	contents, err := utils.ReadFolderContents(decFolder)
	if err != nil {
		return nil, err
	}

	var plugins []PluginDeclaration
	for _, item := range *contents {
		if item.IsDir() {
			continue
		}

		// Check if file is executable and matches naming convention
		name := item.Name()

		if strings.Contains(name, ".json") {
			decPath := filepath.Join(decFolder, name)
			pluginDec, err := pm.readDeclaration(decPath)
			if err != nil {
				continue
			}
			plugins = append(plugins, *pluginDec)

		}
		// if strings.HasSuffix(name, suffix) {
		// 	pluginPath := filepath.Join(pluginPath, name)
		// 	if info, err := os.Stat(pluginPath); err == nil && info.Mode()&0111 != 0 {
		// 		plugins = append(plugins, name)
		// 	}
		// }
	}

	return plugins, nil
}

// cleanup cleans up after plugin execution
func (pm *PluginManager) cleanup() {
	for name, client := range pm.clients {
		log.Printf("Killing plugin: %s", name)
		client.Kill()
	}
	pm.clients = make(map[string]*plugin.Client)
}

// init builds the plugin manager so that plug/s can be executed
func (pm *PluginManager) init() error {
	// Initialize plugin manager
	if pm.pluginDir == "" {
		return fmt.Errorf("plugin directory not found: '%s'", pm.pluginDir)
	}

	// Discover available plugins

	plugins, err := pm.discoverPlugins(pm.pluginDir)
	if err != nil {
		return fmt.Errorf("error discovering plugins: %v", err)
	}
	pm.printVerbose(fmt.Sprintf("Found %d plugins: %v", len(plugins), plugins))

	pm.Plugins = plugins

	return nil
}

// Run executes the plugin
func (pm *PluginManager) Run(pluginName string, body string) (string, error) {
	pm.printVerbose(fmt.Sprintf("Starting plugin: %s execution...", pluginName))

	defer pm.cleanup()

	pm.printVerbose("Loading plugin")
	executer, err := pm.loadPlugin(pluginName)

	if err != nil {
		return "", fmt.Errorf("failed to load plugin %s: %v", pluginName, err)

	}

	// Execute plugin
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)

	pm.printVerbose("Calling plugin...")
	// Execute plugin
	resp, err := executer.Execute(ctx, body)
	if err != nil {
		cancel()
		return "", fmt.Errorf("error calling plugin %s: %v", pluginName, err)
	}

	pm.printVerbose(fmt.Sprintf("Plugin %s response: %s\n", pluginName, resp))
	cancel()

	pm.printVerbose("Plugin execution finished.")
	return resp, nil
}
