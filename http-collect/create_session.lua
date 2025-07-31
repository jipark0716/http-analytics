wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"
wrk.body = '{"client_id": 1,"device_id": "d8b69cd5-0bfe-4be6-8f0f-a5060a1e9085"}'

-- wrk -t6 -c600 -d60s -s create_session.lua http://localhost:8080/api/v1/sessions