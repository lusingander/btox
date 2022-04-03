package hash

import (
	"fmt"
	"os/exec"
	"regexp"
	"strings"
	"testing"
)

func FuzzMD5(f *testing.F) {
	f.Fuzz(func(t *testing.T, s string) {
		want, err := sumByCommand(s, "md5")
		if err != nil {
			t.Fatal(err)
		}
		got := MD5(s)
		if got != want {
			t.Errorf("s = %s, got = %s, want = %s", s, got, want)
		}
	})
}

func FuzzSHA1(f *testing.F) {
	f.Fuzz(func(t *testing.T, s string) {
		want, err := sumByCommand(s, "shasum", "-a", "1")
		if err != nil {
			t.Fatal(err)
		}
		got := SHA1(s)
		if got != want {
			t.Errorf("s = %s, got = %s, want = %s", s, got, want)
		}
	})
}

func FuzzSHA224(f *testing.F) {
	f.Fuzz(func(t *testing.T, s string) {
		want, err := sumByCommand(s, "shasum", "-a", "224")
		if err != nil {
			t.Fatal(err)
		}
		got := SHA224(s)
		if got != want {
			t.Errorf("s = %s, got = %s, want = %s", s, got, want)
		}
	})
}

func FuzzSHA256(f *testing.F) {
	f.Fuzz(func(t *testing.T, s string) {
		want, err := sumByCommand(s, "shasum", "-a", "256")
		if err != nil {
			t.Fatal(err)
		}
		got := SHA256(s)
		if got != want {
			t.Errorf("s = %s, got = %s, want = %s", s, got, want)
		}
	})
}

func FuzzSHA384(f *testing.F) {
	f.Fuzz(func(t *testing.T, s string) {
		want, err := sumByCommand(s, "shasum", "-a", "384")
		if err != nil {
			t.Fatal(err)
		}
		got := SHA384(s)
		if got != want {
			t.Errorf("s = %s, got = %s, want = %s", s, got, want)
		}
	})
}

func FuzzSHA512_224(f *testing.F) {
	f.Fuzz(func(t *testing.T, s string) {
		want, err := sumByCommand(s, "shasum", "-a", "512224")
		if err != nil {
			t.Fatal(err)
		}
		got := SHA512_224(s)
		if got != want {
			t.Errorf("s = %s, got = %s, want = %s", s, got, want)
		}
	})
}

func FuzzSHA512_256(f *testing.F) {
	f.Fuzz(func(t *testing.T, s string) {
		want, err := sumByCommand(s, "shasum", "-a", "512256")
		if err != nil {
			t.Fatal(err)
		}
		got := SHA512_256(s)
		if got != want {
			t.Errorf("s = %s, got = %s, want = %s", s, got, want)
		}
	})
}

func FuzzSHA512_512(f *testing.F) {
	f.Fuzz(func(t *testing.T, s string) {
		want, err := sumByCommand(s, "shasum", "-a", "512")
		if err != nil {
			t.Fatal(err)
		}
		got := SHA512(s)
		if got != want {
			t.Errorf("s = %s, got = %s, want = %s", s, got, want)
		}
	})
}

var r = regexp.MustCompile("(^[0-9a-f]+).*")

func sumByCommand(s, alg string, args ...string) (string, error) {
	cmd := exec.Command(alg, args...)
	cmd.Stdin = strings.NewReader(s)
	out, err := cmd.Output()
	if err != nil {
		return "", err
	}
	ms := r.FindSubmatch(out)
	if len(ms) <= 1 {
		return "", fmt.Errorf("does not match pattern: out = %s", string(out))
	}
	return string(ms[1]), nil
}
