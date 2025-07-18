package cli

import (
	"fmt"
	"odm/terminal"
	"os"
)

type OdmCli struct {
	Print   *terminal.PrintText
	command *Command
}

func NewOdmCli() *OdmCli {
	return &OdmCli{
		Print: terminal.NewPrintText(),
	}
}

func (o *OdmCli) Execute() {

	// os.Args[0] is the program name, so we start parsing from os.Args[1]
	if len(os.Args) < 2 {
		o.Print.Print(o.GlobalUsage(), o.Print.TextColors.Blue)
		os.Exit(1)
	}

	o.command = o.ParseArgs(os.Args[1:])

	o.Print.Print(fmt.Sprintf("Name: %s", o.command.Name))
	fmt.Println(o.command.BoolFlags)
	fmt.Println(o.command.Flags)
	fmt.Println(o.command.Args)

}
