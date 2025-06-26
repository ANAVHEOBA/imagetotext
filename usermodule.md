a@a:~/imagetotext$ curl -X POST http://localhost:8080/api/auth/public/register   -H "Content-Type: application/json"   -d '{
    "email": "wisdomabraham92@gmail.com",
    "password": "your-password-here",
    "full_name": "Wisdom Abraham",
    "account_type": "Individual"
  }'
{"user":{"uuid":"46cf82e2-83a0-4844-8030-3a9fb4b84d89","email":"wisdomabraham92@gmail.com","full_name":"Wisdom Abraham","account_type":"Individual","is_verified":false,"conversion_count":0,"plan":"Free","created_at":"2025-06-23 20:33:10.165 +00:00:00"},"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTA3OTcyMDQsImlhdCI6MTc1MDcxMDgwNCwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.A5MZxGyKxDnt3lXL6i_zm4TAZCI-G51YgDUUumcxff0","message":"Account created successfully. Please check your email for verification code."}a@a:~/imagetotext$ 
















a@a:~/imagetotext$ curl -X POST http://localhost:8080/api/auth/public/verify/46cf82e2-83a0-4844-8030-3a9fb4b84d89 \
  -H "Content-Type: application/json" \
  -d '{
    "code": "692507"
  }'
{"message":"Email verified successfully","data":null}a@a:~/imagetotext$ 










a@a:~/imagetotext$ curl -X POST http://localhost:8080/api/auth/public/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "wisdomabraham92@gmail.com",
    "password": "your-password-here"
  }' -i
HTTP/1.1 200 OK
content-length: 566
x-refresh-token: 8f008ff2-8b30-48ad-b337-0b035e44f807
vary: Origin, Access-Control-Request-Method, Access-Control-Request-Headers
content-type: application/json
access-control-allow-credentials: true
date: Thu, 26 Jun 2025 00:17:59 GMT

{"user":{"uuid":"46cf82e2-83a0-4844-8030-3a9fb4b84d89","email":"wisdomabraham92@gmail.com","full_name":"Wisdom Abraham","account_type":"Individual","is_verified":true,"conversion_count":11,"plan":"Free","created_at":"2025-06-23 20:33:10.165 +00:00:00"},"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTA5ODM0NzksImlhdCI6MTc1MDg5NzA3OSwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.TR-rzObK_zf67nywKZcmqlP7y05EnjHVIPDSWBcMDN8","message":"Login successful"}a@a:~/imagetotext$ 









a@a:~/imagetotext$ curl -X GET "http://localhost:8080/api/auth/protected/verified/profile/46cf82e2-83a0-4844-8030-3a9fb4b84d89" -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTA5ODIxNDAsImlhdCI6MTc1MDg5NTc0MCwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.GyWzKSbOq5MNp_cl9nbqXCXzECrtOiUu8vc8fWer3YI"
{"uuid":"46cf82e2-83a0-4844-8030-3a9fb4b84d89","email":"wisdomabraham92@gmail.com","full_name":"Wisdom Abraham","account_type":"Individual","is_verified":true,"conversion_count":11,"plan":"Free","created_at":"2025-06-23 20:33:10.165 +00:00:00"}a@a:~/imagetotext$ 











a@a:~/imagetotext$ curl -X POST "http://localhost:8080/api/auth/protected/logout/46cf82e2-83a0-4844-8030-3a9fb4b84d89" -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTA5ODIxNDAsImlhdCI6MTc1MDg5NTc0MCwiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.GyWzKSbOq5MNp_cl9nbqXCXzECrtOiUu8vc8fWer3YI"
{"message":"Successfully logged out","data":null}a@a:~/imagetotext$ 












a@a:~/imagetotext$ curl -X POST http://localhost:8080/api/auth/public/refresh-token \
  -H "Content-Type: application/json" \
  -d '{
    "refresh_token": "8f008ff2-8b30-48ad-b337-0b035e44f807"
  }'
{"access_token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NmNmODJlMi04M2EwLTQ4NDQtODAzMC0zYTlmYjRiODRkODkiLCJlbWFpbCI6Indpc2RvbWFicmFoYW05MkBnbWFpbC5jb20iLCJleHAiOjE3NTA5ODM1MjMsImlhdCI6MTc1MDg5NzEyMywiYWNjb3VudF90eXBlIjoiSW5kaXZpZHVhbCJ9.mdn62DvnOmqsJLAwCl0USD6436Z7VHIQmBtbw7dT5uc","refresh_token":"2e3114d7-1c2e-40cb-a571-6f9e05122419","token_type":"Bearer","expires_in":3600}a@a:~/imagetotext$ 