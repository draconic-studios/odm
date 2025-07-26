package plugin

import (
	"context"
	"net/rpc" // Needed for rpc.Client in the Client/Server methods

	"github.com/hashicorp/go-plugin" // Use go-plugin lib
)

type ExecutionRequestBody struct {
	Args    map[string]string `json:"args"`
	Options map[string]any    `json:"options"`
	Input   string            `json:"input"`
}

// Executer is the interface that our plugin will implement.
type Executer interface {
	Execute(ctx context.Context, body string) (string, error)
}

// ExecuterPlugin is the plugin definition for go-plugin.
// It embeds the plugin.Plugin interface.
type ExecuterPlugin struct {
	plugin.Plugin
	Impl Executer
}

// Client returns the client-side implementation of the plugin.
// This method is required by the plugin.Plugin interface.
func (p *ExecuterPlugin) Client(broker *plugin.MuxBroker, client *rpc.Client) (interface{}, error) {
	return &ExecuterRPCClient{client: client}, nil
}

// Server returns the server-side implementation of the plugin.
// This method is required by the plugin.Plugin interface.
func (p *ExecuterPlugin) Server(broker *plugin.MuxBroker) (interface{}, error) {
	return &ExecuterRPCServer{Impl: p.Impl}, nil
}

// ExecuterRPCClient implements the Executer interface for the host side.
type ExecuterRPCClient struct {
	client *rpc.Client
}

func (g *ExecuterRPCClient) Execute(ctx context.Context, body string) (string, error) {
	var resp string
	// Call the GreetRPC method on the plugin server
	// err := g.client.Call("Plugin.Greet", map[string]interface{}{"ctx": ctx, "name": name}, &resp)
	err := g.client.Call("Plugin.Execute", body, &resp)
	return resp, err
}

// ExecuterRPCServer implements the RPC server for the plugin side.
type ExecuterRPCServer struct {
	Impl Executer
}

func (g *ExecuterRPCServer) Execute(body string, resp *string) error {
	// Use a background context for the plugin implementation
	ctx := context.Background()

	result, err := g.Impl.Execute(ctx, body)
	if err != nil {
		return err
	}
	*resp = result
	return nil
}

// HandshakeConfig is used to ensure the host and plugin can find each other.
var HandshakeConfig = plugin.HandshakeConfig{
	ProtocolVersion:  1,
	MagicCookieKey:   "magic",
	MagicCookieValue: "cookie",
}

// PluginMap is the map of plugins we support.
var PluginMap = map[string]plugin.Plugin{
	"executer": &ExecuterPlugin{},
}
