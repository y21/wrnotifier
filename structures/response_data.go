package structures

import "time"

// Response represents the body of http://tt.chadsoft.co.uk/index.json
type Response struct {
	Links struct {
		Self struct {
			Href string `json:"href"`
		} `json:"self"`
		OriginalTracks struct {
			Href string `json:"href"`
		} `json:"original-tracks"`
		CtgpTracks struct {
			Href string `json:"href"`
		} `json:"ctgp-tracks"`
		OtherTracks struct {
			Href string `json:"href"`
		} `json:"other-tracks"`
		AllTracks struct {
			Href string `json:"href"`
		} `json:"all-tracks"`
		OriginalTracks200Cc struct {
			Href string `json:"href"`
		} `json:"original-tracks-200cc"`
		CtgpTracks200Cc struct {
			Href string `json:"href"`
		} `json:"ctgp-tracks-200cc"`
		OtherTracks200Cc struct {
			Href string `json:"href"`
		} `json:"other-tracks-200cc"`
		AllTracks200Cc struct {
			Href string `json:"href"`
		} `json:"all-tracks-200cc"`
		Players struct {
			Href string `json:"href"`
		} `json:"players"`
	} `json:"_links"`
	UniquePlayers    int `json:"uniquePlayers"`
	LeaderboardCount int `json:"leaderboardCount"`
	GhostCount       int `json:"ghostCount"`
	RecentRecords    []struct {
		Links struct {
			Item struct {
				Href string `json:"href"`
			} `json:"item"`
			Player struct {
				Href string `json:"href"`
			} `json:"player"`
			Leaderboard struct {
				Href string `json:"href"`
			} `json:"leaderboard"`
		} `json:"_links"`
		Href             string    `json:"href"`
		Country          int       `json:"country,omitempty"`
		Region           int       `json:"region,omitempty"`
		Continent        int       `json:"continent,omitempty"`
		Player           string    `json:"player"`
		TrackID          string    `json:"trackId"`
		TrackName        string    `json:"trackName"`
		TrackVersion     string    `json:"trackVersion,omitempty"`
		Two00Cc          bool      `json:"200cc"`
		CategoryID       int       `json:"categoryId,omitempty"`
		DefaultTrack     bool      `json:"defaultTrack"`
		FinishTime       string    `json:"finishTime"`
		FinishTimeSimple string    `json:"finishTimeSimple"`
		BestSplit        string    `json:"bestSplit"`
		BestSplitSimple  string    `json:"bestSplitSimple"`
		Hash             string    `json:"hash"`
		VehicleID        int       `json:"vehicleId"`
		DriverID         int       `json:"driverId"`
		DateSet          time.Time `json:"dateSet"`
		IsTie            bool      `json:"isTie"`
	} `json:"recentRecords"`
}
