package actions

import (
	"odm/types"
	"odm/utils"
)

type Action func(*utils.Command, *types.Orchestrator) (string, error)

var ActionList map[string]Action = map[string]Action{
	"add":        Add,
	"remove":     Remove,
	"build-docs": BuildDocs,
}
