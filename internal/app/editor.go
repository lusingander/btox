package app

import (
	"os"
	"os/exec"

	"github.com/atotto/clipboard"
)

func EditInEditor(s string) (string, error) {
	tmp, err := os.CreateTemp(os.TempDir(), "btox-tmp-*")
	if err != nil {
		return "", err
	}
	defer os.Remove(tmp.Name())
	defer tmp.Close()

	if _, err := tmp.WriteString(s); err != nil {
		return "", err
	}

	if err := openEditor(tmp.Name()); err != nil {
		return "", err
	}

	bytes, err := os.ReadFile(tmp.Name())
	if err != nil {
		return "", err
	}

	return string(bytes), nil
}

func openEditor(filepath string) error {
	cmd := exec.Command("vi", filepath)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	return cmd.Run()
}

func CopyToClipboard(s string) error {
	return clipboard.WriteAll(s)
}
