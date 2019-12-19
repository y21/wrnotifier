package worker

import (
	"bytes"
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

var recordCache []structures.RecentRecord

func executeWebhook(id string, token string, message structures.Message) {
	data, err := json.Marshal(message)
	if err != nil {
		fmt.Printf("Error occurred while executing webhook: %s", err)
		return
	}
	buffer := bytes.NewBuffer(data)
	fmt.Println(buffer)
	resp, err := http.Post("https://discordapp.com/api/webhooks/"+id+"/"+token, "application/json", buffer)
	if err != nil {
		fmt.Printf("Error occurred while executing webhook: %s", err)
		return
	}
	if resp.StatusCode == 401 || resp.StatusCode == 404 {
		fmt.Println("webhook error")
		// TODO: remove webhook
	}
}

func work(webhooks *[]structures.Webhook) {
	// Send request and process response
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
	err = json.Unmarshal(body[3:], &data) // Slice 3 because of Byte Order Mark
	if err != nil {
		fmt.Printf("Error while parsing JSON: %s", err)
		return
	}

	// First iteration => No cache
	if len(recordCache) == 0 {
		recordCache = data.RecentRecords
		return
	}

	if data.RecentRecords[0].Hash != recordCache[0].Hash || true { // New WR achieved
		for _, webhook := range *webhooks {
			if webhook.EngineClass150 && data.RecentRecords[0].Two00Cc {
				continue
			}

			var engineClass string
			if data.RecentRecords[0].Two00Cc {
				engineClass = "200cc"
			} else {
				engineClass = "150cc"
			}
			var fields []structures.EmbedField
			fields = append(fields, structures.EmbedField{
				Name:  "Track",
				Value: data.RecentRecords[0].TrackName + " " + data.RecentRecords[0].TrackVersion,
			})
			fields = append(fields, structures.EmbedField{
				Name:  "Engine class",
				Value: engineClass,
			})

			executeWebhook(webhook.ID, webhook.Token, structures.Message{
				Embeds: []structures.Embed{
					{
						Color:  0xae60,
						Title:  "New World Record",
						Fields: fields,
					},
				},
			})
			recordCache = data.RecentRecords
		}
	}
}

func updateLocalCopy(webhooks *[]structures.Webhook, sync *bool) {
	if !*sync {
		fmt.Println("Updating file...")
		bytes, err := json.MarshalIndent(*webhooks, "", "    ")
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
