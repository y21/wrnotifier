package worker

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	"time"

	"github.com/y21/wrnotifier-go/structures"
)

// API is used to fetch recent world records
const API string = "http://tt.chadsoft.co.uk/index.json"

func executeWebhook(id string, token string) {

}

func work(webhooks *[]structures.Webhook) {
	resp, err := http.Get(API)
	if err != nil {
		fmt.Printf("Could not fetch website: %s", err)
		return
	}
	defer resp.Body.Close()
	var data structures.Response
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		fmt.Printf("Error while reading body: %s", err)
		return
	}
	err = json.Unmarshal(body[3:], &data)
	if err != nil {
		fmt.Printf("Error while parsing JSON: %s", err)
		return
	}
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
