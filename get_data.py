#!/usr/bin/env python
import sys
import requests
from datetime import datetime
from pytz import timezone
from time import sleep
from pathlib import Path

# Eric Wastl, the author of Advent-of-Code, asked that if you're automatically
# querying adventofcode.com (as this script does), then you should include
# contact details in the User-Agent field. So please replace my name with yours:
NAME = "Aurélien Geron"

def usage():
    print("Usage:")
    print(f"{sys.argv[0]} {{year}} {{day}}")
    sys.exit(1)

def session_error():
    print("""Please open your browser, login to adventofcode.com, lookup 
the session cookie, and save its value to the .session file.
Here's how to find this cookie in Chrome: right-click > Inspect, select the
Application tab in the inspector, then in the left menu select
Storage > Cookies > https://adventofcode.com, and click on session in the list.
Copy the cookie value: it's a long hexadecimal .""")
    sys.exit(2)

if len(sys.argv) != 3:
    usage()

try:
    year, day = map(int, sys.argv[1:3])
except ValueError:
    usage()

session_path = Path(".session")
if not session_path.is_file():
    session_error()

eastern = timezone('US/Eastern')
requested_day = eastern.localize(datetime(year, 12, day))

while True:
    now = datetime.now(eastern)
    seconds_left = (requested_day - now).total_seconds()
    if seconds_left > 0:
        wait_time = seconds_left % 1
        if wait_time < 0.5:
            wait_time += 1
        sleep(wait_time)
        print(f"⏰ {int(seconds_left)} ", end="\r");
    else:
        break

sleep(1) # just to be safe

url = f"https://adventofcode.com/{year}/day/{day}/input"
cookies = {"session": open(".session").read().strip()}
headers = {"User-Agent": NAME}
request = requests.get(url, cookies=cookies, headers=headers)
if request.status_code != 200:
    print(f"HTTP Error {request.status_code}")
    print(request.text)
    session_error()

text = request.text

if year == 2022:
    with open(f"data/day{day:02}.txt", "w") as f:
        f.write(text)
else:
    print(text)
