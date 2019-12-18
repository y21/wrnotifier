package api

import (
	"fmt"
	"net/http"

	"github.com/gorilla/mux"

	"github.com/y21/wrnotifier-go/structures"
)

// Register is used to register a new webhook
func Register(w http.ResponseWriter, r *http.Request, webhooks *[]structures.Webhook, sync *bool) {
	params := mux.Vars(r)
	*webhooks = append(*webhooks, structures.Webhook{
		EngineClass150: false,
		ID:             params["id"],
		Server:         "", // todo: register server ID
		Token:          params["token"],
	})
	*sync = false

	w.Header().Set("Content-Type", "application/json")
	fmt.Fprintf(w, "{\"status\":%d}", 200)
}
