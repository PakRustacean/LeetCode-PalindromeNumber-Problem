struct Solution;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let s = x.to_string();
        let rev_s: String = s.chars().rev().collect();

        s == rev_s
    }
}

fn main() {
    let result = Solution::is_palindrome(121);
    println!("Is 121 a palindrome? {}", result);

    let result = Solution::is_palindrome(-121);
    println!("Is -121 a palindrome? {}", result);

    let result = Solution::is_palindrome(10);
    println!("Is 10 a palindrome? {}", result);
}
