package middleware

import (
	"fmt"
	"net/http"

	"github.com/y21/wrnotifier-go/structures"
)

// Authorize is used as a middleware function to check whether request is api call and if user is authorized
func Authorize(callback func(http.ResponseWriter, *http.Request, *[]structures.Webhook, *bool), webhooks *[]structures.Webhook, authKey string, sync *bool) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		keys, status := r.URL.Query()["key"]
		if !status || len(keys[0]) < 1 {
			w.WriteHeader(http.StatusBadRequest)
			fmt.Fprintf(w, "{\"message\": \"%s\"}", "Invalid key provided")
		} else {
			key := keys[0]
			if key != authKey {
				w.WriteHeader(http.StatusForbidden)
				fmt.Fprintf(w, "{\"message\": \"%s\"", "Provided key is not correct")
			} else {
				callback(w, r, webhooks, sync)
			}
		}
	}
}
