package uuid

import (
	"strings"

	"github.com/google/uuid"
	u "github.com/google/uuid"
)

func GenerateVersion4(n int, dash, upper bool) ([]string, error) {
	ret := make([]string, n)
	for i := 0; i < n; i++ {
		id, err := u.NewRandom()
		if err != nil {
			return nil, err
		}
		ret[i] = format(id.String(), dash, upper)
	}
	return ret, nil
}

func format(id string, dash, upper bool) string {
	if !dash {
		id = strings.ReplaceAll(id, "-", "")
	}
	if upper {
		id = strings.ToUpper(id)
	}
	return id
}

func FormatIds(ids []string, dash, upper bool) []string {
	ret := make([]string, 0, len(ids))
	for _, id := range ids {
		parsed, err := uuid.Parse(id)
		if err != nil {
			continue // :(
		}
		ret = append(ret, format(parsed.String(), dash, upper))
	}
	return ret
}
