curl -v --request POST \
  --url http://localhost:30080/api/auth \
  --header 'content-type: application/json' \
  --data '{"email": "sabahtalateh@gmail.com", "password": "123456"}'

curl -i --request POST \
  --url http://localhost:30080/api/auth \
  --header 'content-type: application/json' \
  --data '{"email": "trot@crot.snot", "password": "zozon"}'

curl -i --request GET \
  --url http://localhost:30080/api/auth \
  --header 'content-type: application/json' \
  --cookie auth=3w/+ZlFKjNcxPcMR9SZrfuO0GfTbcodJjxk+1XW8qGrz1uVbS15K/VKGNINQzRKY3Edf0UjN

curl -i --request DELETE \
  --url http://localhost:30080/api/auth \
  --header 'content-type: application/json' \
  --cookie auth=3w/+ZlFKjNcxPcMR9SZrfuO0GfTbcodJjxk+1XW8qGrz1uVbS15K/VKGNINQzRKY3Edf0UjN
