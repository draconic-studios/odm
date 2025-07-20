package creds

import (
	"fmt"
	"odm/utils"
)

func FindCredsFile(configPath string) ([]byte, error) {
	contents, err := utils.ReadFolderContents(configPath)
	if err != nil {
		return nil, err
	}

	for _, item := range *contents {
		fmt.Println(item)
	}

	return nil, nil

}
