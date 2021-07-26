package main

import "C"

import (
	// "bytes"
	"context"
	"encoding/json"
	"fmt"
	"log"
	// "os"

	"github.com/open-policy-agent/opa/rego"
)

//export makeDecisionGo
func makeDecisionGo(policy string, message string) *C.char {

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
		rego.Query("input;data.demo"),
		// rego.Load([]string{"src/policy/test.rego"}, nil))
		rego.Module("demo.rego", policy),
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
	// fmt.Println(rs[0].Expressions[0].Value.(map[string]interface{})["mrEnclave"])
	// fmt.Println(rs[0].Expressions[1])

	input := rs[0].Expressions[0].Value.(map[string]interface{})
	dataDemo := rs[0].Expressions[1].Value.(map[string]interface{})

	// fmt.Println(input)
	// fmt.Println(dataDemo)


	parseInfo := make(map[string]interface{})

	var count = 1
	// var str string
	for k, v := range input {
		str := "inputValue" + fmt.Sprint(count)
		value := [2]interface{}{v, dataDemo[k]}
		parseInfo[str] = value
		count = count + 1
	}

	decisionMap := make(map[string]interface{})
	decisionMap["parseInfo"] = parseInfo
	decisionMap["allow"] = dataDemo["allow"]

	// fmt.Println(decisionMap)

	decision, err := json.Marshal(decisionMap)
	if err != nil {
		fmt.Println("json.Marshal failed: ", err)
		// return ""
	}

	// fmt.Println(string(decision))
	// fmt.Println(decision)
	// Decision:="hhhhhhh"
	return C.CString(string(decision))
}

func main() {}
