package cli

func (o *OdmCli) GlobalUsage() string {
	return `Usage: odm <command> [arguments]

Commands:
  build    			Build system
  run     			Run system
  clean				factory reset
  docker-build  		Build Docker image
  docker-run      		Run a Docker container
  help     			Display help information

Use "odm help <command>" for more information about a command.
`
}
