# ODM (Orchestrated Development Management)

- ODM (Omni-Directional Mobility) Gear

An `.odm.config.json` or `.odm.config.yaml` file defines the operations for this CLI tool.

---

# ODM CLI Tool

The ODM CLI tool helps orchestrate various development tasks within a project by defining and executing custom actions and managing project components like Git submodules and documentation.

---

## Configuration

The ODM CLI tool relies on a configuration file named `odm.config.json` or `odm.config.yaml` located in your project's root directory. This file defines the various actions the CLI can perform.

A basic `odm.config.yaml` example:

```yaml
# odm.config.yaml
name: MyProject
description: An example project using ODM
actions:
  my-custom-action:
    tasks:
      - executer: cmd
        options:
          command: echo "Hello from custom action!"
          path: .
projects:
  - name: my-submodule
    path: packages/my-submodule
    type: library
documentation:
  output: docs-output
```

---

## Installation

To install the ODM CLI tool, you'll need to have Go installed.

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/hembrow-innovations/odm.git
    cd odm
    ```
2.  **Build the CLI tool:**
    ```bash
    go build -o odm main.go
    ```
3.  **Add to your PATH (optional, but recommended):**
    Move the `odm` executable to a directory in your system's `PATH` (e.g., `/usr/local/bin` on macOS/Linux).
    ```bash
    mv odm /usr/local/bin/
    ```

---

## Usage

All commands start with `odm`. You must run `odm` commands from your project's root directory or specify the `--root-path` flag.

```bash
odm [command] [arguments] [flags]
```

---

## Commands

### `odm add <repo-url> <destination-path>`

Adds a Git repository as a submodule to your current project.

- `<repo-url>`: The URL of the Git repository to add (e.g., `https://github.com/user/repo.git`).
- `<destination-path>`: The local path within your project where the submodule will be added (e.g., `packages/my-library`).

**Flags:**

- `--root-path <path>`: Specifies the root path of your project if not the current working directory.

**Example:**

```bash
odm add https://github.com/hembrow-innovations/odm-plugin.git plugins/odm-plugin
```

---

### `odm remove <submodule-path>`

Removes a Git submodule from your project. This command handles deinitialization, removal from `.gitmodules`, Git cache, and the local file system.

- `<submodule-path>`: The local path of the submodule to remove (e.g., `plugins/odm-plugin`).

**Flags:**

- `--root-path <path>`: Specifies the root path of your project if not the current working directory.

**Example:**

```bash
odm remove plugins/odm-plugin
```

---

### `odm build-docs`

Builds documentation for your project based on the `documentation` section in your `odm.config.yaml` or `odm.config.json` file. It consolidates documentation from specified submodules and generates a static site.

**Flags:**

- `--root-path <path>`: Specifies the root path of your project if not the current working directory.

**Example:**

```bash
odm build-docs
```

This command will read the `documentation.output` path from your config file, copy documentation from your defined projects/libraries/tools (from their `docs` folder), and generate the necessary files for a static documentation site, including a `_sidebar.md` and an `index.html`.

---

### Custom Actions

You can define custom actions in your `odm.config.yaml` or `odm.config.json` file. These actions can leverage built-in core plugins or external ODM plugins.

To run a custom action defined in your config, use:

```bash
odm <action-name>
```

**Example (from the config above):**

```bash
odm my-custom-action
```

---

## Core Plugins

The ODM CLI tool includes several core plugins that can be used within your custom actions:

### `cmd`

Executes a shell command.

**Options:**

- `command` (string, required): The command string to execute.
- `path` (string, optional): The directory in which to execute the command. Defaults to the current working directory.

**Example Task:**

```yaml
# Inside an action's tasks
- executer: cmd
  options:
    command: 'npm install'
    path: 'frontend-app'
```

### `copy`

Copies files or folders.

**Options:**

- `source` (string, required): The source path (relative to `root-path`).
- `destination` (string, required): The destination path (relative to `root-path`).
- `type` (string, optional): "folder" for copying folders, "file" for copying files. Defaults to "file".

**Example Task:**

```yaml
# Inside an action's tasks
- executer: copy
  options:
    source: 'build/dist'
    destination: 'public/assets'
    type: 'folder'
```

### `env`

Generates an `.env` file from specified sources (env files, JSON, or YAML).

**Options:**

- `output` (string, required): The path where the `.env` file should be created (e.g., `.env` or `config/.env`).
- `items` (array of objects, required): A list of source files to extract environment variables from.
  - `filePath` (string, required): The path to the source file (relative to `root-path`).
  - `file` (string, required): The type of file: "env", "json", or "yaml".
  - `envKeys` (array of strings, optional, for `env` type): A whitelist of environment variable names to include from the `.env` file. If empty, all are included.
  - `keys` (array of objects, optional, for `json`/`yaml` types): A list of key mappings.
    - `key` (string, required): The dot-separated path to the value in the source JSON/YAML (e.g., `database.port`).
    - `envName` (string, required): The name of the environment variable to create (e.g., `DB_PORT`).

**Example Task:**

```yaml
# Inside an action's tasks
- executer: env
  options:
    output: '.env'
    items:
      - filePath: 'config/dev.json'
        file: 'json'
        keys:
          - key: 'api.url'
            envName: 'API_BASE_URL'
          - key: 'db.host'
            envName: 'DATABASE_HOST'
      - filePath: 'secrets/.env.prod'
        file: 'env'
        envKeys: ['STRIPE_KEY', 'AUTH_SECRET']
```

---
