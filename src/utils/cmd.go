package utils

import (
	"fmt"
	"os/exec"
	"strings"
)

// Helper function to execute a command and capture output (optional, for more control)
func RunCommand(dir string, name string, args ...string) (string, error) {
	cmd := exec.Command(name, args...)
	cmd.Dir = dir
	var stdout, stderr strings.Builder
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err := cmd.Run()
	if err != nil {
		return "", fmt.Errorf("command '%s %s' failed in %s: %w\nStdout: %s\nStderr: %s",
			name, strings.Join(args, " "), dir, err, stdout.String(), stderr.String())
	}
	return stdout.String(), nil
}
