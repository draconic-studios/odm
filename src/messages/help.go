package messages

// string explaining the global level usage of the CLI
var GlobalUsage string = `Usage: odm <command> [arguments]

Commands:
  build    			Build system
  run     			Run system
  clean				factory reset
  docker-build  		Build Docker image
  docker-run      		Run a Docker container
  help     			Display help information

Use "odm help <command>" for more information about a command.
`

var BuildUsage string = `
Usage: odm build [arguments]

Arguments:

--project     -p: Path to the root of the project
--output      -o: Output folder to be created inside the project folder
--services    -s: Name of the folder containing the services/application to be built
--config      -c: Name of the folder containing the configuration files (nested inside build type folder)
--build-type  -t: Type of dev being performed ("dev", "prod")
--exclude     -e: list of services to be excluded (seperated by a ",")

exclude functionality is currently in construction
`

var RunUsage string = `
Usage: odm run [arguments]

Arguments:

--project           -p: Path to the root of the project
--system            -s: Name of the folder of the built system
--docker-compose    -c: Name of the docker-compose file
--attach            -a: Attach to be running logs 
--exclude           -e: list of services to be excluded (seperated by a ",")

exclude functionality is currently in construction
`
