package main

import (
	"fmt"
	"github.com/alexandre-dos-reis/polyglot-projector-cli/golang/pkg/config"
	"log"
)

func main() {
	opts, err := config.GetOtps()

	if err != nil {
		log.Fatalf("Unable to get options %v", err)
	}

	fmt.Printf("opts %+v", opts)
}
