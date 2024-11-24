package main

import (
	"bytes"
	"io/ioutil"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
)

type RequestData struct {
	Title  string `json:"title"`
	Id int `json:"id"`
	Input string `json:"input"`
}

func main() {
	// Prepare the data to be sent in the POST request (a JSON object)
	data := RequestData{
		Title:  "my request",
		Id: 0,
		Input: "what is the date the day after tomorrow ?",
	}

	// Convert the data to JSON
	jsonData, err := json.Marshal(data)
	if err != nil {
		log.Fatalf("Error marshalling data: %v", err)
	}

	// Define the local URL (can be localhost or 127.0.0.1)
	url := "http://127.0.0.1:8000/free-gpt/v1/drakos"

	// Create a new POST request with the JSON payload
	req, err := http.NewRequest("POST", url, bytes.NewBuffer(jsonData))
	if err != nil {
		log.Fatalf("Error creating request: %v", err)
	}

	// Set the Content-Type header to application/json
	req.Header.Set("Content-Type", "application/json")

	// Send the HTTP request using the default client
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		log.Fatalf("Error sending request: %v", err)
	}
	defer resp.Body.Close()

	// Print the response status code
	fmt.Printf("Response Status: %s\n", resp.Status)

	// Optionally, you can read and log the response body if needed
	// body, err := ioutil.ReadAll(resp.Body)
	// if err != nil {
	// 	log.Fatalf("Error reading response body: %v", err)
	// }
	// fmt.Println("Response Body:", string(body))

	// Read and print the response body
    	body, err := ioutil.ReadAll(resp.Body) // You can use io.ReadAll(resp.Body) in Go 1.16+
    	if err != nil {
    		log.Fatalf("Error reading response body: %v", err)
    	}

    	// Convert the body to a string and print it
    	fmt.Println("Response Body:", string(body))
}