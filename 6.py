"""
   Find the size of the largest area around a coordinate, where he is closes to, that isn't infinite.
"""

"""
    Start by reading the file containing the data.
    The grid has the 0, 0 in the top left corner.

    Points closer to a coordinate are considered to be part of the area around that coordinate.
    Points that are at the same distance from 2 or more coordinates will not be included in either.
"""

def manhattan(point, coordinate):
    x, y = point
    c_x, c_y = coordinate
    return abs(c_x - x) + abs(c_y - y)

with open("6.txt", 'r') as file:
    coordinates = []
    for coordinate_str in file:
        x,y = [int(s) for s in coordinate_str.split(",")]
        coordinates.append((x,y))

    bound_max_x = max(coordinates, key=lambda c: c[0])[0]
    bound_max_y = max(coordinates, key=lambda c: c[1])[1]

    space = dict()
    for x in range(bound_max_x):
        for y in range(bound_max_y):
            point = (x, y)

            closer_coordinate = None
            closer_distance = None
            for c in coordinates:
                coordinate_distance_to_point = manhattan(point, closer_coordinate)
                if not closer_coordinate or closer_distance < coordinate_distance_to_point:
                    closer_coordinate = c
                    closer_distance = coordinate_distance_to_point
                elif closer_distance == coordinate_distance_to_point:
                    # no coordinate can count with this point
                    # realize that a tie occured

            



    
    # map all the space?
    # if coordinate is in edges -> infinite space?
