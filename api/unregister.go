package api

import (
	"fmt"
	"net/http"

	"github.com/gorilla/mux"
	"github.com/y21/wrnotifier-go/structures"
)

func stateToInt(state bool) int {
	if state {
		return 200
	}
	return 404
}

// Unregister is used to delete a webhook
func Unregister(w http.ResponseWriter, r *http.Request, webhooks *[]structures.Webhook) {
	params := mux.Vars(r)
	found := false

	for i, el := range *webhooks {
		if el.ID == params["id"] && el.Token == params["token"] {
			(*webhooks)[len(*webhooks)-1], (*webhooks)[i] = (*webhooks)[i], (*webhooks)[len(*webhooks)-1]
			*webhooks = (*webhooks)[:len(*webhooks)-1]
			found = true
		}
	}

	w.Header().Set("Content-Type", "application/json")
	fmt.Fprintf(w, "{\"status\": %d}", stateToInt(found))
}
