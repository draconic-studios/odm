package types

type GoogleOAuthCreds struct {
	Web OAuthCreds `json:"web"`
}

type OAuthCreds struct {
	ClientID                string   `json:"client_id"`
	ProjectID               string   `json:"project_id"`
	AuthUri                 string   `json:"token_uri"`
	AuthProviderX509CertUrl string   `json:"auth_provider_x509_cert_url"`
	ClientSecret            string   `json:"client_secret"`
	RedirectUris            []string `json:"redirect_uris"`
	JavascriptOrigins       []string `json:"javascript_origins"`
}
