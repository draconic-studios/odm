package envoy

import (
	"fmt"
	"odm/types"
)

// sortEnvoyHTTPFilters ensures that the 'envoy.filters.http.router' filter is
// always the last one in the http_filters list.
func SortEnvoyHTTPFilters(config *types.EnvoyConfig) error {

	// Assuming there's only one HttpConnectionManager in this structure based on the provided YAML
	// iterate through listeners/filter_chains more generally
	if len(config.StaticResources.Listeners) > 0 &&
		len(config.StaticResources.Listeners[0].FilterChains) > 0 &&
		len(config.StaticResources.Listeners[0].FilterChains[0].Filters) > 0 &&
		config.StaticResources.Listeners[0].FilterChains[0].Filters[0].Name == "envoy.filters.network.http_connection_manager" {

		httpManagerFilterConfig := config.StaticResources.Listeners[0].FilterChains[0].Filters[0].TypedConfig

		httpFilters := httpManagerFilterConfig.HTTPFilters
		if httpFilters == nil {
			// No HTTP filters to sort
			return fmt.Errorf("http filters not found")
		}

		// Separate router filter from others
		var routerFilter types.HTTPFilter
		otherFilters := []types.HTTPFilter{}
		foundRouter := false

		for _, filter := range httpFilters {
			if filter.Name == "envoy.filters.http.router" {
				routerFilter = filter
				foundRouter = true
			} else {
				otherFilters = append(otherFilters, filter)
			}
		}

		// If router was found, append it to the end of the other filters
		if foundRouter {
			otherFilters = append(otherFilters, routerFilter)
		} else {
			return fmt.Errorf("router not found in http filters")
		}

		// Update the HTTP filters in the struct
		config.StaticResources.Listeners[0].FilterChains[0].Filters[0].TypedConfig.HTTPFilters = otherFilters
	} else {
		return fmt.Errorf("could not find HttpConnectionManager filter")
	}

	return nil
}
