package structures

// ExposableWebhook holds non-sensitive information that can be exposed
type ExposableWebhook struct {
	ID             string `json:"id"`
	Server         string `json:"server"`
	EngineClass150 bool   `json:"150ccOnly"`
}
