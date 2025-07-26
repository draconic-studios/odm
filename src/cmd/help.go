package cmd

import (
	"fmt"
	"odm/types"
)

func (cli *Cli) Help(command *types.Command) (string, error) {

	return "", fmt.Errorf("unknown command")
}
