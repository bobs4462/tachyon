# behaves weirdly sometimes, doesn't send the body
import time

import requests

url = "https://edu.stankin.ru/"

def render(i):
    payload = "{\n\t\t\t\"front_link\": \"ponominalu.ru\",\n\t\t\t\"name\": \"Петров Иван Николаевич\",\n\t\t\t\"subevent_full_title\": \"Park-live 2020\",\n\t\t\t\"image_wide_clean\": \"be419aa5c08a0b752a84c3b5725a0d0049cabce1.jpg\",\n\t\t\t\"age\": 18,\n\t\t\t\"date\": \"01.01.2020\",\n\t\t\t\"venue_title\": \"Москонцерт Холл\",\n\t\t\t\"subevent_type\": \"open\",\n\t\t\t\"unsubscribe_link\": \"/unsubscribe\",\n\t\t\t\"cs\": {\n\t\t\t\t\"phone\": \"+77777777777777\",\n\t\t\t\t\"email\": \"johndoe@test.com\",\n\t\t\t\t\"name\": \"johndoe\"\n\t\t\t},\n\t\t\t\"rows\": []\n\t\t}"
    headers = {
        'content-type': "application/json",
        'authorization': "Token 42e61df3-5c13-49d9-9962-64635a13adf8"
        }
    try:
        response = requests.request("GET", url, data=payload.encode(), headers=headers)
    except:
        print(i)

t0 = time.time()
for i in range(0,10000):
    render(i)

t1 = time.time()

print(t1-t0)
