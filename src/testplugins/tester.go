package main

import (
	"context"
	"encoding/json"
	"log"

	odmPlugin "odm/plugin" // Replace with your actual module path

	"github.com/hashicorp/go-plugin"
)

// ExecuterImpl is the concrete implementation of our Executer interface.
type ExecuterImpl struct {
	plugin.Plugin
}

// Greet implements the Greeter interface.
// This signature MUST match shared.Greeter's Greet method.
func (g *ExecuterImpl) Execute(ctx context.Context, body string) (string, error) {

	requestBody := &odmPlugin.ExecutionRequestBody{}

	err := json.Unmarshal([]byte(body), requestBody)
	if err != nil {
		return "", err
	}

	log.Printf("Plugin: Execute called with body: \n\tArguments:%s\n\tInput: %s\n\tOptions: %s", requestBody.Args, requestBody.Input, requestBody.Options)
	return "Success", nil
}

func main() {
	log.Println("Starting Tester plugin...")

	// The plugin must export an implementation of the Greeter interface.
	var handshakeConfig = odmPlugin.HandshakeConfig
	var pluginMap = map[string]plugin.Plugin{
		"executer": &odmPlugin.ExecuterPlugin{Impl: &ExecuterImpl{}},
	}

	plugin.Serve(&plugin.ServeConfig{
		HandshakeConfig: handshakeConfig,
		Plugins:         pluginMap,
		// GRPCServer:      nil,
		// GRPCProvider:    nil, // We're using standard RPC for simplicity
	})
	log.Println("Greeter plugin finished serving.")
	// This line should never be reached under normal circumstances
	log.Println("Plugin: Serve returned (this shouldn't happen)")
}
