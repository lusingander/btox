//go:generate go run .
package main

import (
	"bytes"
	"encoding/json"
	"go/format"
	"io"
	"net/http"
	"os"
	"strings"
	"text/template"
)

const output = "../colors_gen.go"

const t = `// Code generated by ./gen/gen.go; DO NOT EDIT.
package color

var Cols = []Color{
{{- range . }}
	{
		ID: {{ .ColorId }},
		Hex: "{{ .HexString | ToUpper }}",
	},
{{- end }}
}
`

type color struct {
	ColorId   int
	HexString string
	Rgb       struct {
		R int
		G int
		B int
	}
	Hsl struct {
		H float64
		S int
		L int
	}
	Name string
}

var funcMap = template.FuncMap{
	"ToUpper": strings.ToUpper,
}

func generate(cols []*color) error {
	tpl, err := template.New("").Funcs(funcMap).Parse(t)
	if err != nil {
		return err
	}

	var buf bytes.Buffer
	if err := tpl.Execute(&buf, cols); err != nil {
		return err
	}

	src, err := format.Source(buf.Bytes())
	if err != nil {
		return err
	}

	return os.WriteFile(output, src, 0666)
}

func run() error {
	res, err := http.Get("https://www.ditig.com/downloads/256-colors.json")
	if err != nil {
		return err
	}
	defer res.Body.Close()

	body, err := io.ReadAll(res.Body)
	if err != nil {
		return err
	}

	cols := make([]*color, 0)
	if err := json.Unmarshal(body, &cols); err != nil {
		return err
	}

	return generate(cols)
}

func main() {
	if err := run(); err != nil {
		panic(err)
	}
}