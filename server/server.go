package main

import (
	"encoding/json"
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
const authKey string = ""

var webhooks []structures.Webhook
var sync bool = true

func handleError(err error) {
	if err != nil {
		fmt.Printf("An error occurred: %s", err)
		os.Exit(1)
	}
}

func main() {
	// Read file
	jsonFile, err := os.Open("webhooks.json")
	handleError(err)
	defer jsonFile.Close()

	// Unmarshal JSON and store it in webhooks var
	bytes, err := ioutil.ReadAll(jsonFile)
	handleError(err)
	json.Unmarshal(bytes, &webhooks)

	// Start worker
	go worker.Loop(&webhooks, &sync)

	// Start webserver
	fmt.Println("Starting webserver...")
	router := mux.NewRouter()

	router.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "wr notifier version %s", version)
	}).Methods("GET")
	router.HandleFunc("/webhooks", middleware.Authorize(api.Fetch, &webhooks, authKey, &sync)).Methods("GET")
	router.HandleFunc("/register/{id}/{token}", middleware.Authorize(api.Register, &webhooks, authKey, &sync)).Methods("POST")
	router.HandleFunc("/unregister/{id}/{token}", middleware.Authorize(api.Unregister, &webhooks, authKey, &sync)).Methods("POST")

	http.ListenAndServe(":3000", router)
}
