package main

import "C"

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"log"
	// "os"

	"github.com/open-policy-agent/opa/rego"
)

//export handleInput
func handleInput(policy string, message string) string {

	message_map := make(map[string]string)

	// m_u_byte := byteDelZero(mr_usr)
	fmt.Println("In Golang")
	fmt.Println(policy)

	err := json.Unmarshal([]byte(message), &message_map)
	if err != nil {
		log.Fatal(err)
		panic(err)
	}

	ctx := context.Background()

	// Construct a Rego object that can be prepared or evaluated.
	r := rego.New(
		rego.Query("data.demo.allow"),
		// rego.Load([]string{"src/policy/test.rego"}, nil))
		rego.Module("example.rego", policy),
	)

	// Create a prepared query that can be evaluated.
	query, err := r.PrepareForEval(ctx)
	if err != nil {
		log.Fatal(err)
		panic(err)
	}

	rs, err := query.Eval(ctx, rego.EvalInput(message_map))
	if err != nil {
		log.Fatal(err)
		panic(err)
	}
	// fmt.Println(rs[0].Expressions[0])
	// Do something with the result.
	if fmt.Sprint(rs[0].Expressions[0]) == "true" {
		return "true"
	}
	return "false"
}

func byteDelZero(data_str string) []byte {
	data_byte := []byte(data_str)
	index := bytes.IndexByte(data_byte, 0)
	data_byte = data_byte[:index]
	return data_byte
}

func main() {}
