package main

func isValid(s string) bool {
	var stack []string
	valid := true
	for i := 0; i < len(s); i++ {
		c := s[i : i+1]

		if c == string('(') || c == string('{') || c == string('[') {
			stack = append(stack, c)
			continue
		}

		if len(stack) == 0 {
			valid = false
			break
		}

		o := stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		if (c == string(')') && o != string('(')) || (c == string('}') && o != string('{')) || (c == string(']') && o != string('[')) {
			valid = false
			break
		}
	}

	if len(stack) != 0 {
		return false
	}

	return valid
}
