package main

import (
	"aap/repl"
	"fmt"
	"os"
	"os/user"
)

func main() {
	user, err := user.Current()
	if err != nil {
		panic(err)
	}
	fmt.Printf("Hello %s! Welcome to Aab lang!\n", user.Username)
	fmt.Println("Feel free to start typing stuff")
	repl.Start(os.Stdin, os.Stdout)
}
