"""
   Find how many units of polymer remain after fully reacting.
"""

"""
    Start by reading the file containing the data.
    There are several types of units one for each letter in the alphabet.
    Each unit type has 2 polarities. Ex a and A 
    If the polarities are adjacent they will react and leave nothing behind.
    abBC -> aC

    There can be chain reactions after a reaction takes place. For example:
    caBbADD -> caADD -> cDD
"""
import re

def react_polymer(polymer):
    stack = []
    for c in polymer:
        if stack:
            last_c = stack[-1]
            # Check if they have the same type
            # Check for different polarities
            #if c.lower() == last_c.lower() and c != last_c:
            if c.swapcase() == last_c:
                # React!
                stack.pop()
                continue
        stack.append(c)
    return stack

with open("5.txt", 'r') as file:
    polymer = file.readline().strip()

    # Remove a unit type and see which one produces the shortest polymer
    unit_type_set = set(polymer.lower())
    smallest_polymer_length = len(polymer)
    for unit_type in unit_type_set:
        # remove unit type from polymer
        reg_exp = f"[{unit_type + unit_type.upper()}]"
        polymer_wo_type = re.sub(reg_exp, "", polymer)

        polymer_reacted = react_polymer(polymer_wo_type)

        smallest_polymer_length = min(smallest_polymer_length, len(polymer_reacted))
    
    print(f"Len of smallest polymer: {smallest_polymer_length}")
