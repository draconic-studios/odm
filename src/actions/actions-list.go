package actions

import "odm/utils"

type Action func(*utils.Command) (string, error)

var ActionList map[string]Action = map[string]Action{
	"add":    Add,
	"remove": Remove,
}
