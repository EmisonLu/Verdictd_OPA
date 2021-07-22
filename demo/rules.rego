package demo

MRENCLAVE = "73284f63a6d8796f"
MRSIGNER = "c4219b312ce36827"

default allow = false

allow = true {
	MRENCLAVE == input.MRENCLAVE
	MRSIGNER == input.MRSIGNER
}
