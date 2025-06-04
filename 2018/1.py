"""
    Starting with a frequency of zero, what is the resulting frequency after all of the changes in frequency have been applied?
"""

"""
    Start by reading the file containing the changes in frequency.
    Add them up and print the result.
"""

with open("1.txt", 'r') as file:
    frequency_changes = (int(change_freq) for change_freq in file)

    resulting_frequency = sum(frequency_changes)

    print(f"Resulting Frequency: {resulting_frequency}")
