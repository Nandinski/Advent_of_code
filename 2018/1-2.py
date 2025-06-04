"""
    Starting with a frequency of zero, find the first repeated frequency.
"""

"""
    Start by reading the file containing the changes in frequency.
    Loop if necessary:
        Find where the frequency repeats for the first time.
"""

def main():
    # Start with the 0 frequency (Starting point)
    current_frequency = 0
    # Create set for frequencies already seen
    frequencies_observed = {current_frequency}

    while True:
        with open("1.txt", 'r') as file:
            frequency_changes = (int(frequency_change) for frequency_change in file)
            for frequency_change in frequency_changes:
                current_frequency += frequency_change

                if current_frequency in frequencies_observed:
                    print(f"This frequency was the first to be repeated twice: {current_frequency}")
                    return # Seems better than multiple breaks
                else:
                    frequencies_observed.add(current_frequency)

    print("No duplicate was found")

main()
