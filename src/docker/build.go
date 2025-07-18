package docker

// docker build -t <image_name>[:<tag>] <path_to_dockerfile>
type Docker struct {
}

func DockerBuild(tag string, fsContext string) (string, error) {

	cmdString := "docker build"

	return "", nil
}
