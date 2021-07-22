package main

import "C"

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"log"
	"os"

	"github.com/open-policy-agent/opa/rego"
)

//export handleInput
func handleInput(mr_usr string) bool {

	m_u_map := make(map[string]string)

	// m_u_byte := byteDelZero(mr_usr)

	err := json.Unmarshal([]byte(mr_usr), &m_u_map)
	if err != nil {
		log.Fatal(err)
		panic(err)
	}

	ctx := context.Background()

	// Construct a Rego object that can be prepared or evaluated.
	r := rego.New(
		rego.Query("data.demo.allow"),
		rego.Load([]string{"rules.rego"}, nil))

	// Create a prepared query that can be evaluated.
	query, err := r.PrepareForEval(ctx)
	if err != nil {
		log.Fatal(err)
		panic(err)
	}

	rs, err := query.Eval(ctx, rego.EvalInput(m_u_map))
	if err != nil {
		log.Fatal(err)
		panic(err)
	}
	// fmt.Println(rs[0].Expressions[0])
	// Do something with the result.
	if fmt.Sprint(rs[0].Expressions[0]) == "true" {
		return true
	}
	return false

}

// generate default rego file
//export handleReference
func handleReference(mr_ref string) {
	m_r_map := make(map[string]string)

	// m_r_byte := byteDelZero(mr_ref)
  fmt.Println(mr_ref)
	err := json.Unmarshal([]byte(mr_ref), &m_r_map)
	if err != nil {
		panic(err)
	}

	// generate
	policy := "package demo\n\n" +
		"MRENCLAVE = \"" + m_r_map["MRENCLAVE"] + "\"\n" +
		"MRSIGNER = \"" + m_r_map["MRSIGNER"] + "\"\n\n" +
		"default allow = false\n\n" +
		"allow = true {\n" +
		"	MRENCLAVE == input.MRENCLAVE\n" +
		"	MRSIGNER == input.MRSIGNER\n" +
		"}"

	fileName := "rules.rego"
	dstFile, err := os.Create(fileName)
	if err != nil {
		fmt.Println(err.Error())
		return
	}

	defer dstFile.Close()
	dstFile.WriteString(policy + "\n")
}

func byteDelZero(data_str string) []byte {
	data_byte := []byte(data_str)
	index := bytes.IndexByte(data_byte, 0)
	data_byte = data_byte[:index]
	return data_byte
}

//export Hello
func Hello(){
  fmt.Println("Hello")
}

func main() {}
