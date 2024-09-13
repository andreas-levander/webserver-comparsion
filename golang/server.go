package main

import (
	"log"
	"net/http"
	"os"
	"slices"

	"github.com/gin-gonic/gin"
	"github.com/gocarina/gocsv"
)

type User struct { // Our example struct, you can use "-" to ignore a field
	Username string `csv:"Username"`
	Name     string `csv:"Name"`
	Phone    string `csv:"Phone Number"`
	Address  string `csv:"Address"`
	Email    string `csv:"Email"`
}

func main() {
	r := gin.Default()
	r.GET("/ping", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "pong",
		})
	})
	users := readCSV("./data/dummy_data.csv")

	r.GET("/user", func(c *gin.Context) {
		username := c.Query("username")

		i := slices.IndexFunc(users, func(u *User) bool {
			return u.Username == username
		})

		if i != -1 {
			c.JSON(http.StatusOK, users[i])
		} else {
			c.AbortWithStatus(http.StatusNotFound)
		}

	})
	r.Run() // listen and serve on 0.0.0.0:8080 (for windows "localhost:8080")
}

func readCSV(filename string) []*User {
	// os.Open() opens specific file in
	// read-only mode and this return
	// a pointer of type os.File
	file, err := os.Open(filename)

	// Checks for the error
	if err != nil {
		log.Fatal("Error while reading the file", err)
		panic(err)
	}

	// Closes the file
	defer file.Close()

	users := []*User{}

	if err := gocsv.UnmarshalFile(file, &users); err != nil { // Load clients from file
		panic(err)
	}

	return users
}
