import java.util.HashMap;

public class Main {
    public static void main(String[] args) {
        System.out.println(Solution.divideArray(new int[]{3, 2, 3, 2, 2, 2}));
        System.out.println(Solution.divideArray(new int[]{1, 2, 3, 4}));
    }

    class Solution {
        public static boolean divideArray(int[] nums) {
            var map = new HashMap<Integer, Integer>();

            for (var n: nums) {
                var count = map.get(n);
                if (count == null) {
                    map.put(n, 1);
                } else {
                    map.put(n, count + 1);
                }
            }

            var canDivide = true;
            for (var c: map.values()) {
                if (c % 2 == 1) {
                    canDivide = false;
                }
            }

            return canDivide;
        }
    }
}