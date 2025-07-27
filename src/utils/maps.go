package utils

import "fmt"

// GetNestedValue searches for a value at a given key path within a map[string]any.
// It returns the value and a boolean indicating if the value was found.
func GetNestedValue(data any, keys []string) (any, bool) {
	if len(keys) == 0 {
		return data, true // Base case: all keys processed, return current data
	}

	if m, ok := data.(map[string]any); ok {
		if val, found := m[keys[0]]; found {
			return GetNestedValue(val, keys[1:]) // Recurse with the rest of the keys
		}
	} else if s, ok := data.([]any); ok {
		// If the current element is a slice, and the next "key" is a valid integer index
		if len(keys[0]) > 0 && keys[0][0] >= '0' && keys[0][0] <= '9' {
			index := 0
			fmt.Sscanf(keys[0], "%d", &index) // Safely parse the integer key
			if index >= 0 && index < len(s) {
				return GetNestedValue(s[index], keys[1:])
			}
		}
	}

	return nil, false // Key not found or type mismatch
}

// ConvertAnyToString attempts to type assert 'value' to a string.
// If it's already a string, it's returned directly.
// Otherwise, it tries to convert it to a string using fmt.Sprintf.
func ConvertAnyToString(value any) string {
	if s, ok := value.(string); ok {
		// Value is already a string, return it directly
		return s
	}

	// Value is not a string, so convert it.
	// fmt.Sprintf("%v", value) is a general way to get a string representation
	// of any Go value. It uses the default format for the type.
	return fmt.Sprintf("%v", value)
}
