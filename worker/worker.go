package worker

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
	"time"

	"github.com/y21/wrnotifier-go/structures"
)

// API is used to fetch recent world records
const API string = "http://tt.chadsoft.co.uk/index.json"

func work(webhooks *[]structures.Webhook) {
	// TODO
}

func updateLocalCopy(webhooks *[]structures.Webhook, sync *bool) {
	if !*sync {
		fmt.Println("Updating file...")
		bytes, err := json.MarshalIndent(*webhooks, "", " ")
		if err != nil {
			fmt.Printf("An error occurred while trying to decode JSON: %s", err)
			os.Exit(1)
		}
		err = ioutil.WriteFile("webhooks.json", bytes, 0644)
		if err != nil {
			fmt.Printf("An error occurred while trying to write to file: %s", err)
			os.Exit(1)
		}
		*sync = true
	}
}

// Loop is used to concurrently fetch data
func Loop(webhooks *[]structures.Webhook, sync *bool) {
	for {
		time.Sleep(3 * time.Second)
		go work(webhooks)
		go updateLocalCopy(webhooks, sync)
	}
}
