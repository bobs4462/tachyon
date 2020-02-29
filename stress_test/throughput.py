import requests
import time

import json

def render():
    data = {"front_link": "ponominalu.ru",
            "name": "Петров Иван Николаевич",
            "subevent_full_title": "Park-live 2020",
            "image_wide_clean": "be419aa5c08a0b752a84c3b5725a0d0049cabce1.jpg",
            "age": 18,
            "date": "01.01.2020",
            "venue_title": "Москонцерт Холл",
            "subevent_type": "open",
            "unsubscribe_link": "/unsubscribe",
            "cs": {
                "phone": "+77777777777777",
                "email": "johndoe@test.com",
                "name": "johndoe"
                },
            "rows": []}
    headers = {'user-agent': 'my-app/0.0.1'}
    try:
        res = requests.post('http://127.0.0.1:5001/render/abandoned.html', data=json.dumps(data), headers=headers)
        f = open('test.html', 'w')
        f.write(res.text)
        f.close()
    except Exception:
        print(json.dumps(data))
    

t0 = time.time()

for i in range(0,100000):
    render()

t1 = time.time()

print(t1-t0)
