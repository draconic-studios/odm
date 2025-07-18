package terminal

import "fmt"

type PrintText struct {
	// Reset to default
	Reset string

	TextColors       *TextColors
	TextAttributes   *TextAttributes
	BackgroundColors *BackgroundColors
}

type TextColors struct {
	// Text Colors (Foreground)
	Black   string
	Red     string
	Green   string
	Yellow  string
	Blue    string
	Magenta string
	Cyan    string
	White   string

	// Bright Text Colors (Foreground)
	BrightBlack   string
	BrightRed     string
	BrightGreen   string
	BrightYellow  string
	BrightBlue    string
	BrightMagenta string
	BrightCyan    string
	BrightWhite   string
}

type BackgroundColors struct {
	// Background Colors
	BgBlack   string
	BgRed     string
	BgGreen   string
	BgYellow  string
	BgBlue    string
	BgMagenta string
	BgCyan    string
	BgWhite   string

	// Bright Background Colors
	BgBrightBlack   string
	BgBrightRed     string
	BgBrightGreen   string
	BgBrightYellow  string
	BgBrightBlue    string
	BgBrightMagenta string
	BgBrightCyan    string
	BgBrightWhite   string
}
type TextAttributes struct {
	// Text Attributes
	Bold          string
	Dim           string
	Italic        string
	Underline     string
	Blink         string // May not work in all terminals
	Reverse       string // Swap foreground and background colors
	Hidden        string // Invisible text
	Strikethrough string // May not work in all terminals
}

func NewPrintText() *PrintText {
	return &PrintText{
		Reset: "\033[0m",
		TextColors: &TextColors{
			// Text Colors (Foreground)
			Black:   "\033[30m",
			Red:     "\033[31m",
			Green:   "\033[32m",
			Yellow:  "\033[33m",
			Blue:    "\033[34m",
			Magenta: "\033[35m",
			Cyan:    "\033[36m",
			White:   "\033[37m",

			// Bright Text Colors (Foreground)
			BrightBlack:   "\033[90m",
			BrightRed:     "\033[91m",
			BrightGreen:   "\033[92m",
			BrightYellow:  "\033[93m",
			BrightBlue:    "\033[94m",
			BrightMagenta: "\033[95m",
			BrightCyan:    "\033[96m",
			BrightWhite:   "\033[97m",
		},
		BackgroundColors: &BackgroundColors{
			// Background Colors
			BgBlack:   "\033[40m",
			BgRed:     "\033[41m",
			BgGreen:   "\033[42m",
			BgYellow:  "\033[43m",
			BgBlue:    "\033[44m",
			BgMagenta: "\033[45m",
			BgCyan:    "\033[46m",
			BgWhite:   "\033[47m",

			// Bright Background Colors
			BgBrightBlack:   "\033[100m",
			BgBrightRed:     "\033[101m",
			BgBrightGreen:   "\033[102m",
			BgBrightYellow:  "\033[103m",
			BgBrightBlue:    "\033[104m",
			BgBrightMagenta: "\033[105m",
			BgBrightCyan:    "\033[106m",
			BgBrightWhite:   "\033[107m",
		},
		TextAttributes: &TextAttributes{
			// Text Attributes
			Bold:          "\033[1m",
			Dim:           "\033[2m",
			Italic:        "\033[3m",
			Underline:     "\033[4m",
			Blink:         "\033[5m", // May not work in all terminals
			Reverse:       "\033[7m", // Swap foreground and background colors
			Hidden:        "\033[8m", // Invisible text
			Strikethrough: "\033[9m", // May not work in all terminals
		},
	}
}

// Helper function to apply format to text then print to terminal
func (p *PrintText) Print(text string, attributes ...string) {
	// Apply format
	for _, attr := range attributes {
		fmt.Print(attr)
	}
	// Apply text string
	fmt.Print(text)
	fmt.Println(p.Reset) // reset formatting
}
