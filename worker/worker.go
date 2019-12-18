package worker

import (
	"time"

	"github.com/y21/wrnotifier-go/structures"
)

func work(webhooks *[]structures.Webhook) {

}

// Loop is used to concurrently fetch data
func Loop(webhooks *[]structures.Webhook) {
	for {
		time.Sleep(3 * time.Second)
		go work(webhooks)
	}
}
