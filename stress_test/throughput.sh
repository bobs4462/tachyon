curl --request POST \
  --url 'http://35.228.134.188:5001/render/abandoned.html' \
  --header 'authorization: Token 42e61df3-5c13-49d9-9962-64635a13adf8' \
  --header 'content-type: application/json' \
  --data '{
			"front_link": "ponominalu.ru",
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
			"rows": []
		}'
