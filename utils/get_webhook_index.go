package utils

import (
	"github.com/y21/wrnotifier-go/structures"
)

// GetExposableWebhookIndex gets the index of a specific exposable webhook in array
func GetExposableWebhookIndex(webhooks *[]structures.Webhook, webhook structures.ExposableWebhook) int {
	for i, el := range *webhooks {
		if el.ID == webhook.ID && el.Server == webhook.Server {
			return i
		}
	}
	return -1
}

// GetWebhookIndex gets the index of a specific webhook in array
func GetWebhookIndex(webhooks *[]structures.Webhook, webhook structures.Webhook) int {
	return GetExposableWebhookIndex(webhooks, structures.ExposableWebhook {
		EngineClass150: webhook.EngineClass150,
		ID: webhook.ID,
		Server: webhook.Server
	})
}
