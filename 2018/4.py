"""
   Find the guard that spends most time sleeping and what minute he is most likely to be asleep.
"""

"""
    Start by reading the file containing the data.
    Data contains at what time the change happend and can have 3 types of information.
    [1518-04-18 00:00] Guard #1933 begins shift
    [1518-04-18 00:53] falls asleep
    [1518-04-18 00:57] wakes up

    The information about waking up and falling asleep refers to the most recent guard in post.
    
"""

from collections import defaultdict
from datetime import datetime, timedelta
import re

def get_date_from_event(event_string):
    date = re.search(r"\[(.*)\]", event_string).group(1)
    return datetime.strptime(date, "%Y-%m-%d %H:%M")

with open("4.txt", 'r') as file:
    # First we need to sort the events
    events_in_order = sorted(file, key=get_date_from_event)

    #.readlines() will keep the \n at the end of each line whereas .read().split("\n") removes them.
    # We could have done file.read().split('\n')

    guards_sleep_time = defaultdict(int)
    guards_sleep_schedule = defaultdict(lambda: defaultdict(int))
    current_guard = None
    for event in events_in_order:
        event_date = get_date_from_event(event)
        event_info = re.search(r"\[.*\] (.*$)", event).group(1)
        
        # New guard is on duty
        if event_info.startswith("Guard "):
            current_guard = re.search(r"#(\d+)", event_info).group(1)
        elif event_info == "falls asleep":
            start_sleep = event_date        
        elif event_info == "wakes up":
            stop_sleep = event_date

            for min_sleeping in range(start_sleep.minute, stop_sleep.minute):
                guards_sleep_time[current_guard] += 1
                guards_sleep_schedule[current_guard][min_sleeping] += 1

    # Get receives the key and returns the value
    guard_slept_most = max(guards_sleep_time, key=guards_sleep_time.get) 
    most_slept_min = max(guards_sleep_schedule[guard_slept_most], key=guards_sleep_schedule[guard_slept_most].get) 

    print(f"ID of the guard that slept the most: #{guard_slept_most}")
    print(f"Minute guard spends asleep the most: {most_slept_min}")
    product = int(guard_slept_most) * int(most_slept_min)
    print(f"Product = {product}")

    freq_sleep_count = 0
    freq_guard_id, freq_most_slept_min = "#0000", "00"
    # Get most slept minute for each guard
    for guard_id in guards_sleep_schedule.keys():
        most_slept_min = max(guards_sleep_schedule[guard_id], key=guards_sleep_schedule[guard_id].get) 
        slept_count = guards_sleep_schedule[guard_id][most_slept_min]
        
        # We have a new record!
        if slept_count > freq_sleep_count:
            freq_sleep_count = slept_count
            freq_guard_id, freq_most_slept_min = guard_id, most_slept_min
    
    print(f"ID of the guard that slept most frequent in the same minute: #{freq_guard_id}")
    print(f"Minute guard spends asleep the most: {freq_most_slept_min} x{freq_sleep_count}")
    product = int(freq_guard_id) * int(freq_most_slept_min)
    print(f"Product = {product}")
