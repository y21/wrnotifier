package structures

// Message represents a message sent as a webhook
type Message struct {
	Embeds []Embed `json:"embeds"`
}

// EmbedField represents an embed field
type EmbedField struct {
	Name  string `json:"name"`
	Value string `json:"value"`
}

// Embed represents an embed that is sent as webhook
type Embed struct {
	Title  string       `json:"title"`
	Color  int          `json:"color"`
	Fields []EmbedField `json:"fields"`
}
