package structures

// ExposableWebhook holds non-sensitive information that can be exposed
type TargetWebhook struct {
	ID             string `json:"id"`
	Server         string `json:"server"`
}
