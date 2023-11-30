package main

import (
	"fmt"
	"github.com/alexandre-dos-reis/polyglot-projector-cli/golang/pkg/lib"
	"log"
)

func main() {
	opts, err := lib.GetOtps()

	if err != nil {
		log.Fatalf("Unable to get options %v", err)
	}

	config, err := lib.NewConfig(opts)
	if err != nil {
		log.Fatalf("Unable to get config %v", err)
	}

	fmt.Printf("opts %+v", config)
}
