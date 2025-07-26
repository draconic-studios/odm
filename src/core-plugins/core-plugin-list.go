package coreplugins

import odmplugin "github.com/hembrow-innovations/odm-plugin"

type CoreExecuter func(*odmplugin.ExecutionRequestBody) (string, error)

var CorePluginList map[string]CoreExecuter = map[string]CoreExecuter{
	"cmd": ExecuterCommand,
}
