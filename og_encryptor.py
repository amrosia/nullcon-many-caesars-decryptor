import string  # Import the string module to use predefined character sets like ascii_letters and digits
import re      # Import the re module to use regular expressions

# Read the contents of 'text.txt'
# Convert all characters in the text to lowercase to standardize further processing
text = open('text.txt','r').read().lower()

# Read the flag from 'flag.txt'
# .strip() removes any leading/trailing whitespace, [4:-1] slices off the first 4 characters and the last character,
# and .replace('_','+') replaces underscores with plus signs.
flag = open('flag.txt','r').read().strip()[4:-1].replace('_','+')

# Define a string 'chars' consisting of allowed characters:
# all letters (both uppercase and lowercase), all digits, plus sign, forward slash, and equal sign.
chars = string.ascii_letters + string.digits + '+/='

# Create a regular expression that matches any string composed solely of characters in 'chars'
# and that has a length between 5 and 70 characters inclusive.
regex = re.compile('[' + chars + ']{5,70}')

# Ensure that the 'flag' exactly matches the pattern defined by the regex.
assert re.fullmatch(regex, flag)

# Define the caesar function which applies a Caesar cipher-like shift to the message (msg).
def caesar(msg, shift):
    # For each character 'c' in msg:
    #   1. Find its index in 'chars'.
    #   2. Add the provided 'shift' value.
    #   3. Use modulo arithmetic to wrap around if necessary,
    #   4. Then get the resulting character from 'chars'.
    # The function returns the transformed message after shifting every character.
	return ''.join(chars[(chars.index(c) + shift) % len(chars)] for c in msg)

i = 0         # Initialize index 'i' to keep track of our current position in 'text'
count = 0     # Initialize 'count' to cycle through characters in 'flag' for shifting purposes

# Process the text character by character until the end of the text.
while i < len(text):
	# If the current character is not a lowercase letter:
	#   Print it directly and move to the next character.
	if text[i] not in string.ascii_lowercase:
		print(text[i], end = '')
		i += 1
	else:
		# When encountering a lowercase letter, determine the contiguous substring of lowercase letters.
		j = i
		while text[j] in string.ascii_lowercase:
			j += 1
		# Determine the shift for the caesar cipher:
		#   Use the value (index) of the current flag character in 'chars', where the flag character is chosen
		#   cyclically (using count % len(flag)).
		# Apply the caesar() function on the identified substring.
		print(caesar(text[i:j], chars.index(flag[count % len(flag)])), end = '')
		count += 1  # Increment count to use the next flag character for the next substring.
		i = j       # Set the index 'i' to 'j' to continue processing after the current substring.