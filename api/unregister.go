package api

import (
	"net/http"

	"github.com/y21/wrnotifier-go/structures"
)

// Unregister is used to delete a webhook
func Unregister(w http.ResponseWriter, r *http.Request, webhooks *[]structures.Webhook) {

}
