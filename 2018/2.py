"""
    Compute the checksum of several box ID's to make sure all of them are accounted for.
"""

"""
    Start by reading the file containing the Box Ids.
    For each box ID see if it has the same letter repeated twice or trice. 
    If multiple occurences happen only account for them once. Ex. aabbcc just count 1 for twice.
"""

from collections import defaultdict

with open("2.txt", 'r') as file:
    c2 = 0
    c3 = 0

    for box_id in file:
        dict = defaultdict(int)
        for id_symbol in box_id:
            dict[id_symbol] += 1

        # Check for letters repeated twice
        repeated_twice = False
        repeated_trice = False
        for count in dict.values():
            if count == 2 and (not repeated_twice):
                repeated_twice = True
                c2 += 1
            elif count == 3 and (not repeated_trice):
                repeated_trice = True
                c3 += 1
            # Nothing left to do
            if repeated_twice and repeated_trice:
                break
        
    checksum = c2 * c3
    print(f"Checksum for the list is: {checksum}")