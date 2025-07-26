package plugin

import (
	"context"
	"fmt"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"

	"github.com/hashicorp/go-plugin"
)

// TODO Log verbose

type PluginManager struct {
	pluginDir    string
	pluginSuffix string
	verbose      bool
	clients      map[string]*plugin.Client
	Plugins      []string
}

type PluginManagerOptions struct {
	PluginDir    string
	PluginSuffix string
	Verbose      bool
}

func NewPluginManager(options *PluginManagerOptions) *PluginManager {
	newManager := &PluginManager{
		pluginDir:    options.PluginDir,
		pluginSuffix: options.PluginSuffix,
		verbose:      options.Verbose,
		clients:      make(map[string]*plugin.Client),
	}
	newManager.init()

	return newManager
}

func (pm *PluginManager) printVerbose(text string) {
	if pm.verbose {
		fmt.Println(text)
	}
}

func (pm *PluginManager) loadPlugin(name string, pluginsPath string) (Executer, error) {
	pluginPath := filepath.Join(pluginsPath, name)

	// Check if plugin binary exists
	if _, err := os.Stat(pluginPath); os.IsNotExist(err) {
		return nil, fmt.Errorf("plugin not found: %s", pluginPath)
	}

	// Create plugin client
	client := plugin.NewClient(&plugin.ClientConfig{
		HandshakeConfig:  HandshakeConfig,
		Plugins:          PluginMap, // TODO Need this to map to all plugins a user may install
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
	executer, ok := raw.(Executer)
	if !ok {
		client.Kill()
		delete(pm.clients, name)
		return nil, fmt.Errorf("failed to type assert plugin to Executer interface")
	}

	return executer, nil
}

// discoverPlugins finds all plugins with a specified suffix within a directory path
func (pm *PluginManager) discoverPlugins(suffix string, pluginPath string) ([]string, error) {
	files, err := os.ReadDir(pluginPath)
	if err != nil {
		return nil, err
	}

	var plugins []string
	for _, file := range files {
		if file.IsDir() {
			continue
		}

		// Check if file is executable and matches naming convention
		name := file.Name()
		if strings.HasSuffix(name, suffix) {
			pluginPath := filepath.Join(pluginPath, name)
			if info, err := os.Stat(pluginPath); err == nil && info.Mode()&0111 != 0 {
				plugins = append(plugins, name)
			}
		}
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
	// Default suffix to "-plugin" on filename
	if pm.pluginSuffix == "" {
		pm.pluginSuffix = "-plugin"
	}
	plugins, err := pm.discoverPlugins(pm.pluginSuffix, pm.pluginDir)
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
	executer, err := pm.loadPlugin(pluginName, pm.pluginDir)

	if err != nil {
		return "", fmt.Errorf("failed to load plugin %s: %v", pluginName, err)

	}

	// Test the plugin
	// ! not sure if this really tests it
	// TODO clarify and create a proper test if required
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
