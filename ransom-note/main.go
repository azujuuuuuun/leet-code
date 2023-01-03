package main

func canConstruct(ransomNote string, magazine string) bool {
	ransomNoteCounter := make(map[string]int)
	for _, c := range ransomNote {
		ransomNoteCounter[string(c)]++
	}
	magazineCounter := make(map[string]int)
	for _, c := range magazine {
		magazineCounter[string(c)]++
	}

	ans := true
	for k, v := range ransomNoteCounter {
		mv, ok := magazineCounter[k]
		if !ok {
			ans = false
			break
		}
		if v > mv {
			ans = false
			break
		}
	}

	return ans
}
