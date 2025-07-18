package terminal

import (
	"bytes"
	"fmt"
	"log"
	"os/exec"
)

func RunCommand(name string, arg ...string) (string, error) {
	cmd := exec.Command(name, arg...)
	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	log.Printf("Running command: %s %v\n", name, arg)
	err := cmd.Run()
	if err != nil {
		return "", fmt.Errorf("command failed: %s %s, error: %w", stdout.String(), stderr.String(), err)
	}
	return stdout.String(), nil
}
