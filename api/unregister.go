package api

import (
	"fmt"
	"net/http"

	"github.com/gorilla/mux"
	"github.com/y21/wrnotifier-go/structures"
	"github.com/y21/wrnotifier-go/utils"
)

func stateToInt(state bool) int {
	if state {
		return 200
	}
	return 404
}

// Unregister is used to delete a webhook
func Unregister(w http.ResponseWriter, r *http.Request, webhooks *[]structures.Webhook, sync *bool) {
	params := mux.Vars(r)
	found := false

	index := utils.GetWebhookIndex(webhooks, structures.Webhook{
		EngineClass150: false,
		ID: params["id"],
		Server: "",
		Token: params["token"],
	})

	if index > -1 {
		(*webhooks)[len(*webhooks)-1], (*webhooks)[index] = (*webhooks)[index], (*webhooks)[len(*webhooks)-1]
		*webhooks = (*webhooks)[:len(*webhooks)-1]
		found = true
		*sync = false
	}

	w.Header().Set("Content-Type", "application/json")
	fmt.Fprintf(w, "{\"status\": %d}", stateToInt(found))
}
