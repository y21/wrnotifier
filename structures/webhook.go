package structures

// Webhook represents a Discord webhook
type Webhook struct {
	ID             string `json:"id"`
	Token          string `json:"token"`
	Server         string `json:"server"`
	EngineClass150 bool   `json:"150ccOnly"`
}
