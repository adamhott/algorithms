// Leetcode 124. Valid Palindrome

//A phrase is a palindrome if, after converting 
//all uppercase letters into lowercase letters and 
//removing all non-alphanumeric characters, it reads 
//the same forward and backward. Alphanumeric characters include letters and numbers.

//Given a string s, return true if it is a palindrome, or false otherwise.


// Function to check if a character is alphanumeric by checking ASCII ranges
fn is_alphanumeric(c: char) -> bool {
    let ascii = c as u8; // Get the ASCII value of the character
    // Check if it's a letter (uppercase or lowercase) or a digit
    (ascii >= b'A' && ascii <= b'Z') || // Uppercase letters (A-Z)
    (ascii >= b'a' && ascii <= b'z') || // Lowercase letters (a-z)
    (ascii >= b'0' && ascii <= b'9')    // Digits (0-9)
}

// Function to convert a character to lowercase manually
fn to_lowercase(c: char) -> char {
    let ascii = c as u8; // Get the ASCII value of the character
    if ascii >= b'A' && ascii <= b'Z' {
        // Convert uppercase to lowercase
        (ascii + 32) as char // ASCII difference between 'A' and 'a' is 32
    } else {
        c // If it's not uppercase, return the character unchanged
    }
}

// Function to check if a string is a palindrome
fn is_palindrome(s: &str) -> bool {
    // Create a filtered version of the input string, keeping only alphanumeric characters
    let filtered: String = s.chars()
        .filter(|&c| is_alphanumeric(c)) // Keep only alphanumeric characters
        .map(|c| to_lowercase(c)) // Convert to lowercase manually
        .collect(); // Collect into a string

    if filtered.is_empty() {
        return true; // An empty string or no alphanumeric characters means it's a palindrome
    }

    // Define two pointers, one at the beginning and one at the end
    let mut left = 0; // Initialize the left pointer
    let mut right = filtered.len() - 1; // Initialize the right pointer, only if filtered is not empty

    // Compare characters from the left and right pointers
    while left < right {
        if filtered.chars().nth(left) != filtered.chars().nth(right) {
            return false; // If characters don't match, it's not a palindrome
        }
        left += 1; // Increment the left pointer
        right -= 1; // Decrement the right pointer
    }

    true // If all characters match, it's a palindrome
}


fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test case for example 1
    fn test_example_1() {
        let input = "A man, a plan, a canal: Panama"; // Input for the test case
        let expected = true; // Expected output
        assert_eq!(is_palindrome(input), expected); // Assertion to check if the function output matches the expected output
    }

    #[test]
    // Test case for example 2
    fn test_example_2() {
        let input = "race a car"; // Input for the test case
        let expected = false; // Expected output
        assert_eq!(is_palindrome(input), expected); // Assertion to check if the function output matches the expected output
    }

    #[test]
    // Test case for example 3
    fn test_example_3() {
        let input = " "; // Input for the test case
        let expected = true; // Expected output
        assert_eq!(is_palindrome(input), expected); // Assertion to check if the function output matches the expected output
    }
}

