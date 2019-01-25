"""
    Find strings that differ by 1 char
"""

"""
    Start by reading the file containing the Box Ids.
    Compare each string against every other string
    Check how many char match
"""

with open("2.txt", 'r') as file:
    seen_box_ids_set = set()
    for box_id in file:
        for seen_box_id in seen_box_ids_set:
            differing_ids = 0
            for i in range(len(seen_box_id)):
                if seen_box_id[i] != box_id[i]:
                    differing_ids += 1
                # No point in continuing
                if differing_ids > 1:
                    break
            # If ids only differ by one character 
            # We found the correct match!
            if differing_ids == 1:
                match = []
                for i in range(len(seen_box_id)):
                    if seen_box_id[i] == box_id[i]:
                        match.append(seen_box_id[i])
                print(''.join(match))
        # Save id 
        seen_box_ids_set.add(box_id)