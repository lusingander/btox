package hash

import (
	"crypto/md5"
	"crypto/sha1"
	"crypto/sha256"
	"crypto/sha512"
	"fmt"
)

func MD5(s string) string {
	return sum(s, md5.Sum)
}

func SHA1(s string) string {
	return sum(s, sha1.Sum)
}

func SHA224(s string) string {
	return sum(s, sha256.Sum224)
}

func SHA256(s string) string {
	return sum(s, sha256.Sum256)
}

func SHA384(s string) string {
	return sum(s, sha512.Sum384)
}

func SHA512_224(s string) string {
	return sum(s, sha512.Sum512_224)
}

func SHA512_256(s string) string {
	return sum(s, sha512.Sum512_256)
}

func SHA512(s string) string {
	return sum(s, sha512.Sum512)
}

type Arr interface {
	[16]byte | [20]byte | [28]byte | [32]byte | [48]byte | [64]byte
}

func sum[T Arr](s string, f func([]byte) T) string {
	bytes := f([]byte(s))
	return fmt.Sprintf("%x", bytes)
}
