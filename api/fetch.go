package api

import (
	"encoding/json"
	"net/http"

	"github.com/y21/wrnotifier-go/structures"
)

// Fetch is used to get ID and Server of webhook
func Fetch(w http.ResponseWriter, r *http.Request, webhooks *[]structures.Webhook, _ *bool) {
	data := make([]structures.ExposableWebhook, len(*webhooks))
	for i, el := range *webhooks {
		data[i] = structures.ExposableWebhook {
			EngineClass150: el.EngineClass150,
			ID: el.ID,
			Server: el.Server,
		}
	}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(data)
}
