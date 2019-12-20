package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"

	"github.com/gorilla/mux"
	"github.com/y21/wrnotifier-go/api"
	"github.com/y21/wrnotifier-go/middleware"
	"github.com/y21/wrnotifier-go/structures"
	"github.com/y21/wrnotifier-go/worker"
)

const version string = "1.1.0"

var authKey string
var webhooksFilePath string
var webhooks []structures.Webhook
var sync bool = true

func handleError(err error) {
	if err != nil {
		fmt.Printf("An error occurred: %s", err)
		os.Exit(1)
	}
}

func main() {
	// Parse flags
	flag.StringVar(&authKey, "auth", "", "The authorization key")
	flag.StringVar(&webhooksFilePath, "file", "webhooks.json", "File path to JSON file where webhooks are stored")
	flag.Parse()

	// Validate authKey flag
	if authKey == "" {
		fmt.Println("No authorization key provided. Please start process with auth flag as follows: -auth <key>")
		os.Exit(1)
	}

	// Read file
	jsonFile, err := os.Open(webhooksFilePath)
	handleError(err)
	defer jsonFile.Close()

	// Unmarshal JSON and store it in webhooks var
	bytes, err := ioutil.ReadAll(jsonFile)
	handleError(err)
	json.Unmarshal(bytes, &webhooks)

	// Start worker
	go worker.Loop(&webhooks, &sync)

	// Start webserver
	router := mux.NewRouter()

	router.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "wr notifier version %s", version)
	}).Methods("GET")
	router.HandleFunc("/webhooks", middleware.Authorize(api.Fetch, &webhooks, authKey, &sync)).Methods("GET")
	router.HandleFunc("/register/{id}/{token}", middleware.Authorize(api.Register, &webhooks, authKey, &sync)).Methods("POST")
	router.HandleFunc("/unregister/{id}/{token}", middleware.Authorize(api.Unregister, &webhooks, authKey, &sync)).Methods("POST")

	fmt.Printf("Webserver started, auth key: %s\n", authKey)
	http.ListenAndServe(":3000", router)
}
