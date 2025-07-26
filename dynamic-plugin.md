You're absolutely right\! The primary purpose of a plugin system is to allow extending functionality _without_ recompiling the main application. My previous example still had a hardcoded map in `main.go`, which required recompilation. Let's fix that.

To achieve true dynamic loading without recompiling the main application, you need to:

1.  **Discover Plugins:** Instead of a hardcoded list, the host application needs to discover available plugin binaries. The most common way is to scan a designated plugin directory.
2.  **Load Plugins:** Once discovered, load each plugin dynamically.
3.  **Manage Configuration:** Allow users to specify which plugins to load and perhaps pass specific configurations to them, often via a configuration file.

Here's how we'll modify the Greeter example to achieve this:

### Modified Project Structure (No Change, Just for Clarity)

```
greeter-plugin-system/
├── go.mod
├── go.sum
├── main.go               # Host application (will NOT be recompiled for new plugins)
├── config.json           # New: Configuration file for plugins
├── plugins/              # Directory for plugin binaries
│   └── greeter-en/       # English Greeter Plugin
│       ├── main.go
│       └── go.mod
│       └── go.sum
│   └── greeter-es/       # Spanish Greeter Plugin
│       ├── main.go
│       └── go.mod
│       └── go.sum
│   └── greeter-fr/       # French Greeter Plugin (newly added for demonstration)
│       ├── main.go
│       └── go.mod
│       └── go.sum
└── proto/
    ├── greeter.proto
    └── gen/
        └── greeter.pb.go
        └── greeter_grpc.pb.go
└── shared/
    └── plugin.go
```

### Step 1: Configuration File (`config.json`)

We'll introduce a `config.json` file at the root of the `greeter-plugin-system` directory. This file will list the plugins to load and their paths.

```json
// config.json
{
  "plugins_directory": "./plugins",
  "enabled_plugins": [
    {
      "name": "english-greeter",
      "path": "greeter-en/greeter-en",
      "language_code": "en"
    },
    {
      "name": "spanish-greeter",
      "path": "greeter-es/greeter-es",
      "language_code": "es"
    },
    {
      "name": "french-greeter",
      "path": "greeter-fr/greeter-fr",
      "language_code": "fr"
    }
  ]
}
```

**Explanation:**

- `plugins_directory`: The base directory where plugin binaries are located.
- `enabled_plugins`: An array of objects, where each object defines a plugin to be loaded.
  - `name`: A unique identifier for the plugin (useful for logging/referencing).
  - `path`: The path to the plugin's executable, relative to `plugins_directory`.
  - `language_code`: An example of plugin-specific configuration.

### Step 2: Modify the Host Application (`main.go`)

The main application will now read this `config.json` file, dynamically discover and load plugins based on its content.

```go
// main.go
package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"os/exec"
	"path/filepath" // For joining paths
	"time"

	"greeter-plugin-system/shared" // Import the shared plugin definition

	hclog "github.com/hashicorp/go-hclog"
	"github.com/hashicorp/go-plugin"
)

// PluginConfig represents a single plugin's configuration from the JSON file
type PluginConfig struct {
	Name         string `json:"name"`
	Path         string `json:"path"`
	LanguageCode string `json:"language_code"` // Example of plugin-specific config
}

// AppConfig represents the overall application configuration
type AppConfig struct {
	PluginsDirectory string         `json:"plugins_directory"`
	EnabledPlugins   []PluginConfig `json:"enabled_plugins"`
}

func main() {
	logger := hclog.New(&hclog.LoggerOptions{
		Name:   "host",
		Output: os.Stderr,
		Level:  hclog.Debug,
	})

	// 1. Load configuration from file
	configPath := "./config.json"
	configBytes, err := os.ReadFile(configPath)
	if err != nil {
		log.Fatalf("Failed to read config file %s: %v", configPath, err)
	}

	var appConfig AppConfig
	if err := json.Unmarshal(configBytes, &appConfig); err != nil {
		log.Fatalf("Failed to unmarshal config file %s: %v", configPath, err)
	}

	clients := make(map[string]*plugin.Client)
	greeters := make(map[string]shared.Greeter) // Store dispensed greeters by their config name

	// 2. Iterate through enabled plugins and load them
	for _, pConfig := range appConfig.EnabledPlugins {
		pluginAbsolutePath := filepath.Join(appConfig.PluginsDirectory, pConfig.Path)
		logger.Info("Loading plugin",
			"config_name", pConfig.Name,
			"path", pluginAbsolutePath,
			"language_code", pConfig.LanguageCode,
		)

		// Create a new plugin client for each plugin.
		client := plugin.NewClient(&plugin.ClientConfig{
			HandshakeConfig: shared.HandshakeConfig,
			Plugins:         shared.PluginMap, // Use the shared plugin map
			Cmd:             exec.Command(pluginAbsolutePath),
			Logger:          logger,
			AllowedProtocols: []plugin.Protocol{plugin.ProtocolGRPC},
		})

		clients[pConfig.Name] = client
		defer client.Kill() // Ensure the plugin process is killed on exit

		// Connect via RPC
		rpcClient, err := client.Client()
		if err != nil {
			logger.Error("Error connecting to plugin", "name", pConfig.Name, "err", err)
			continue
		}

		// Request the plugin by its "greeter" key defined in shared.PluginMap
		raw, err := rpcClient.Dispense("greeter")
		if err != nil {
			logger.Error("Error dispensing plugin", "name", pConfig.Name, "err", err)
			continue
		}

		// Assert that the dispensed raw plugin implements the Greeter interface
		greeter, ok := raw.(shared.Greeter)
		if !ok {
			logger.Error("Dispensed plugin did not implement Greeter interface", "name", pConfig.Name)
			continue
		}
		greeters[pConfig.Name] = greeter // Store the actual greeter instance
	}

	// 3. Use the loaded plugins based on their language code from config
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	fmt.Println("\n--- Using Loaded Plugins ---")
	for name, greeter := range greeters {
		// Find the corresponding config to get the language_code
		var langCode string
		for _, pConfig := range appConfig.EnabledPlugins {
			if pConfig.Name == name {
				langCode = pConfig.LanguageCode
				break
			}
		}

		if langCode == "" {
			logger.Warn("Language code not found for plugin", "name", name)
			continue
		}

		greeting, err := greeter.Greet(ctx, "World", langCode)
		if err != nil {
			fmt.Printf("Error from %s plugin (lang: %s): %v\n", name, langCode, err)
		} else {
			fmt.Printf("%s plugin (lang: %s) says: %s\n", name, langCode, greeting)
		}
	}

	// Example of trying an unsupported language on the English greeter
	fmt.Println("\n--- Testing Unsupported Language ---")
	if enGreeter, ok := greeters["english-greeter"]; ok {
		greeting, err := enGreeter.Greet(ctx, "Charlie", "de") // Try German on English plugin
		if err != nil {
			fmt.Printf("English plugin error (expected, for 'de'): %v\n", err)
		} else {
			fmt.Printf("English plugin says (unexpected): %s\n", greeting)
		}
	}
}
```

### Step 3: Plugin `main.go`s (No Change)

The plugin `main.go` files (`greeter-en/main.go`, `greeter-es/main.go`, `greeter-fr/main.go`) remain exactly the same. They just need to implement the `shared.Greeter` interface and call `plugin.Serve()`.

### Step 4: Build and Run

1.  **Ensure `go.mod` files are tidy:**

    ```bash
    cd greeter-plugin-system
    go mod tidy
    cd plugins/greeter-en
    go mod tidy
    cd ../greeter-es
    go mod tidy
    cd ../greeter-fr # For the new plugin
    go mod tidy
    cd ../.. # Back to greeter-plugin-system root
    ```

2.  **Generate Protobuf Code (if any changes to `greeter.proto`):**

    ```bash
    protoc --go_out=./proto/gen --go_opt=paths=source_relative \
           --go-grpc_out=./proto/gen --go-grpc_opt=paths=source_relative \
           proto/greeter.proto
    ```

3.  **Build All Plugins:**

    ```bash
    go build -o plugins/greeter-en/greeter-en ./plugins/greeter-en
    go build -o plugins/greeter-es/greeter-es ./plugins/greeter-es
    go build -o plugins/greeter-fr/greeter-fr ./plugins/greeter-fr # Build the new plugin
    ```

4.  **Build the Main Host Application:**

    ```bash
    go build -o mygreeterapp main.go
    ```

5.  **Run the Host Application (from the `greeter-plugin-system` directory):**

    ```bash
    ./mygreeterapp
    ```

    or

    ```bash
    go run main.go
    ```

### How to Install a _New_ Plugin (without recompiling the main app)

Now, with this setup, adding a _new_ plugin (e.g., a "German Greeter") involves these steps:

1.  **Create the new Plugin Project (`greeter-de`):**

    - Create `greeter-plugin-system/plugins/greeter-de/main.go` and `go.mod` (similar to the French one, but with "Guten Tag").

2.  **Build the New Plugin Binary:**

    ```bash
    cd greeter-plugin-system/plugins/greeter-de
    go mod tidy
    go build -o greeter-de ./main.go
    ```

    This creates `greeter-de` executable in `plugins/greeter-de/`.

3.  **Update the `config.json` file:**

    - Open `greeter-plugin-system/config.json`.

    - Add a new entry to the `enabled_plugins` array:

      ```json
      // config.json
      {
        "plugins_directory": "./plugins",
        "enabled_plugins": [
          // ... existing plugins ...
          {
            "name": "german-greeter",
            "path": "greeter-de/greeter-de",
            "language_code": "de"
          }
        ]
      }
      ```

4.  **Restart the Main Host Application:**

    - Simply stop the currently running `mygreeterapp` (or `go run main.go`) and start it again.

**Crucially, you did _not_ need to recompile `mygreeterapp` (the main host application) at any point in step 4\!**

### Further Enhancements for a Robust System:

- **Plugin Discovery by Directory Scanning:** Instead of explicitly listing plugins in `config.json`, you could have the host scan `plugins_directory` for executables that match a certain pattern (e.g., `greeter-*.exe` or just any executable in a subdirectory).
  - This would make `config.json` even simpler (just `plugins_directory`) or allow more flexible per-plugin configuration within separate files.
  - You'd use `os.ReadDir` or `filepath.Glob` in `main.go` to find plugin binaries.
- **Plugin Metadata:** Plugins could export a small metadata file (e.g., `plugin.json` next to their binary) containing their name, version, supported languages, etc., which the host could read _before_ launching them.
- **Dynamic Reloading:** For truly "hot" updates, you'd need a mechanism to:
  - Detect changes in the `plugins` directory or `config.json`.
  - Gracefully shut down existing plugin processes.
  - Launch new versions of plugins.
  - This is significantly more complex and often involves a manager layer that handles lifecycle and ensures no requests are dropped during the transition. HashiCorp Consul/Nomad/Vault often handle this at a higher orchestration level.
- **Error Reporting and Health Checks:** Implement health checks for plugins and robust error reporting if a plugin crashes or fails to respond.
- **Security:** If plugins come from untrusted sources, consider additional sandboxing (e.g., containerization like Docker) beyond what `go-plugin` provides for process isolation.

By externalizing the plugin list into a configuration file and allowing the host to dynamically load them based on that, you achieve the true purpose of a plugin system: extensibility without modifying (or recompiling) the core application.
