wrk.method = "POST"

-- wrk -t6 -c600 -d60s -s create_session.lua http://localhost:8080/api/v1/sessions