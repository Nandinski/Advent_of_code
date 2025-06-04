"""
   Find how much fabric would be used by multiple people.
"""

"""
    Start by reading the file containing the claims.
    Claim is of the form #1 @ 55,885: 22x10.
    Where the number is the claim number the first point is the top left corner of a rectangle and the 
    second value is the size of the rectangle.
"""

from collections import defaultdict
import re

with open("3.txt", 'r') as file:
    used_fabric = defaultdict(list)
    claim_in_conflict = defaultdict(list)
    for claim in file:
        claim_id, left, top, width, height = map(int, re.findall(r"\d+", claim))

        claim_in_conflict[claim_id] = set()
        # Go through all of the squares and mark them as being used
        for x in range(left, left + width):
            for y in range(top, top + height):

                # Check if someone else is already using this fabric                
                if used_fabric[(x, y)]:
                    for overlapping_claim_id in used_fabric[(x, y)]:
                        claim_in_conflict[overlapping_claim_id].add(claim_id)
                        claim_in_conflict[claim_id].add(overlapping_claim_id)
                
                used_fabric[(x, y)].append(claim_id)

    multiple_use_fabric_area = len([c for c in used_fabric if len(used_fabric[c]) > 1])
    print(f"Multiple use of fabric area: {multiple_use_fabric_area}")

    claim_without_conflict = [c for c in claim_in_conflict if len(claim_in_conflict[c]) == 0][0]
    print(f"Claim without conflict = {claim_without_conflict}")