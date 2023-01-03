function romanToInt(s: string): number {
  const romanNumToIntMap: { [key: string]: number } = {
    I: 1,
    IV: 4,
    V: 5,
    IX: 9,
    X: 10,
    XL: 40,
    L: 50,
    XC: 90,
    C: 100,
    CD: 400,
    D: 500,
    CM: 900,
    M: 1000,
  };

  let currentIndex = 0;
  let num = 0;

  while (currentIndex < s.length) {
    if (currentIndex === s.length - 1) {
      num += romanNumToIntMap[s[currentIndex]];
      break;
    }

    const letters = s.substring(currentIndex, currentIndex + 2);
    if (romanNumToIntMap[letters]) {
      num += romanNumToIntMap[letters];
      currentIndex += 2;
      continue;
    }

    num += romanNumToIntMap[s[currentIndex]];
    currentIndex++;
  }

  return num;
}

function main() {
  console.log(romanToInt("III"));
  console.log(romanToInt("LVIII"));
  console.log(romanToInt("MCMXCIV"));
}

main();
